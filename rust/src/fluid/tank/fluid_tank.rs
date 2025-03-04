use super::tank_shape::TankShape;

#[derive(Clone, Copy, PartialEq)]
struct UnknownFluidTankElevation {
    unknown_id: usize,
    height: f64,
}

pub(crate) struct FluidTankBuilder {
    simple_fluid_tanks: Vec<SimpleFluidTankBuilder>,
}
pub(crate) struct SimpleFluidTankBuilder {
    downlinks: Vec<SimpleFluidTankIdentifier>,
    volume: Option<Box<dyn TankShape>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct SimpleFluidTankIdentifier(usize);

enum FluidTankFinalizeError {
    NotOrdered,
    UninitializedSimpleFluidTank,
}

pub(crate) struct FluidTank;
pub(crate) struct SimpleFluidTank;

impl SimpleFluidTankIdentifier {
    fn increment(&mut self) {
        self.0 += 1;
    }
}
