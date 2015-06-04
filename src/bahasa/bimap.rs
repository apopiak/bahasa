use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
use std::clone::Clone;

pub struct Bimap<T, S> {
    key_value: HashMap<T, S>,
    value_key: HashMap<S, T>,
}

impl<T: , S> Bimap<T, S>
        where T: Eq + Hash, S: Eq + Hash
{
    pub fn new () -> Bimap<T, S> {
        Bimap { key_value: HashMap::new(), value_key: HashMap::new() }
    }

    pub fn insert(&mut self, first: T, second: S)
            where T: Clone, S: Clone
    {
        self.key_value.insert(first.clone(), second.clone());
        self.value_key.insert(second, first);
    }

    pub fn get_value(&self, key: &T) -> Option<&S> {
        self.key_value.get(key)
    }

    pub fn get_key(&self, key: &S) -> Option<&T> {
        self.value_key.get(key)
    }
}

impl<T> Bimap<T, T>
        where T: Eq + Hash
{
    pub fn get(&self, key: &T) -> Option<&T> {
        if self.key_value.get(key).is_some() {
            return self.key_value.get(key);
        } else if self.value_key.get(key).is_some() {
            return self.value_key.get(key);
        } else {
            return None;
        }
    }
}
