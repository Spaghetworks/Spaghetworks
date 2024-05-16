pub trait Validatable {
    fn is_valid(&self) -> bool;
}

impl<T: Validatable> Validatable for Vec<T> {
    fn is_valid(&self) -> bool {
        self.iter().all(T::is_valid)
    }
}
