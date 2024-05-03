use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

mod matrixsolver;
pub use matrixsolver::MatrixSolver;

mod alternating_current;
pub use alternating_current::{AcBridge, AcComponent, AcNode, AcSystem};

mod math;
mod util;
