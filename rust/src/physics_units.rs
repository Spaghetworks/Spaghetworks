macro_rules! wrapper_struct {
    ($newtype:ident($contained:ty)) => {
        wrapper_struct!(pub(self) $newtype($contained))
    };
    ($visibility:vis $newtype:ident($contained:ty)) => {
        #[derive(Clone, Copy, PartialOrd, PartialEq)]
        $visibility struct $newtype($contained);
        impl From<$contained> for $newtype {
            fn from(value: $contained) -> Self {
                Self(value)
            }
        }
        impl From<$newtype> for $contained {
            fn from(value: $newtype) -> Self {
                value.0
            }
        }
    };
    ($newtype:ident($contained:ty); $rest:tt) => {
        wrapper_struct!($newtype($contained));
        wrapper_struct!($rest);
    };
    ($visibility:vis $newtype:ident($contained:ty); $rest:tt) => {
        wrapper_struct!($visibility $newtype($contained));
        wrapper_struct!($rest);
    };
}

macro_rules! wrapper_addition_group {
    ($type:ty) => {
        impl std::ops::AddAssign for $type {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }
        impl std::ops::Add for $type {
            type Output = Self;
            fn add(mut self, rhs: Self) -> Self::Output {
                self += rhs;
                self
            }
        }
        impl std::ops::Neg for $type {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self(-self.0)
            }
        }
        impl std::ops::SubAssign for $type {
            fn sub_assign(&mut self, rhs: Self) {
                <Self as std::ops::AddAssign>::add_assign(self, -rhs);
            }
        }
        impl std::ops::Sub for $type {
            type Output = Self;
            fn sub(mut self, rhs: Self) -> Self::Output {
                self -= rhs;
                self
            }
        }
    };
    ($type:ty, $($rest:ty),+) => {
        wrapper_addition_group!($type);
        wrapper_addition_group!($($rest),+);
    }
}

macro_rules! wrapper_vector_space {
    ([$type:ty], $scalar:ty) => {
        impl std::ops::MulAssign<$scalar> for $type {
            fn mul_assign(&mut self, rhs: $scalar) {
                self.0 *= rhs;
            }
        }
        impl std::ops::Mul<$scalar> for $type {
            type Output = Self;
            fn mul(mut self, rhs: $scalar) -> Self::Output {
                self *= rhs;
                self
            }
        }
        impl std::ops::DivAssign<$scalar> for $type {
            fn div_assign(&mut self, rhs: $scalar) {
                self.0 /= rhs;
            }
        }
        impl std::ops::Div<$scalar> for $type {
            type Output = Self;
            fn div(mut self, rhs: $scalar) -> Self::Output {
                self /= rhs;
                self
            }
        }
    };
    ([$type:ty, $($rest:ty),+], $scalar:ty) => {
        wrapper_vector_space!([$type], $scalar);
        wrapper_vector_space!([$($rest),+], $scalar);
    }
}

macro_rules! wrapper_squares {
    (Sq($a:ty) -> $b:ty) => {
        // Define a * a => b
        impl std::ops::Mul<$a> for $a {
            type Output = $b;
            fn mul(self, rhs: $a) -> Self::Output {
                Self::Output::from(self.0 * rhs.0)
            }
        }

        // Define b / a => a
        impl std::ops::Div<$a> for $b {
            type Output = $a;
            fn div(self, rhs: $a) -> Self::Output {
                Self::Output::from(self.0 / rhs.0)
            }
        }
    };
}
macro_rules! wrapper_multiplies {
    (Mul($a:ty, $b:ty) -> $c:ty) => {
        // Define a * b = c
        impl std::ops::Mul<$b> for $a {
            type Output = $c;
            fn mul(self, rhs: $b) -> Self::Output {
                <Self::Output as core::convert::From<_>>::from(self.0 * rhs.0)
            }
        }
        // Define b * a = c
        impl std::ops::Mul<$a> for $b {
            type Output = $c;
            fn mul(self, rhs: $a) -> Self::Output {
                <Self::Output as core::convert::From<_>>::from(self.0 * rhs.0)
            }
        }
        // Define c / a = b
        impl std::ops::Div<$a> for $c {
            type Output = $b;
            fn div(self, rhs: $a) -> Self::Output {
                <Self::Output as core::convert::From<_>>::from(self.0 / rhs.0)
            }
        }
        // Define c / b = a
        impl std::ops::Div<$b> for $c {
            type Output = $a;
            fn div(self, rhs: $b) -> Self::Output {
                <Self::Output as core::convert::From<_>>::from(self.0 / rhs.0)
            }
        }
    };
}

macro_rules! wrapper_dsl {
    {} => {};
    {define_wrapper ($vis:vis $wrapper:ident($wrapped:ty)); $($rest:tt)*} => {
        wrapper_struct!($vis $wrapper($wrapped));
        wrapper_dsl!{$($rest)*}
    };
    {define_wrapper ($wrapper:ident($wrapped:ty)); $($rest:tt)*} => {
        wrapper_struct!(pub(self) $wrapper($wrapped));
        wrapper_dsl!{$($rest)*}
    };
    {addition_group($($ty:ty),+); $($rest:tt)*} => {
        wrapper_addition_group!($($ty),+);
        wrapper_dsl!{$($rest)*}
    };
    {vector_space([$($vectors:ty),+], $scalar:ty); $($rest:tt)*} => {
        wrapper_vector_space!([$($vectors),+], $scalar);
        wrapper_dsl!{$($rest)*}
    };
    {Sq($ty:ty) -> $result:ty; $($rest:tt)*} => {
        wrapper_squares!(Sq($ty) -> $result);
        wrapper_dsl!{$($rest)*}
    };
    {Mul($a:ty, $b:ty) -> $c:ty; $($rest:tt)*} => {
        wrapper_multiplies!(Mul($a, $b) -> $c);
        wrapper_dsl!{$($rest)*}
    }
}

wrapper_dsl! {
    define_wrapper(pub(crate) Length(f64));
    define_wrapper(pub(crate) Area(f64));
    define_wrapper(pub(crate) Volume(f64));
    define_wrapper(pub(crate) Mass(f64));
    define_wrapper(pub(crate) Density(f64));

    addition_group(Length, Area, Volume, Mass, Density);
    vector_space([Length, Area, Volume, Mass, Density], f64);

    Sq(Length) -> Area;
    Mul(Length, Area) -> Volume;
    Mul(Density, Volume) -> Mass;
}
