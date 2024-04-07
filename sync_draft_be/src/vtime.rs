pub mod vt {

use std::collections::HashMap;

#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Ord {
    Lt = -1,
    Eq = 0,
    Gt = 1,
    Cc = 2,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VTime {
    counters: HashMap<String, u64>,
}

impl VTime {
    pub fn new() -> VTime {
        VTime {
            counters: HashMap::new(),
        }
    }

    fn inc(&mut self, key: &str) {
        let counter = self.counters.entry(key.to_string()).or_insert(0);
        *counter += 1;
    }

    fn merge(&mut self, other: &VTime) {
        for (key, &value) in other.counters.iter() {
            let counter = self.counters.entry(key.to_string()).or_insert(0);
            *counter = (*counter).max(value);
        }
    }

    pub fn compare(a: &VTime, b: &VTime) -> Ord {
        let val_or_default = |k: &String, map: &VTime| {
            match map.counters.get(k) {
                Some(&v) => v,
                None => 0,
            }
        };
    
        let akeys: Vec<&String> = a.counters.keys().collect();
        let bkeys: Vec<&String> = b.counters.keys().collect();
    
        let mut union_keys = akeys.iter().chain(&bkeys).collect::<Vec<&&String>>();
        union_keys.sort();
        union_keys.dedup();
    
        let mut result = Ord::Eq;
    
        for key in union_keys {
            let va = val_or_default(*key, a);
            let vb = val_or_default(*key, b);
    
            match result {
                Ord::Eq if va > vb => result = Ord::Gt,
                Ord::Eq if va < vb => result = Ord::Lt,
                Ord::Lt if va > vb => result = Ord::Cc,
                Ord::Gt if va < vb => result = Ord::Cc,
                _ => (),
            }
        }
        result
    }
}

}