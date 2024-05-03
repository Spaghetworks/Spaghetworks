use std::ops::{Add, Mul};

use nalgebra::{Complex, DMatrixViewMut, Matrix2};

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

impl Add<AcMultiplier> for AcMultiplier {
    type Output = AcMultiplier;
    fn add(self, rhs: AcMultiplier) -> Self::Output {
        match (self, rhs) {
            (AcMultiplier::Zero, _) => rhs,
            (_, AcMultiplier::Zero) => self,
            (AcMultiplier::Unit, AcMultiplier::Unit) => AcMultiplier::Scalar(2 as RealType),
            (AcMultiplier::Unit, AcMultiplier::Scalar(rhs_real)) => {
                AcMultiplier::Scalar(1 as RealType + rhs_real)
            }
            (AcMultiplier::Scalar(lhs_real), AcMultiplier::Unit) => {
                AcMultiplier::Scalar(lhs_real + 1 as RealType)
            }
            (AcMultiplier::Scalar(lhs_real), AcMultiplier::Scalar(rhs_real)) => {
                AcMultiplier::Scalar(lhs_real + rhs_real)
            }
            (
                AcMultiplier::ScaledDerivative(lhs_real),
                AcMultiplier::ScaledDerivative(rhs_real),
            ) => AcMultiplier::ScaledDerivative(lhs_real + rhs_real),
            (AcMultiplier::ScaledDerivative(lhs_real), AcMultiplier::ScaledIntegral(rhs_real)) => {
                AcMultiplier::ScaledDerivative(lhs_real - rhs_real)
            }
            (AcMultiplier::ScaledIntegral(lhs_real), AcMultiplier::ScaledDerivative(rhs_real)) => {
                AcMultiplier::ScaledIntegral(lhs_real - rhs_real)
            }
            (AcMultiplier::ScaledIntegral(lhs_real), AcMultiplier::ScaledIntegral(rhs_real)) => {
                AcMultiplier::ScaledIntegral(lhs_real + rhs_real)
            }
            (AcMultiplier::ScaleRotate(lhs_complex), AcMultiplier::ScaleRotate(rhs_complex)) => {
                AcMultiplier::ScaleRotate(lhs_complex + rhs_complex)
            }
            (lhs, rhs) => AcMultiplier::General(Matrix2::from(lhs) + Matrix2::from(rhs)),
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
pub type MeasurableMapping = fn(AcMeasurable) -> usize;
impl ConstraintList {
    pub fn compile_to(
        &mut self,
        mapping: MeasurableMapping,
        out_view: &mut DMatrixViewMut<RealType>,
    ) {
        let (row_count, column_count) = out_view.shape();
        assert!(row_count == 2);

        out_view.fill(0 as RealType);

        if self.elements.is_empty() {
            return;
        }

        for element in self.elements.iter() {
            let column = mapping(element.measurable);
            assert!(column % 2 == 0);
            assert!(column + 1 < column_count);
            let mut section_view = out_view.view_range_mut(0..1, column..(column + 1));
            section_view += Matrix2::from(element.multiplier);
        }
    }
}
