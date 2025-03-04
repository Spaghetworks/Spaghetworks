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
