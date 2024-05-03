use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

mod matrixsolver;
pub use matrixsolver::MatrixSolver;
