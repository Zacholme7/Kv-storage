use std::collections::hash_map::HashMap;
use std::error::Error;

pub struct Memory {
    store: HashMap<String, String>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            store: HashMap::new(),
        }
    }

    pub fn process_values(&mut self, values: Vec<&str>) -> Result<(), Box<dyn Error>> {
        match values[0] {
            "get" => {
                if values.len() < 2 {
                    return Err("No key provided for 'get'".into());
                }
                self.process_get(values[1])
            }
            "set" => {
                if values.len() < 3 {
                    return Err("Not enough arguments for 'set'".into());
                }
                self.process_set(values[1], values[2])
            }
            "del" => {
                if values.len() < 2 {
                    return Err("No key provided for 'del'".into());
                }
                self.process_del(values[1])
            }
            _ => Err("Unsupported command".into()),
        }
    }

    fn process_get(&self, key: &str) -> Result<(), Box<dyn Error>> {
        match self.store.get(key) {
            Some(val) => {
                println!("Value: {}", val);
                Ok(())
            }
            None => Err("Key not found".into()),
        }
    }

    fn process_set(&mut self, key: &str, value: &str) -> Result<(), Box<dyn Error>> {
        // Insert the key-value pair into the store
        self.store.insert(key.to_string(), value.to_string());
        println!("Set successful");
        Ok(())
    }

    fn process_del(&mut self, key: &str) -> Result<(), Box<dyn Error>> {
        // Remove the key from the store
        match self.store.remove(key) {
            Some(_) => {
                println!("Delete successful");
                Ok(())
            }
            None => Err("Key not found".into()),
        }
    }
}
