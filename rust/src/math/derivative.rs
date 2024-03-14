pub trait Differentiable {
    type Output;
    fn derivative(&self) -> Self::Output;
}
