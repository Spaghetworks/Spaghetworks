use godot::prelude::*;

struct SpaghetworksExtension;

#[gdextension]
unsafe impl ExtensionLibrary for SpaghetworksExtension {}

mod matrixsolver;
pub use matrixsolver::MatrixSolver;

mod alternating_current;
pub use alternating_current::{AcBridge, AcSystem, AcVertex};

mod math;
mod util;
