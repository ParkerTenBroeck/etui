use std::collections::{HashMap, HashSet};

use super::id::{hash, Id, TypeId};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct IdCollision;

#[derive(Default, Debug)]
pub struct Memory {
    map: HashMap<u64, Box<dyn std::any::Any>>,
    seen: HashSet<u64>,
}

impl Memory {
    pub fn get_or<T: Clone + 'static>(&mut self, id: Id, default: T) -> Result<T, IdCollision> {
        self.get_or_create(id, || default)
    }

    pub fn get_or_create<T: Clone + 'static>(
        &mut self,
        id: Id,
        default: impl FnOnce() -> T,
    ) -> Result<T, IdCollision> {
        let hash = hash(TypeId::of::<T>(), id);
        self.seen(hash)?;
        self.map.entry(hash).or_insert_with(|| Box::new(default()));
        if let Some(item) = self.map.get_mut(&hash) {
            if let Some(item) = item.downcast_mut::<T>() {
                Ok(item.clone())
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    pub fn get<T: Clone + 'static>(&mut self, id: Id) -> Result<Option<T>, IdCollision> {
        let hash = hash(TypeId::of::<T>(), id);
        self.seen(hash)?;
        if let Some(item) = self.map.get_mut(&hash) {
            if let Some(item) = item.downcast_mut::<T>() {
                Ok(Some(item.clone()))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub fn insert<T: 'static>(&mut self, id: Id, default: T) {
        let hash = hash(TypeId::of::<T>(), id);
        let _ = self.seen(hash);
        self.map.insert(hash, Box::new(default));
    }

    fn seen(&mut self, value: u64) -> Result<(), IdCollision> {
        if self.seen.contains(&value) {
            Err(IdCollision)
        } else {
            self.seen.insert(value);
            Ok(())
        }
    }

    pub fn clear_seen(&mut self) {
        self.seen.clear();
    }
}

#[test]
pub fn test() {
    use crate::math_util::VecI2;
    let mut test = Memory::default();
    test.insert(Id::new("Bruh"), 12i32);
    test.insert(Id::new("Nice"), VecI2::new(420, 69));

    _ = test.get_or(Id::new("NOOOO"), Box::new(666i32));

    println!("{:#?}", test.get::<i32>(Id::new("Bruh")));
    println!("{:#?}", test.get::<VecI2>(Id::new("Nice")));
    println!("{:#?}", test.get::<Box<i32>>(Id::new("NOOOO")));
}
