use std::{cell::RefCell, collections::BinaryHeap, num::NonZeroUsize};

use nalgebra::{Complex, DMatrix, DVector, Matrix, MatrixN, U1};
use num_traits::{One, Zero};

use super::RealType;

#[derive(Debug, Clone, Copy)]
enum OldOrUpToDate<T> {
    Old(T),
    UpToDate(T),
}

impl<T> OldOrUpToDate<T> {
    fn into_old(self) -> Self {
        Self::Old(self.into_inner())
    }
    fn into_up_to_date(self) -> Self {
        Self::UpToDate(self.into_inner())
    }

    fn is_old(&self) -> bool {
        if let Self::Old(_) = self {
            true
        } else {
            false
        }
    }

    fn is_up_to_date(&self) -> bool {
        if let Self::UpToDate(_) = self {
            true
        } else {
            false
        }
    }

    fn into_inner(self) -> T {
        match self {
            Self::Old(x) => x,
            Self::UpToDate(x) => x,
        }
    }
}

#[derive(Debug)]
pub(crate) struct AcCalculator<T: Clone> {
    pub(super) vertices: VertexCollection<T>,
    constraints: ConstraintCollection,
    matrix: RefCell<OldOrUpToDate<DMatrix<Complex<RealType>>>>,
    vertex_results: RefCell<Option<Vec<Complex<RealType>>>>,
}
impl<T: Clone> AcCalculator<T> {
    pub(crate) fn new() -> Self {
        Self {
            vertices: VertexCollection::new(),
            constraints: ConstraintCollection::new(),
            matrix: RefCell::new(OldOrUpToDate::UpToDate(DMatrix::zeros(0, 0))),
            vertex_results: RefCell::new(None),
        }
    }

    pub(crate) fn print_matrix(&self) {
        println!("Matrix:\n{}", self.matrix.borrow().clone().into_inner());
    }
    pub(crate) fn register_vertex(&mut self, payload: T) -> AcVertexIdentifier {
        let associated_id = self.vertices.add_vertex(payload);
        // *(self.vertex_results.get_mut()) = None;
        associated_id
    }

    fn register_constraint(&mut self, constraint: Constraint) -> ConstraintIdentifier {
        // Invalidate cache
        self.invalidate_constraint_cache();

        for constraint_element in constraint.elements.iter() {
            if let VertexOrUnit::Vertex(vertex_id) = constraint_element.variable {
                let _ = self.vertices.get_or_create_vertex_result_index(vertex_id);
            }
        }
        let constraint_identifier = self.constraints.add(constraint);
        constraint_identifier
    }

    pub(crate) fn get_constraint_builder(&mut self) -> ConstraintBuilder<T> {
        ConstraintBuilder::new(self)
    }

    fn invalidate_constraint_cache(&mut self) {
        *(self.vertex_results.get_mut()) = None;
        let matrix = self
            .matrix
            .replace(OldOrUpToDate::Old(DMatrix::zeros(0, 0)))
            .into_inner();
        self.matrix.replace(OldOrUpToDate::Old(matrix));
    }

    fn compile_constraints_into(&mut self, output_matrix: &mut DMatrix<Complex<RealType>>) {
        output_matrix.resize_mut(
            self.vertices.get_row_count(),
            self.constraints.constraint_list.len() + 1,
            Complex::zero(),
        );
        output_matrix.fill(Complex::zero());
        output_matrix[(0, 0)] = Complex::one();
        for (row_index_minus_one, constraint) in
            self.constraints.constraint_list.iter_mut().enumerate()
        {
            for constraint_element in constraint.elements.iter_mut() {
                let column_index: usize =
                    if let VertexOrUnit::Vertex(vertex_identifier) = constraint_element.variable {
                        self.vertices
                            .get_or_create_vertex_result_index(vertex_identifier)
                            .expect("Constraint should only hold valid vertex ids")
                    } else {
                        0
                    };
                output_matrix[(row_index_minus_one + 1, column_index)] =
                    constraint_element.multiplier;
            }
        }
    }

    fn get_constraints_matrix(
        &mut self,
    ) -> std::cell::Ref<
        '_,
        OldOrUpToDate<
            nalgebra::Matrix<
                Complex<f64>,
                nalgebra::Dyn,
                nalgebra::Dyn,
                nalgebra::VecStorage<Complex<f64>, nalgebra::Dyn, nalgebra::Dyn>,
            >,
        >,
    > {
        if self.matrix.borrow().is_old() {
            let mut matrix = self
                .matrix
                .replace(OldOrUpToDate::Old(DMatrix::zeros(0, 0)))
                .into_inner();

            self.compile_constraints_into(&mut matrix);
            self.matrix.replace(OldOrUpToDate::UpToDate(matrix));
        }
        self.matrix.borrow()
    }

