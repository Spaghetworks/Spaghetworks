use godot::prelude::*;

use crate::physics_units::{Density, Mass, Volume};

#[derive(GodotClass)]
#[class(base=Resource)]
#[non_exhaustive]
pub struct FluidPrototype {
    base: Base<Resource>,

    pub(crate) density: Density,
}

#[derive(Clone)]
pub(crate) struct Fluid {
    pub(crate) prototype: Gd<FluidPrototype>,
    pub(crate) volume: Volume,
}

impl Fluid {
    pub(crate) fn get_mass(&self) -> Mass {
        self.prototype.bind().density * self.volume
    }

    // Err(()) if the volume is more than the current volume
    pub(crate) fn split_off(&mut self, volume: Volume) -> Result<Fluid, ()> {
        if self.volume < volume {
            self.volume -= volume;
            Ok(Fluid {
                prototype: self.prototype.clone(),
                volume: volume,
            })
        } else {
            Err(())
        }
    }
}
