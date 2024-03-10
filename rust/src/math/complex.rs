use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// Numeric = Add + Copy + Mul + PartialEq + Neg + AddAssign + MulAssign + Sub + SubAssign
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex<T: Add + Copy + Mul + PartialEq + Neg + AddAssign + MulAssign + Sub + SubAssign>
{
    pub re: T,
    pub im: T,
}

pub type Complex64 = Complex<f64>;

impl<T> AddAssign for Complex<T>
where
    T: Add + Copy + Mul + PartialEq + Neg + AddAssign + MulAssign + Sub + SubAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T> Add for Complex<T>
where
    T: Add + Copy + Mul + PartialEq + Neg + AddAssign + MulAssign + Sub + SubAssign,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result += rhs;
        result
    }
}

impl<T> MulAssign for Complex<T>
where
    T: Add<T, Output = T>
        + Copy
        + Mul<T, Output = T>
        + PartialEq
        + Neg
        + AddAssign
        + MulAssign
        + Sub<T, Output = T>
        + SubAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        (self.re, self.im) = (
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}

impl<T> Mul for Complex<T>
where
    T: Add<T, Output = T>
        + Copy
        + Mul<T, Output = T>
        + PartialEq
        + Neg
        + AddAssign
        + MulAssign
        + Sub<T, Output = T>
        + SubAssign,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result *= rhs;
        result
    }
}

impl<T> MulAssign<T> for Complex<T>
where
    T: Add<T, Output = T>
        + Copy
        + Mul<T, Output = T>
        + PartialEq
        + Neg
        + AddAssign
        + MulAssign
        + Sub<T, Output = T>
        + SubAssign,
{
    fn mul_assign(&mut self, rhs: T) {
        self.re *= rhs;
        self.im *= rhs;
    }
}

impl<T> Mul<T> for Complex<T>
where
    T: Add<T, Output = T>
        + Copy
        + Mul<T, Output = T>
        + PartialEq
        + Neg
        + AddAssign
        + MulAssign
        + Sub<T, Output = T>
        + SubAssign,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let mut result = self;
        result *= rhs;
        result
    }
}

impl<T> Neg for Complex<T>
where
    T: Add + Copy + Mul + PartialEq + Neg<Output = T> + AddAssign + MulAssign + Sub + SubAssign,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Complex::<T> {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl<T> SubAssign for Complex<T>
where
    T: Add + Copy + Mul + PartialEq + Neg + AddAssign + MulAssign + Sub + SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.re -= rhs.re;
        self.im -= rhs.im;
    }
}

impl<T> Sub for Complex<T>
where
    T: Add + Copy + Mul + PartialEq + Neg + AddAssign + MulAssign + Sub + SubAssign,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result -= rhs;
        result
    }
}