    fn refresh_results_cache(&mut self) {
        if self.matrix.borrow().is_old() {
            // The cache isn't valid; refresh the cache
            let matrix = self.get_constraints_matrix().clone().into_inner();
            let mut constant = DVector::zeros(matrix.nrows());
            constant[0] = Complex::<RealType>::one();

            if let Some(output_vector) = nalgebra::linalg::QR::new(matrix).solve(&constant) {
                // This should be a column vector with as many elements as there are vertices plus one
                let number_of_vertices_with_constraints = self.vertices.get_row_count();
                assert_eq!(
                    output_vector.shape(),
                    (number_of_vertices_with_constraints, 1)
                );

                let output_vector_length = output_vector.len();
                // Safe because we're resizing `data` into its original size, meaning we get only a vector of initialized `Complex<RealType>`s. Thus, the transmutation is safe
                let result_vec = unsafe {
                    Box::from_raw(core::mem::transmute::<_, *mut [Complex<RealType>]>(
                        Box::into_raw(
                            output_vector
                                .data
                                .resize(output_vector_length)
                                .into_boxed_slice(),
                        ),
                    ))
                }
                .into_vec();

                *(self.vertex_results.borrow_mut()) = Some(result_vec);
            }
        }
    }

    pub(crate) fn get_vertex_result(
        &mut self,
        vertex: AcVertexIdentifier,
    ) -> Option<Complex<RealType>> {
        if let Some(index) = self.vertices.get_vertex_result_index(vertex) {
            self.refresh_results_cache();
            // Now self.vertex_results could be Some if the matrix inversion worked, or None if it did not.

            self.vertex_results
                .borrow()
                .as_ref()
                .and_then(|results| Some(results[usize::from(index)]))
        } else {
            Some(Complex::zero())
        }
    }
}

pub struct ConstraintBuilder<'a, T: Clone> {
    constraint_list: Vec<ConstraintElement>,
    ac_calculator: &'a mut AcCalculator<T>,
}
impl<'a, T: Clone> ConstraintBuilder<'a, T> {
    fn new(ac_calculator: &'a mut AcCalculator<T>) -> Self {
        Self {
            constraint_list: vec![],
            ac_calculator,
        }
    }

    pub fn set_vertex_constraint(
        mut self,
        vertex: AcVertexIdentifier,
        multiplier: Complex<RealType>,
    ) -> Self {
        for element in self.constraint_list.iter_mut() {
            if element.variable == VertexOrUnit::Vertex(vertex) {
                element.multiplier = multiplier;
                return self;
            }
        }
        let element = ConstraintElement {
            variable: VertexOrUnit::Vertex(vertex),
            multiplier,
        };
        self.constraint_list.push(element);
        self
    }

    pub fn set_constant_constraint(mut self, constant: Complex<RealType>) -> Self {
        let element = ConstraintElement {
            variable: VertexOrUnit::Unit,
            multiplier: constant,
        };
        if self.constraint_list.is_empty() {
            self.constraint_list.push(element);
        } else if self.constraint_list[0].variable != VertexOrUnit::Unit {
            let move_to_end = core::mem::replace(&mut self.constraint_list[0], element);
            self.constraint_list.push(move_to_end);
        } else {
            self.constraint_list[0] = element;
        }

        self
    }

