use std::ops::{Add, Mul};

use nalgebra::{Complex, Dyn, MatrixViewMut, U1};
use num_traits::Zero;

use super::{AcConnection, AcVertex, RealType};
use crate::util::TypedInstanceId;

#[derive(Copy, Clone, Debug)]
pub enum VertexType {
    Node,
    Resistor(ResistorData),
    VoltageSource(VoltageSourceData),
    OtherComponent,
}

#[derive(Clone, Copy, Debug)]
pub struct ResistorData {
    pub resistance: RealType,
}

#[derive(Clone, Copy, Debug)]
pub struct VoltageSourceData {
    pub voltage: Complex<RealType>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AcMeasurable {
    Node(TypedInstanceId<AcVertex>),
    Connection(AcConnection),
    Constant,
}

pub type AcMultiplier = Complex<RealType>;

#[derive(Debug)]
pub struct ConstraintElement {
    measurable: AcMeasurable,
    multiplier: AcMultiplier,
}

impl Mul<AcMultiplier> for AcMeasurable {
    type Output = ConstraintElement;
    fn mul(self, rhs: AcMultiplier) -> Self::Output {
        ConstraintElement {
            measurable: self,
            multiplier: rhs,
        }
    }
}

impl Add<ConstraintElement> for Constraint {
    type Output = Constraint;
    fn add(mut self, rhs: ConstraintElement) -> Self::Output {
        self.elements.push(rhs);
        self
    }
}

impl Add<Constraint> for Constraint {
    type Output = Constraint;
    fn add(mut self, mut rhs: Constraint) -> Self::Output {
        self.elements.append(&mut rhs.elements);
        self
    }
}

#[derive(Debug, Default)]
pub struct Constraint {
    elements: Vec<ConstraintElement>,
}

pub type MeasurableMapping = fn(AcMeasurable) -> usize;
impl Constraint {
    pub fn compile_to(
        &mut self,
        mapping: MeasurableMapping,
        out_row_view: &mut MatrixViewMut<Complex<RealType>, U1, Dyn>,
    ) {
        let (_, column_count) = out_row_view.shape();

        out_row_view.fill(Complex::zero());

        if self.elements.is_empty() {
            return;
        }

        for element in self.elements.iter() {
            let column = mapping(element.measurable);

            out_row_view[(0, column)] = element.multiplier;
        }
    }
}
