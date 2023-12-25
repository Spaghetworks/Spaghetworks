use godot::prelude::*;
use godot::engine::RefCounted;
//use godot::engine::IRefCounted;

use nalgebra::base::Matrix;
use nalgebra::base::OMatrix;
use nalgebra::base::OVector;
use nalgebra::base::dimension::Const;
use nalgebra::base::dimension::Dyn;
use nalgebra::linalg::SVD;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=RefCounted)]
struct MatrixSolver {
    #[base]
    base: Base<RefCounted>,

    size: i64,
    svd: SVD<f64, Dyn, Dyn>,
}

#[godot_api]
impl MatrixSolver {
    #[func]
    pub fn create(_size: i64, rows: PackedFloat64Array) -> Gd<MatrixSolver> {
        assert!(_size * _size == (rows.len() as i64));
        let rows_vec :Vec<f64>= rows.to_vec();
        let row_iter = rows_vec.into_iter();
        let matrix = OMatrix::from_row_iterator_generic(Dyn(_size.try_into().unwrap()), Dyn(_size.try_into().unwrap()), row_iter);
        let _svd = SVD::new_unordered(matrix, true, true);
        Gd::from_init_fn(|base| {MatrixSolver{ base, size: _size, svd: _svd}})
    }

    #[func]
    pub fn solve(&self, b_vec: PackedFloat64Array) -> PackedFloat64Array {
        assert!(self.size == (b_vec.len() as i64));
        let eps :f64 = 1e-9;
        //let b_vec_iter = b_vec.to_vec().into_iter();
        //let b_matrix = Matrix::from_row_iterator(size.try_into().unwrap(), 1, b_vec_iter);
        let b_matrix = Matrix::<f64,Dyn,Const<1>,_>::from_vec(b_vec.to_vec());
        let x_vec = self.svd.solve(&b_matrix, eps).unwrap();
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
