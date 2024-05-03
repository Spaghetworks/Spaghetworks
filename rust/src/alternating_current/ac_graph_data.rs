use std::ops::{Add, Mul};

use nalgebra::{Complex, Matrix2};

use super::{AcConnection, AcVertex, RealType};
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
    pub resistance: RealType,
}

#[derive(Clone, Copy, Debug)]
pub struct VoltageSourceData {
    pub voltage: Complex<RealType>,
}

#[derive(Clone, Copy, Debug)]
pub enum AcMeasurable {
    Node(TypedInstanceId<AcVertex>),
    Connection(AcConnection),
    Constant(Complex<RealType>),
}

#[derive(Clone, Copy, Debug)]
pub enum AcMultiplier {
    Scalar(RealType),
    ScaledDerivative(RealType),
    ScaledIntegral(RealType),
    ScaleRotate(Complex<RealType>),
    General(Matrix2<RealType>),
}

#[derive(Debug)]
pub struct ConstraintElement {
    measurable: AcMeasurable,
    multiplier: AcMultiplier,
}

impl Mul<AcMultiplier> for AcMeasurable {
    type Output = ConstraintElement;
    fn mul(self, rhs: AcMultiplier) -> Self::Output {
        ConstraintElement {
            measurable: self,
            multiplier: rhs,
        }
    }
}

impl Add<ConstraintElement> for ConstraintList {
    type Output = ConstraintList;
    fn add(mut self, rhs: ConstraintElement) -> Self::Output {
        self.elements.push(rhs);
        self
    }
}

impl From<ConstraintElement> for Matrix2<RealType> {
    fn from(value: ConstraintElement) -> Self {
        todo!()
    }
}

impl Add<ConstraintList> for ConstraintList {
    type Output = ConstraintList;
    fn add(mut self, mut rhs: ConstraintList) -> Self::Output {
        self.elements.append(&mut rhs.elements);
        self
    }
}

#[derive(Debug, Default)]
pub struct ConstraintList {
    elements: Vec<ConstraintElement>,
}
