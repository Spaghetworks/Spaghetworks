use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

mod fluid;
mod matrixsolver;

pub use matrixsolver::MatrixSolver;

pub mod physics_units;
