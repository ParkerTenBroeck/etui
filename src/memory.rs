use std::collections::HashMap;

use super::id::{hash, Id, TypeId};

#[derive(Default, Debug)]
pub struct Memory {
    map: HashMap<u64, Box<dyn std::any::Any>>,
}

impl Memory {
    pub fn get_or<T: Clone + 'static>(&mut self, id: Id, default: T) -> T {
        self.get_or_create(id, || default)
    }

    pub fn get_or_create<T: Clone + 'static>(&mut self, id: Id, default: impl FnOnce() -> T) -> T {
        let hash = hash(TypeId::of::<T>(), id);
        self.map.entry(hash).or_insert_with(|| Box::new(default()));
        if let Some(item) = self.map.get_mut(&hash) {
            if let Some(item) = item.downcast_mut::<T>() {
                item.clone()
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    pub fn get<T: Clone + 'static>(&mut self, id: Id) -> Option<T> {
        let hash = hash(TypeId::of::<T>(), id);
        if let Some(item) = self.map.get_mut(&hash) {
            item.downcast_mut::<T>().map(|item| item.clone())
        } else {
            None
        }
    }

    pub fn insert<T: 'static>(&mut self, id: Id, default: T) {
        let hash = hash(TypeId::of::<T>(), id);
        self.map.insert(hash, Box::new(default));
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
