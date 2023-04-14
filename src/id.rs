use std::hash::Hasher;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(u64);

impl Id {
    pub fn value(&self) -> u64 {
        self.0
    }

    pub fn new(source: impl std::hash::Hash) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        source.hash(&mut hasher);
        Self(hasher.finish())
    }
}

pub fn hash(type_id: TypeId, id: Id) -> u64 {
    type_id.value() ^ id.value()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct TypeId(u64);

impl TypeId {
    #[inline]
    pub fn of<T: std::any::Any + 'static>() -> Self {
        Self(Id::new(std::any::TypeId::of::<T>()).value())
    }

    #[inline(always)]
    pub(crate) fn value(&self) -> u64 {
        self.0
    }
}
