use std::{hash::Hash, marker::PhantomData};

use godot::{
    builtin::meta::ConvertError,
    obj::{Gd, GodotClass, InstanceId},
};

use super::Validatable;

#[derive(Debug)]
pub struct TypedInstanceId<T>
where
    T: GodotClass,
{
    instance_id: InstanceId,
    _phantom: PhantomData<T>,
}
impl<T: GodotClass> Copy for TypedInstanceId<T> {}
impl<T: GodotClass> Clone for TypedInstanceId<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: GodotClass> Eq for TypedInstanceId<T> {}
impl<T: GodotClass> PartialEq for TypedInstanceId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.instance_id == other.instance_id
    }
}
impl<T: GodotClass> Ord for TypedInstanceId<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.instance_id.cmp(&other.instance_id)
    }
}
impl<T: GodotClass> PartialOrd for TypedInstanceId<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: GodotClass> Hash for TypedInstanceId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.instance_id.hash(state)
    }
}
impl<T: GodotClass> From<Gd<T>> for TypedInstanceId<T> {
    fn from(value: Gd<T>) -> Self {
        TypedInstanceId {
            instance_id: value.instance_id(),
            _phantom: PhantomData,
        }
    }
}
impl<T: GodotClass> TryFrom<InstanceId> for TypedInstanceId<T> {
    type Error = ConvertError;
    /// Checks the engine for validity. The object may still drop afterwards, invalidating the TypedInstanceId.
    fn try_from(value: InstanceId) -> Result<Self, Self::Error> {
        Ok(Gd::<T>::try_from_instance_id(value)?.into())
    }
}
impl<T: GodotClass> TryFrom<TypedInstanceId<T>> for Gd<T> {
    type Error = godot::builtin::meta::ConvertError;
    fn try_from(value: TypedInstanceId<T>) -> Result<Self, Self::Error> {
        Gd::try_from_instance_id(value.instance_id)
    }
}

impl<T: GodotClass> Validatable for TypedInstanceId<T> {
    fn is_valid(&self) -> bool {
        Gd::<T>::try_from_instance_id(self.instance_id).is_ok()
    }
}
