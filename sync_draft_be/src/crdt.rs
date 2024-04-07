pub mod crdt {

use crate::vtime::vt::VTime;


type ORSet<'a> = Vec<(&'a str, VTime)>;

enum Command<'a> {
    Add(&'a str),
    Remove(&'a str),
}

enum Op<'a> {
    Add(&'a str),
    Remove(Vec<VTime>),
}

pub struct Crdt<'a> {
    default: ORSet<'a>,
}

impl<'a> Crdt<'a> {
    fn new() -> Self {
        Crdt {
            default: Vec::new(),
        }
    }

    fn query(&self, orset: &ORSet<'a>) -> Vec<&'a str> {
        orset.iter().map(|(item, _)| *item).collect()
    }

    fn prepare(&self, orset: &ORSet<'a>, cmd: Command<'a>) -> Op<'a> {
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

    fn effect(&self, orset: &mut ORSet<'a>, e: Op<'a>) {
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