use crate::physics_units::{Area, Length, Volume};

pub(crate) trait TankShape {
    /// It is a logic error for this to change if the tank shape does not change
    fn get_full_volume(&self) -> Volume;

    /// It is a logic error for this to change if the tank shape does not change
    fn get_full_depth(&self) -> Length;

    /// It is a logic error for this to be not monotonically increasing
    /// It is a logic error for this to change for a given volume if the tank shape does not change
    /// It is a logic error for this to be inconsistant with get_volume_from_depth
    fn get_depth_from_volume(&self, volume: Volume) -> Option<Length>;

    /// It is a logic error for this to be not monotonically increasing
    /// It is a logic error for this to change for a given depth if the tank shape does not change
    /// It is a logic error for this to be inconsistant with get_depth_from_volume
    fn get_volume_from_depth(&self, depth: Length) -> Option<Volume>;
}

pub(crate) struct PrismTankShape {
    base_area: Area,
    depth: Length,
}
impl TankShape for PrismTankShape {
    fn get_full_depth(&self) -> Length {
        self.depth
    }
    fn get_full_volume(&self) -> Volume {
        self.base_area * self.depth
    }
    fn get_depth_from_volume(&self, volume: Volume) -> Option<Length> {
        if Volume::from(0.0) <= volume && volume <= self.get_full_volume() {
            Some(volume / self.base_area)
        } else {
            None
        }
    }
    fn get_volume_from_depth(&self, depth: Length) -> Option<Volume> {
        if Length::from(0.0) <= depth && depth <= self.depth {
            Some(depth * self.base_area)
        } else {
            None
        }
    }
}
pub(crate) struct StackedTankShape {
    // A list of tank shapes, from bottom to top
    stack: Vec<Box<dyn TankShape>>,
}

impl TankShape for StackedTankShape {
    fn get_full_depth(&self) -> Length {
        self.stack.iter().map(|shape| shape.get_full_depth()).sum()
    }
    fn get_full_volume(&self) -> Volume {
        self.stack.iter().map(|shape| shape.get_full_volume()).sum()
    }

    fn get_depth_from_volume(&self, mut volume: Volume) -> Option<Length> {
        let mut cumulative_depth = Length::from(0.0);
        for shape in self.stack.iter() {
            if shape.get_full_volume() > volume {
                return shape
                    .get_depth_from_volume(volume)
                    .map(|depth| depth + cumulative_depth);
            } else {
                volume -= shape.get_full_volume();
                cumulative_depth += shape.get_full_depth();
            }
        }
        None
    }

    fn get_volume_from_depth(&self, mut depth: Length) -> Option<Volume> {
        let mut cumulative_volume = Volume::from(0.0);
        for shape in self.stack.iter() {
            if shape.get_full_depth() > depth {
                return shape
                    .get_volume_from_depth(depth)
                    .map(|volume| volume + cumulative_volume);
            } else {
                depth -= shape.get_full_depth();
                cumulative_volume += shape.get_full_volume();
            }
        }
        None
    }
}
