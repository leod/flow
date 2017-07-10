use std::collections::{HashMap, hash_map};
use std::hash::Hash;
use std::iter::FromIterator;

pub trait Canonize {
    type Canon: Eq + Hash;

    fn canonize(&self) -> Self::Canon;
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CanonMap<K: Canonize, V>(HashMap<K::Canon, V>);

impl<K: Eq + Hash + Copy + Canonize, V> CanonMap<K, V> {
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

impl<K: Eq + Hash + Copy + Canonize, V> FromIterator<(K, V)> for CanonMap<K, V> {
    fn from_iter<I: IntoIterator<Item=(K, V)>>(iter: I) -> Self {
        let mut m = CanonMap::new();

        for (k, v) in iter {
            m.set(k, v);
        }

        m
    }
}
