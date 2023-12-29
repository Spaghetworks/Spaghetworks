use godot::engine::RefCounted;
use godot::prelude::*;
use nalgebra::base::dimension::Const;
use nalgebra::base::dimension::Dyn;
use nalgebra::base::Matrix;
use nalgebra::base::OMatrix;
use nalgebra::linalg::SVD;

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct MatrixSolver {
    #[base]
    base: Base<RefCounted>,

    size: i64,
    svd: SVD<f64, Dyn, Dyn>,
}

#[godot_api]
impl MatrixSolver {
    #[func]
    pub fn create(_size: i64, rows: PackedFloat64Array) -> Gd<MatrixSolver> {
        let matrix_size = _size as usize;
        assert_eq!(matrix_size * matrix_size, rows.len());

        let rows_vec = rows.to_vec();
        let row_iter = rows_vec.into_iter();
        let matrix =
            OMatrix::from_row_iterator_generic(Dyn(matrix_size), Dyn(matrix_size), row_iter);
        let _svd = SVD::new_unordered(matrix, true, true);
        Gd::from_init_fn(|base| MatrixSolver {
            base,
            size: _size,
            svd: _svd,
        })
    }

    #[func]
    pub fn solve(&self, b_vec: PackedFloat64Array) -> PackedFloat64Array {
        assert_eq!(self.size, (b_vec.len() as i64));
        let epsilon: f64 = 1e-9;
        //let b_vec_iter = b_vec.to_vec().into_iter();
        //let b_matrix = Matrix::from_row_iterator(size.try_into().unwrap(), 1, b_vec_iter);
        let b_matrix = Matrix::<f64, Dyn, Const<1>, _>::from_vec(b_vec.to_vec());
        let x_vec = self.svd.solve(&b_matrix, epsilon).unwrap();
        //let x_iter = x_vec.data.as_vec();
        PackedFloat64Array::from(x_vec.as_slice())
        //PackedFloat64Array::from_iter(x_iter)
    }
}

/*
#[godot_api]
impl IRefCounted for MatrixSolver {
    fn init(base: Base<RefCounted>) -> Self {
        MatrixSolver{
            size:0,
            base,
            svd,
        }
    }
}
// */
