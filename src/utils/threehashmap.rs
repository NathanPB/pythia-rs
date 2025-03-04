///! TODO: Rename mod and reexport from the super module.
use std::collections::HashMap;
use std::hash::Hash;

/// A HashMap with two keys.
#[derive(Debug)]
pub struct K2HashMap<K1, K2, V> {
    map: HashMap<K1, HashMap<K2, V>>,
}

impl<K1, K2, V> K2HashMap<K1, K2, V>
where
    K1: Eq + Hash,
    K2: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key1: K1, key2: K2, value: V) {
        self.map
            .entry(key1)
            .or_insert_with(HashMap::new)
            .insert(key2, value);
    }

    pub fn get(&self, key1: &K1, key2: &K2) -> Option<&V> {
        self.map.get(key1)?.get(key2)
    }

    pub fn contains_key(&self, key1: &K1, key2: &K2) -> bool {
        self.map
            .get(key1)
            .map(|m| m.contains_key(key2))
            .unwrap_or(false)
    }

    pub fn keys(&self) -> impl Iterator<Item = (&K1, &K2)> {
        self.map
            .iter()
            .flat_map(|(k1, v)| v.keys().map(move |k2| (k1, k2)))
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.map.values().flat_map(|v| v.values())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K1, &K2, &V)> {
        self.map
            .iter()
            .flat_map(|(k1, v)| v.iter().map(move |(k2, v)| (k1, k2, v)))
    }

    pub fn len(&self) -> usize {
        self.map.values().fold(0, |acc, v| acc + v.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_way_map() {
        let mut map = K2HashMap::new();

        map.insert("k1", "kA", 10);
        map.insert("k1", "kB", 20);
        map.insert("k2", "kC", 30);

        assert_eq!(map.get(&"k1", &"kA"), Some(&10));
        assert_eq!(map.get(&"k1", &"kB"), Some(&20));
        assert_eq!(map.get(&"k2", &"kC"), Some(&30));
    }
}
