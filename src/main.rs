use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::io::{self};


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum KvEnum {
    Int(usize),
    Str(String),
    Bool(bool),
}
#[derive(Debug)]
pub struct KeyValueStore {
    store: HashMap<String, KvEnum>,
    file_path: String,
}

impl KeyValueStore {
    pub fn new(file_path: &str) -> std::io::Result<Self> {
        let store = match File::open(&file_path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let mut store = HashMap::new();
                for line in reader.lines() {
                    let line = line?;
                    let parts: Vec<&str> = line.split('=').collect();
                    if parts.len() != 2 {
                        return Err(std::io::Error::new(
                            ErrorKind::InvalidData,
                            "Malformed file error",
                        ));
                    }
                    let key = parts[0].to_string();
                    let value = match parts[1] {
                        "true" => KvEnum::Bool(true),
                        "false" => KvEnum::Bool(false),
                        s => {
                            if let Ok(int) = s.parse::<usize>() {
                                KvEnum::Int(int)
                            } else {
                                KvEnum::Str(s.to_string())
                            }
                        }
                    };
                    store.insert(key, value);
                }
                store
            }
            Err(ref err) if err.kind() == ErrorKind::NotFound => HashMap::new(),
            Err(error) => return Err(error),
        };
        Ok(Self {
            store,
            file_path: file_path.to_string(),
        })
    }

    pub fn insert(&mut self, key: String, value: KvEnum) -> std::io::Result<InsertResult> {
        self.store.insert(key.clone(), value.clone());
        let mut file = File::create(&self.file_path)?;
        writeln!(&mut file, "{}", serde_json::to_string(&self.store)?)?;
        Ok(InsertResult::Inserted)
    }

    pub fn get(&self, key: &str) -> std::io::Result<Option<KvEnum>> {
        Ok(self.store.get(key).cloned())
    }

    pub fn remove(&mut self, key: &str) -> std::io::Result<RemoveResult> {
        self.store.remove(key);
        let mut file = File::create(&self.file_path)?;
        for (k, v) in &self.store {
            writeln!(&mut file, "{}={}", k, serde_json::to_string(v)?)?;
        }
        Ok(RemoveResult::Removed)
    }

    pub fn len(&self) -> usize {
        self.store.len()
    }

    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    pub fn clear(&mut self) -> std::io::Result<()> {
        self.store.clear();
        let file = File::create(&self.file_path)?;
        file.set_len(0)?;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum InsertResult {
    Inserted,
}

#[derive(Debug, PartialEq)]
pub enum RemoveResult {
    Removed,
}

#[derive(Debug, PartialEq)]
pub enum KvError {
    MalformedFileError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        // Test creating a new KeyValueStore instance
        let kvs = KeyValueStore::new("test_new.txt").unwrap();
        assert_eq!(kvs.len(), 0);
        assert!(kvs.is_empty());
    }

    #[test]
    fn test_insert() {
        // Test inserting a new key-value pair
        let mut kvs: KeyValueStore = KeyValueStore::new("test_insert.txt").unwrap();
        assert_eq!(kvs.insert("foo".to_string(), KvEnum::Int(42)).unwrap(), InsertResult::Inserted);
        assert_eq!(kvs.len(), 1);
        assert!(!kvs.is_empty());
        assert_eq!(kvs.get("foo").unwrap().unwrap(), KvEnum::Int(42));
    }

    #[test]
    fn test_get() {
        // Test getting an existing key-value pair
        let mut kvs = KeyValueStore::new("test_get.txt").unwrap();
        kvs.insert("foo".to_string(), KvEnum::Bool(true)).unwrap();
        assert_eq!(kvs.get("foo").unwrap().unwrap(), KvEnum::Bool(true));

        // Test getting a non-existent key
        assert_eq!(kvs.get("bar").unwrap(), None);
    }

    #[test]
    fn test_remove() {
        // Test removing an existing key-value pair
        let mut kvs = KeyValueStore::new("test_remove.txt").unwrap();
        kvs.insert("foo".to_string(), KvEnum::Bool(false)).unwrap();
        assert_eq!(kvs.remove("foo").unwrap(), RemoveResult::Removed);
        assert_eq!(kvs.len(), 0);

        // Test removing a non-existent key
        assert_eq!(kvs.remove("bar").unwrap(), RemoveResult::Removed);
    }

    #[test]
    fn test_clear() {
        // Test clearing the store
        let mut kvs = KeyValueStore::new("test_clear.txt").unwrap();
        kvs.insert("foo".to_string(), KvEnum::Bool(true)).unwrap();
        kvs.clear().unwrap();
        assert_eq!(kvs.len(), 0);
        assert!(kvs.is_empty());
    }
}


fn main() -> io::Result<()> {
    let mut kvs = KeyValueStore::new("data.txt")?;
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        let parts: Vec<&str> = input.split_whitespace().collect();
        match parts[0] {
            "get" => {
                if parts.len() != 2 {
                    println!("Usage: get <key>");
                    continue;
                }
                match kvs.get(parts[1])? {
                    Some(value) => println!("{:?}", value),
                    None => println!("Key not found"),
                }
            }
            "set" => {
                if parts.len() != 3 {
                    println!("Usage: set <key> <value>");
                    continue;
                }
                let key = parts[1].to_string();
                let value = match parts[2].parse::<usize>() {
                    Ok(int) => KvEnum::Int(int),
                    Err(_) => KvEnum::Str(parts[2].to_string()),
                };
                match kvs.insert(key, value)? {
                    InsertResult::Inserted => println!("Value inserted"),
                }
            }
            "remove" => {
                if parts.len() != 2 {
                    println!("Usage: remove <key>");
                    continue;
                }
                match kvs.remove(parts[1])? {
                    crate::RemoveResult::Removed => println!("Key removed"),
                }
            }
            "list" => {
                for (key, value) in &kvs.store {
                    println!("{}: {:?}", key, value);
                }
            }
            "clear" => {
                match kvs.clear() {
                    Ok(_) => println!("Store cleared"),
                    Err(err) => println!("Failed to clear store: {:?}", err),
                }
            }
            "exit" => break,
            _ => println!("Unknown command"),
        }
    }
    Ok(())
}
