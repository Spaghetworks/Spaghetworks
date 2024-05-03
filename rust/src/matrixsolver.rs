use godot::engine::RefCounted;
use godot::prelude::*;
use nalgebra::base::dimension::Const;
use nalgebra::base::dimension::Dyn;
use nalgebra::base::Matrix;
use nalgebra::base::OMatrix;
use nalgebra::linalg::SVD;

#[derive(GodotClass)]
#[class(no_init, base=RefCounted)]
pub struct MatrixSolver {
    base: Base<RefCounted>,

    size: i64,
    decomp: SVD<f64, Dyn, Dyn>,
    matrix: OMatrix<f64, Dyn, Dyn>,
}

#[godot_api]
impl MatrixSolver {
    #[func]
    pub fn create(_size: i64, rows: PackedFloat64Array) -> Gd<MatrixSolver> {
        let matrix_size = _size as usize;
        assert_eq!(matrix_size * matrix_size, rows.len());

        let rows_vec = rows.to_vec();
        let row_iter = rows_vec.into_iter();
        let _matrix =
            OMatrix::from_row_iterator_generic(Dyn(matrix_size), Dyn(matrix_size), row_iter);
        let _decomp = SVD::new_unordered(_matrix.clone(), true, true);
        Gd::from_init_fn(|base| MatrixSolver {
            base,
            size: _size,
            decomp: _decomp,
            matrix: _matrix,
        })
    }

    #[func]
    pub fn solve(&self, b_vec: PackedFloat64Array) -> PackedFloat64Array {
        assert_eq!(self.size, (b_vec.len() as i64));
        let b_matrix = Matrix::<f64, Dyn, Const<1>, _>::from_vec(b_vec.to_vec());
        let epsilon: f64 = 1e-9;
        let x_vec = self.decomp.solve(&b_matrix, epsilon).unwrap();
        PackedFloat64Array::from(x_vec.as_slice())
    }

    #[func]
    pub fn solve_elided(
        &self,
        b_vec: PackedFloat64Array,
        elided: PackedInt64Array,
    ) -> PackedFloat64Array {
        assert_eq!(self.size, (b_vec.len() as i64));
        let elided_vec: Vec<usize> = elided
            .to_vec()
            .into_iter()
            .map(|element| element as usize)
            .collect();
        let elided_slice: &[usize] = elided_vec.as_slice().into();
        let b_matrix = Matrix::<f64, Dyn, Const<1>, _>::from_vec(b_vec.to_vec());
        let b_matrix_elided = b_matrix.remove_rows_at(elided_slice);
        let a_matrix_elided = self
            .matrix
            .clone()
            .remove_rows_at(elided_slice)
            .remove_columns_at(elided_slice);
        let decomp_elided = SVD::new_unordered(a_matrix_elided.clone(), true, true);
        let epsilon: f64 = 1e-9;
        let x_vec_elided = decomp_elided.solve(&b_matrix_elided, epsilon).unwrap();

        //Put the zeros back
        let mut x_vec: Vec<f64> = Vec::with_capacity(self.size as usize);
        let mut j: usize = 0;
        for i in 0..(self.size) {
            if elided_vec.contains(&(i as usize)) {
                x_vec.push(0.0);
            } else {
                x_vec.push(x_vec_elided[j]);
                j += 1;
            }
        }
        PackedFloat64Array::from(x_vec.as_slice())
    }
}
