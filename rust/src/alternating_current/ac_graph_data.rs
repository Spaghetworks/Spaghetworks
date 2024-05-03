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
    Zero,
    Unit,
    Scalar(RealType),
    ScaledDerivative(RealType),
    ScaledIntegral(RealType),
    ScaleRotate(Complex<RealType>),
    General(Matrix2<RealType>),
}

impl From<AcMultiplier> for Matrix2<RealType> {
    fn from(value: AcMultiplier) -> Self {
        match value {
            AcMultiplier::Zero => Matrix2::zeros(),
            AcMultiplier::Unit => Matrix2::identity(),
            AcMultiplier::Scalar(scalar) => Matrix2::new_scaling(scalar),
            AcMultiplier::ScaledDerivative(scalar) => {
                Matrix2::new(0 as RealType, scalar, -scalar, 0 as RealType)
            }
            AcMultiplier::ScaledIntegral(scalar) => {
                Matrix2::new(0 as RealType, -scalar, scalar, 0 as RealType)
            }
            AcMultiplier::ScaleRotate(complex) => {
                Matrix2::new(complex.re, complex.im, -complex.im, complex.re)
            }
            AcMultiplier::General(matrix) => matrix,
        }
    }
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
