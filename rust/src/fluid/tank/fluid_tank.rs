use super::tank_shape::TankShape;
use crate::physics_units::Length;

#[derive(Clone, Copy, PartialEq)]
struct UnknownFluidTankElevation {
    unknown_id: usize,
    height: Length,
}

#[derive(Clone, Copy, PartialEq)]
struct UnknownFluidTankElevationSolution {
    from_id: usize,
    to_id: usize,
    difference: Length,
}

pub(crate) struct FluidTankBuilder {
    simple_fluid_tanks: Vec<SimpleFluidTankBuilder>,
    next_unused_unknown_id: usize,
}
pub(crate) struct SimpleFluidTankBuilder {
    downlinks: Vec<SimpleFluidTankIdentifier>,
    shape: Box<dyn TankShape>,
    elevation: UnknownFluidTankElevation,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct SimpleFluidTankIdentifier(usize);

pub(crate) struct SimpleFluidTank;
pub(crate) struct FluidTank;

impl SimpleFluidTankIdentifier {
    fn increment(&mut self) {
        self.0 += 1;
    }
}

impl UnknownFluidTankElevation {
    fn solve(
        &self,
        other: UnknownFluidTankElevation,
    ) -> Result<UnknownFluidTankElevationSolution, ()> {
        if self.unknown_id == other.unknown_id && self.height != other.height {
            Err(())
        } else {
            let (from, to) = if self.unknown_id < other.unknown_id {
                (&other, self)
            } else {
                (self, &other)
            };
            Ok(UnknownFluidTankElevationSolution {
                from_id: from.unknown_id,
                to_id: to.unknown_id,
                difference: to.height - from.height,
            })
        }
    }

    fn apply_solution(&mut self, solution: UnknownFluidTankElevationSolution) {
        if self.unknown_id == solution.from_id {
            self.unknown_id = solution.to_id;
            self.height += solution.difference;
        }
    }
}

impl std::ops::AddAssign<Length> for UnknownFluidTankElevation {
    fn add_assign(&mut self, rhs: Length) {
        self.height += rhs;
    }
}

impl std::ops::Add<Length> for UnknownFluidTankElevation {
    type Output = Self;
    fn add(mut self, rhs: Length) -> Self::Output {
        self += rhs;
        self
    }
}

impl std::ops::SubAssign<Length> for UnknownFluidTankElevation {
    fn sub_assign(&mut self, rhs: Length) {
        self.height -= rhs;
    }
}

impl std::ops::Sub<Length> for UnknownFluidTankElevation {
    type Output = Self;
    fn sub(mut self, rhs: Length) -> Self::Output {
        self += rhs;
        self
    }
}

// TODO: Below is old code to be inspected for useful stuff then removed

pub(crate) enum FluidTankFinalizeError {
    NotOrdered,
    UninitializedSimpleFluidTank,
}

#[derive(PartialEq, Eq, Hash)]
enum DfsBookkeepingState {
    Unvisited,
    Pending,
    Finalized,
}

impl FluidTankBuilder {
    pub(crate) fn new() -> Self {
        Self {
            simple_fluid_tanks: Vec::new(),
            next_unused_unknown_id: 0,
        }
    }
    fn make_simple_tank(&mut self, shape: Box<dyn TankShape>) -> SimpleFluidTankBuilder {
        let tank = SimpleFluidTankBuilder {
            downlinks: Vec::new(),
            shape,
            elevation: UnknownFluidTankElevation {
                unknown_id: self.next_unused_unknown_id,
                height: 0.0.into(),
            },
        };
        self.next_unused_unknown_id += 1;

        tank
    }
    pub(crate) fn create_at_bottom(
        &mut self,
        shape: Box<dyn TankShape>,
    ) -> SimpleFluidTankIdentifier {
        let tank = self.make_simple_tank(shape);
        self.simple_fluid_tanks.push(tank);

        SimpleFluidTankIdentifier(self.simple_fluid_tanks.len() - 1)
    }
    pub(crate) fn create_above(
        &mut self,
        target: SimpleFluidTankIdentifier,
        shape: Box<dyn TankShape>,
    ) -> SimpleFluidTankIdentifier {
        let tank = self.make_simple_tank(shape);

        self.simple_fluid_tanks.push(tank);

        let id = SimpleFluidTankIdentifier(self.simple_fluid_tanks.len() - 1);
        self.add_link(id, target);
        id
    }
    pub(crate) fn add_link(
        &mut self,
        above: SimpleFluidTankIdentifier,
        below: SimpleFluidTankIdentifier,
    ) -> Result<(), ()> {
        if above == below {
            return Err(());
        }
        // Solve for how elevations change
        let elevation_linkage_solution = {
            let above_tank: &SimpleFluidTankBuilder = &self.simple_fluid_tanks[above.0];
            let below_tank: &SimpleFluidTankBuilder = &self.simple_fluid_tanks[below.0];

            // Verify
            above_tank.elevation.solve(below_tank.elevation)?
        };

        // Change elevations
        self.simple_fluid_tanks
            .iter_mut()
            .for_each(|tank| tank.elevation.apply_solution(elevation_linkage_solution));

        // Add the link
        let above_tank = &mut self.simple_fluid_tanks[above.0];
        above_tank.downlinks.push(below);

        Ok(())
    }

    pub(crate) fn finalize(self) -> FluidTank {
        todo!()
    }
}
