use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
struct Item<K, V> {
    key: K,
    value: V,
}

#[derive(Clone, Debug)]
struct HashTable<K, V> {
    items: Vec<Vec<Item<K, V>>>,
    capacity: u64,
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

impl<K, V> HashTable<K, V>
where
    K: std::hash::Hash,
    K: std::cmp::PartialEq,
    K: std::clone::Clone,
    V: std::clone::Clone,
{
    fn new(capacity: usize) -> Self {
        Self {
            items: vec![vec![]; capacity],
            capacity: capacity as u64,
        }
    }

    fn put(&mut self, key: K, value: V) {
        self.delete(key.clone());
        let hash = (calculate_hash(&key) % self.capacity) as usize;
        self.items[hash].push(Item { key, value });
    }

    fn get(&self, key: K) -> Option<&V> {
        let hash = (calculate_hash(&key) % self.capacity) as usize;
        if let Some(item) = self.items[hash].iter().find(|item| item.key == key) {
            return Some(&item.value);
        }
        None
    }

    fn delete(&mut self, key: K) {
        let hash = (calculate_hash(&key) % self.capacity) as usize;
        if let Some((index, _item)) = self.items[hash]
            .iter()
            .enumerate()
            .find(|(_, item)| item.key == key)
        {
            self.items[hash].remove(index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_table() {
        let mut table = HashTable::<String, String>::new(10);
        table.put("dog".to_string(), "Doge".to_string());
        assert_eq!(table.get("dog".to_string()).unwrap(), "Doge");
        table.delete("dog".to_string());
        assert_eq!(table.get("dog".to_string()), None);
    }
}

fn main() {
    let mut table = HashTable::<String, String>::new(10);
    table.put("dog".to_string(), "Doge".to_string());
    table.get("dog".to_string());
    table.delete("dog".to_string());
}
