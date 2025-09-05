use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fmt::Display;

const INITIAL_CAPACITY: usize = 16;
const LOAD_FACTOR_THRESHOLD: f64 = 0.75;

#[derive(Debug, Clone)]
struct Entry<K, V> {
    key: K,
    value: V,
    is_deleted: bool,
}

#[derive(Debug)]
struct HashTable<K, V> {
    buckets: Vec<Option<Entry<K, V>>>,
    size: usize,
    capacity: usize,
}

impl<K: Hash + Eq + Clone, V: Clone> HashTable<K, V> {
    fn new() -> Self {
        HashTable {
            buckets: vec![None; INITIAL_CAPACITY],
            size: 0,
            capacity: INITIAL_CAPACITY,
        }
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.capacity
    }

    fn load_factor(&self) -> f64 {
        self.size as f64 / self.capacity as f64
    }

    fn resize(&mut self) {
        let old_buckets = std::mem::replace(&mut self.buckets, vec![None; self.capacity * 2]);
        let old_capacity = self.capacity;
        self.capacity *= 2;
        self.size = 0;

        // Rehash all existing entries
        for bucket in old_buckets {
            if let Some(entry) = bucket {
                if !entry.is_deleted {
                    self.insert_internal(entry.key, entry.value);
                }
            }
        }

        println!("Hash table resized from {} to {} buckets", old_capacity, self.capacity);
    }

    fn find_slot(&self, key: &K) -> (usize, bool) {
        let mut index = self.hash(key);
        let mut first_deleted = None;
        
        for _ in 0..self.capacity {
            match &self.buckets[index] {
                None => {
                    // Empty slot found
                    return (first_deleted.unwrap_or(index), false);
                }
                Some(entry) => {
                    if entry.is_deleted {
                        if first_deleted.is_none() {
                            first_deleted = Some(index);
                        }
                    } else if entry.key == *key {
                        // Key found
                        return (index, true);
                    }
                }
            }
            
            // Linear probing
            index = (index + 1) % self.capacity;
        }
        
        // Table is full (shouldn't happen with proper resizing)
        (first_deleted.unwrap_or(0), false)
    }

    fn insert_internal(&mut self, key: K, value: V) {
        let (index, found) = self.find_slot(&key);
        
        if found {
            // Update existing entry
            if let Some(entry) = &mut self.buckets[index] {
                entry.value = value;
            }
        } else {
            // Insert new entry
            self.buckets[index] = Some(Entry {
                key,
                value,
                is_deleted: false,
            });
            self.size += 1;
        }
    }

    fn insert(&mut self, key: K, value: V) {
        if self.load_factor() > LOAD_FACTOR_THRESHOLD {
            self.resize();
        }
        
        self.insert_internal(key, value);
    }

    fn get(&self, key: &K) -> Option<&V> {
        let (index, found) = self.find_slot(key);
        
        if found {
            if let Some(entry) = &self.buckets[index] {
                if !entry.is_deleted {
                    return Some(&entry.value);
                }
            }
        }
        None
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let (index, found) = self.find_slot(key);
        
        if found {
            if let Some(entry) = &mut self.buckets[index] {
                if !entry.is_deleted {
                    entry.is_deleted = true;
                    self.size -= 1;
                    return Some(entry.value.clone());
                }
            }
        }
        None
    }

    fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn keys(&self) -> Vec<&K> {
        self.buckets
            .iter()
            .filter_map(|bucket| {
                if let Some(entry) = bucket {
                    if !entry.is_deleted {
                        Some(&entry.key)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    fn values(&self) -> Vec<&V> {
        self.buckets
            .iter()
            .filter_map(|bucket| {
                if let Some(entry) = bucket {
                    if !entry.is_deleted {
                        Some(&entry.value)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    fn display_internal_structure(&self) {
        println!("Hash Table Internal Structure:");
        println!("Capacity: {}, Size: {}, Load Factor: {:.2}", 
                 self.capacity, self.size, self.load_factor());
        println!("Buckets:");
        
        for (i, bucket) in self.buckets.iter().enumerate() {
            match bucket {
                None => println!("  [{}]: Empty", i),
                Some(entry) => {
                    let status = if entry.is_deleted { " (DELETED)" } else { "" };
                    println!("  [{}]: {:?} -> {:?}{}", i, entry.key, entry.value, status);
                }
            }
        }
    }
}

fn main() {
    println!("🗂️  Hash Table Implementation");
    println!("=============================");

    let mut hash_table: HashTable<String, i32> = HashTable::new();

    loop {
        println!("\nOptions:");
        println!("1. Insert key-value pair");
        println!("2. Get value by key");
        println!("3. Remove key");
        println!("4. Check if key exists");
        println!("5. Display all keys");
        println!("6. Display all values");
        println!("7. Show hash table statistics");
        println!("8. Show internal structure");
        println!("9. Run demo");
        println!("10. Exit");
        print!("Choose an option (1-10): ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        
        match input.trim() {
            "1" => {
                print!("Enter key: ");
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                let mut key = String::new();
                std::io::stdin().read_line(&mut key).unwrap();
                let key = key.trim().to_string();

                print!("Enter value: ");
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                let mut value_input = String::new();
                std::io::stdin().read_line(&mut value_input).unwrap();
                
                if let Ok(value) = value_input.trim().parse::<i32>() {
                    hash_table.insert(key.clone(), value);
                    println!("✓ Inserted: {} -> {}", key, value);
                } else {
                    println!("Invalid value! Please enter an integer.");
                }
            }
            "2" => {
                print!("Enter key to search: ");
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                let mut key = String::new();
                std::io::stdin().read_line(&mut key).unwrap();
                let key = key.trim();

                match hash_table.get(&key.to_string()) {
                    Some(value) => println!("✓ Found: {} -> {}", key, value),
                    None => println!("✗ Key '{}' not found", key),
                }
            }
            "3" => {
                print!("Enter key to remove: ");
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                let mut key = String::new();
                std::io::stdin().read_line(&mut key).unwrap();
                let key = key.trim().to_string();

                match hash_table.remove(&key) {
                    Some(value) => println!("✓ Removed: {} -> {}", key, value),
                    None => println!("✗ Key '{}' not found", key),
                }
            }
            "4" => {
                print!("Enter key to check: ");
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                let mut key = String::new();
                std::io::stdin().read_line(&mut key).unwrap();
                let key = key.trim().to_string();

                if hash_table.contains_key(&key) {
                    println!("✓ Key '{}' exists", key);
                } else {
                    println!("✗ Key '{}' does not exist", key);
                }
            }
            "5" => {
                let keys = hash_table.keys();
                if keys.is_empty() {
                    println!("No keys in hash table");
                } else {
                    println!("Keys: {:?}", keys);
                }
            }
            "6" => {
                let values = hash_table.values();
                if values.is_empty() {
                    println!("No values in hash table");
                } else {
                    println!("Values: {:?}", values);
                }
            }
            "7" => {
                println!("Hash Table Statistics:");
                println!("  Size: {}", hash_table.len());
                println!("  Capacity: {}", hash_table.capacity);
                println!("  Load Factor: {:.2}", hash_table.load_factor());
                println!("  Empty: {}", hash_table.is_empty());
            }
            "8" => {
                hash_table.display_internal_structure();
            }
            "9" => {
                println!("Running demo with sample data...");
                let demo_data = vec![
                    ("apple", 5),
                    ("banana", 3),
                    ("cherry", 8),
                    ("date", 2),
                    ("elderberry", 12),
                ];

                for (key, value) in demo_data {
                    hash_table.insert(key.to_string(), value);
                    println!("Inserted: {} -> {}", key, value);
                }

                println!("\nDemo complete! Hash table now contains {} items.", hash_table.len());
            }
            "10" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice! Please enter 1-10.");
            }
        }
    }
}