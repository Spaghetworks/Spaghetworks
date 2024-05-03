use nalgebra::Complex;

use super::{AcConnection, AcVertex};
use crate::util::TypedInstanceId;

#[derive(Copy, Clone, Debug)]
pub enum VertexType {
    Node,
    Resistor(ResistorData),
    VoltageSource(VoltageSourceData),
    OtherComponent,
}

#[derive(Clone, Copy, Debug)]
pub struct ResistorData {
    pub resistance: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct VoltageSourceData {
    pub voltage: Complex<f64>,
}

pub enum AcMeasurable {
    Node(TypedInstanceId<AcVertex>),
    Connection(AcConnection),
}