    pub fn finalize(self) -> ConstraintIdentifier {
        self.ac_calculator.register_constraint(Constraint {
            elements: self.constraint_list.into_boxed_slice(),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum VertexOrUnit {
    Vertex(AcVertexIdentifier),
    Unit,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ConstraintElement {
    variable: VertexOrUnit,
    multiplier: Complex<RealType>,
}
#[derive(Debug)]
struct Constraint {
    elements: Box<[ConstraintElement]>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ConstraintIdentifier {}

#[derive(Debug)]
struct ConstraintCollection {
    constraint_list: Vec<Constraint>,
}
impl ConstraintCollection {
    fn new() -> Self {
        Self {
            constraint_list: vec![],
        }
    }
    fn add(&mut self, constraint: Constraint) -> ConstraintIdentifier {
        self.constraint_list.push(constraint);
        ConstraintIdentifier {}
    }
}

#[derive(Debug)]
struct VertexRegistration<T> {
    generation: AcVertexIdGeneration,
    payload: Option<T>,
    result_index: Option<NonZeroUsize>,
}

#[derive(Debug)]
pub(super) struct VertexCollection<T: Clone> {
    available_reuse_identifiers: BinaryHeap<core::cmp::Reverse<usize>>,
    registered_vertices: Vec<VertexRegistration<T>>,
    /// Maps from index - 1 to the identifier
    result_index_to_identifier: Vec<AcVertexIdentifier>,
}
impl<T: Clone> VertexCollection<T> {
    fn new() -> Self {
        Self {
            available_reuse_identifiers: BinaryHeap::new(),
            registered_vertices: vec![],
            result_index_to_identifier: vec![],
        }
    }
    fn add_vertex(&mut self, vertex: T) -> AcVertexIdentifier {
        if let Some(index) = self.available_reuse_identifiers.pop().map(|x| x.0) {
            assert!(self.registered_vertices[index].payload.is_none());
            let identifier = AcVertexIdentifier {
                index,
                generation: self.registered_vertices[index].generation.next_generation(),
            };
            let registration = VertexRegistration {
                generation: identifier.generation,
                payload: Some(vertex),
                result_index: None,
            };
            self.registered_vertices[index] = registration;
            identifier
        } else {
            let identifier = AcVertexIdentifier {
                index: self.registered_vertices.len(),
                generation: AcVertexIdGeneration(0),
            };
            let registration = VertexRegistration {
                generation: identifier.generation,
                payload: Some(vertex),
                result_index: None,
            };
            self.registered_vertices.push(registration);
            identifier
        }
    }
    fn get_vertex(&self, id: AcVertexIdentifier) -> Option<T> {
        if self.registered_vertices.len() > id.index
            && self.registered_vertices[id.index].generation == id.generation
        {
            self.registered_vertices[id.index].payload.clone()
        } else {
            None
        }
    }

    fn is_identifier_valid(&self, id: AcVertexIdentifier) -> bool {
        self.registered_vertices.len() > id.index
            && self.registered_vertices[id.index].payload.is_some()
    }

    /// DOES NOT REMOVE the association from the vertex node
    fn remove_vertex(&mut self, id: AcVertexIdentifier) {
        if self.is_identifier_valid(id) {
            self.remove_vertex_result_index(id);
            self.registered_vertices[id.index].payload = None
        }
    }

    fn get_or_create_vertex_result_index(&mut self, id: AcVertexIdentifier) -> Result<usize, ()> {
        if self.is_identifier_valid(id) {
            match self.registered_vertices[id.index].result_index {
                None => {
                    let index = self.result_index_to_identifier.len() + 1;
                    self.result_index_to_identifier.push(id);
                    // Is nonzero because index is a usize + 1
                    self.registered_vertices[id.index].result_index =
                        Some(unsafe { NonZeroUsize::new_unchecked(index) });
                    Ok(index)
                }
                Some(index) => Ok(index.into()),
            }
        } else {
            Err(())
        }
    }

    fn vertex_has_result_index(&self, id: AcVertexIdentifier) -> bool {
        self.is_identifier_valid(id) && self.registered_vertices[id.index].result_index.is_some()
    }

    pub(super) fn get_vertex_result_index(&self, id: AcVertexIdentifier) -> Option<NonZeroUsize> {
        if self.is_identifier_valid(id) {
            self.registered_vertices[id.index].result_index
        } else {
            None
        }
    }

    fn remove_vertex_result_index(&mut self, id: AcVertexIdentifier) {
        if self.is_identifier_valid(id) {
            if let Some(index) = self.registered_vertices[id.index].result_index {
                let list_index = Into::<usize>::into(index) - 1;
                let swapping_id = *self.result_index_to_identifier.last().expect(
                    "is_identifier_valid implies that result_index_to_identifier is not empty",
                );
                self.result_index_to_identifier.swap_remove(list_index);
                if swapping_id != id {
                    assert!(self.is_identifier_valid(swapping_id), "An element of result_index_to_identifier was not a valid AcVertexIdentifier");
                    assert!(self.registered_vertices[swapping_id.index]
                        .result_index
                        .is_some(), "An element of result_index_to_identifier is an AcVertexIdentifier referring to a vertex without a result_index");
                    self.registered_vertices[swapping_id.index].result_index = Some(index);
                }
            }
        }
    }

    fn get_row_count(&self) -> usize {
        self.result_index_to_identifier.len() + 1
    }
}
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct AcVertexIdGeneration(usize);
impl AcVertexIdGeneration {
    fn next_generation(&self) -> Self {
        Self(self.0.wrapping_add(1))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct AcVertexIdentifier {
    index: usize,
    generation: AcVertexIdGeneration,
}

mod test {
    #[test]
    fn a_test() {}
}
