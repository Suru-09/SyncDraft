pub mod crdt {

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
    pub counters: HashMap<String, u64>,
}

impl VTime {
    pub fn new() -> VTime {
        VTime {
            counters: HashMap::new(),
        }
    }

    pub fn inc(&mut self, key: &str) {
        let counter = self.counters.entry(key.to_string()).or_insert(0);
        *counter += 1;
    }

    pub fn merge(&mut self, other: &VTime) {
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
    

pub type ORSet = Vec<(char, VTime)>;

pub enum Command {
    Add(char),
    Remove(char),
}

pub enum Op {
    Add(char),
    Remove(Vec<VTime>),
}

pub struct Crdt {
    default: ORSet,
}

impl Crdt {
    pub fn new() -> Self {
        Crdt {
            default: Vec::new(),
        }
    }

    pub fn query(&self, orset: &ORSet) -> Vec<char> {
        orset.iter().map(|(item, _)| *item).collect()
    }

    pub fn prepare(&self, orset: &ORSet, cmd: Command) -> Op {
        match cmd {
            Command::Add(item) => Op::Add(item),
            Command::Remove(item) => {
                let timestamps: Vec<VTime> = orset.iter()
                    .filter_map(|(i, ts)| if *i == item { Some(ts.clone()) } else { None })
                    .collect();
                Op::Remove(timestamps)
            }
        }
    }

    pub fn effect(&self, orset: &mut ORSet, e: Op) {
        match e {
            Op::Add(item) => {
                orset.push((item, VTime::new()));
            }
            Op::Remove(versions) => {
                orset.retain(|(_, ts)| !versions.contains(ts));
            }
        }
    }
}

}

#[cfg(test)]
mod tests {
    use super::crdt::*;

    #[test]
    fn test_vtime_inc() {
        let mut vtime = VTime::new();
        vtime.inc("a");
        vtime.inc("a");
        assert_eq!(vtime.counters.get("a"), Some(&2));
    }

    #[test]
    fn test_vtime_merge() {
        let mut vtime1 = VTime::new();
        vtime1.inc("a");
        vtime1.inc("b");
        vtime1.inc("b");
        vtime1.inc("b");
        let mut vtime2 = VTime::new();
        vtime2.inc("b");
        vtime2.inc("b");
        vtime2.inc("b");
        vtime2.inc("b");
        vtime1.merge(&vtime2);
        assert_eq!(vtime1.counters.get("a"), Some(&1));
        assert_eq!(vtime1.counters.get("b"), Some(&4));
    }

    #[test]
    fn test_vtime_compare() {
        let vtime1 = VTime::new();
        let mut vtime2 = VTime::new();
        vtime2.inc("a");
        assert_eq!(VTime::compare(&vtime1, &vtime2), Ord::Lt);
    }

    #[test]
    fn test_crdt_query() {
        let crdt = Crdt::new();
        let orset = vec![('a'.clone(), VTime::new()), ('b'.clone(), VTime::new())];
        assert_eq!(crdt.query(&orset), vec!['a', 'b']);
    }

    #[test]
    fn test_crdt_effect_add() {
        let mut orset = vec![('a', VTime::new()), ('b', VTime::new())];
        let crdt = Crdt::new();
        let op = Op::Add('c');
        crdt.effect(&mut orset, op);
        assert_eq!(orset.len(), 3);
    }

    #[test]
    fn test_crdt_effect_remove() {
        let mut orset = vec![('a', VTime::new()), ('b', VTime::new())];
        let crdt = Crdt::new();
        let op = Op::Remove(vec![VTime::new()]);
        crdt.effect(&mut orset, op);
        assert_eq!(orset.len(), 0);
    }
}