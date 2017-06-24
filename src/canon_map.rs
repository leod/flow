use std::collections::{HashMap, hash_map};
use std::fmt::Debug;
use std::hash::Hash;

pub trait Canonize {
    type Canon: Eq + Hash + Debug;

    fn canonize(&self) -> Self::Canon;
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CanonMap<K: Canonize + Eq + Hash, V: Eq>(HashMap<K::Canon, V>);

impl<K: Eq + Hash + Copy + Canonize, V: Eq + Clone + Debug> CanonMap<K, V> {
    pub fn new() -> Self {
        CanonMap(HashMap::new())
    }

    pub fn set(&mut self, k: K, v: V) {
        self.0.insert(k.canonize(), v);
    }
    
    pub fn get(&self, k: K) -> Option<&V> {
        self.0.get(&k.canonize())
    }

    pub fn remove(&mut self, k: K) -> Option<V> {
        self.0.remove(&k.canonize())
    }

    pub fn iter(&self) -> hash_map::Iter<K::Canon, V> {
        self.0.iter()
    }
}


