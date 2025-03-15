use super::tank_shape::TankShape;
use crate::{
    fluid::fluid_prototype::Fluid,
    physics_units::{Length, Volume},
};
// use std::marker::PhantomData;

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

pub(crate) struct SimpleFluidTank {
    downlinks: Vec<SimpleFluidTankIdentifier>,
    shape: Box<dyn TankShape>, // TODO: Switch to using &'owner dyn TankShape
    elevation: Length,
    fluids: Vec<Fluid>, // _phantom: PhantomData<&'owner dyn TankShape>,
} // TODO
pub(crate) struct FluidTank {
    simple_fluid_tanks: Vec<SimpleFluidTank>,
    // shapes: Bump // Container that owns all the &dyn TankShape objects
} // TODO

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

    pub(crate) fn finalize(self) -> Result<FluidTank, Self> {
        for x in &self.simple_fluid_tanks {
            if x.elevation.unknown_id != 0 {
                return Err(self);
            }
        }
        let tanks: Vec<_> = self
            .simple_fluid_tanks
            .into_iter()
            .map(|simple_fluid_tank_builder| SimpleFluidTank {
                downlinks: simple_fluid_tank_builder.downlinks,
                elevation: simple_fluid_tank_builder.elevation.height,
                shape: simple_fluid_tank_builder.shape,
                fluids: Vec::new(),
            })
            .collect();
        Ok(FluidTank {
            simple_fluid_tanks: tanks,
        })
    }
}

impl FluidTank {
    pub(crate) fn add_fluid_to(&mut self, fluid: Fluid, tank: SimpleFluidTankIdentifier) {
        todo!()
    }
}

impl SimpleFluidTank {
    fn add_fluid(&mut self, fluid: Fluid) {
        let mut index = None;
        // Merge fluid if it already exists
        for (idx, own_fluid) in self.fluids.iter_mut().enumerate() {
            if own_fluid.prototype == fluid.prototype {
                own_fluid.volume += fluid.volume;
                return;
            } else if own_fluid.prototype.bind().density < fluid.prototype.bind().density {
                index = Some(idx);
                break;
            }
        }
        // Insertion sort, but like only one iteration
        let index = index.unwrap_or(self.fluids.len());
        self.fluids.insert(index, fluid);
    }
    fn get_fill_volume(&self) -> Volume {
        let volume = self.fluids.iter().map(|fluid| fluid.volume).sum();

        volume
    }
    /// Err indicates that the tank is over-full
    fn get_fill_height(&self) -> Option<Length> {
        self.shape.get_depth_from_volume(self.get_fill_volume())
    }

    fn is_overflowing(&self) -> bool {
        self.get_fill_volume() > self.shape.get_full_volume()
    }

    fn pop_overflowing_fluid(&mut self) -> Option<Fluid> {
        let overflow_volume = self.get_fill_volume() - self.shape.get_full_volume();

        if overflow_volume > Volume::from(0.0) {
            if {
                let top_fluid = self
                    .fluids
                    .last()
                    .expect("Overflowing, so there must be at least one fluid");
                top_fluid.volume > overflow_volume
            } {
                self.fluids
                    .last_mut()
                    .expect("We checked earlier")
                    .split_off(overflow_volume)
                    .ok()
            } else {
                self.fluids.pop()
            }
        } else {
            None
        }
    }
}
