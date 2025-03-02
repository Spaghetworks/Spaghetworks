use godot::prelude::*;

use crate::physics_units::Density;

#[derive(GodotClass)]
#[class(base=Resource)]
#[non_exhaustive]
struct FluidPrototype {
    base: Base<Resource>,

    density: Density,
}
