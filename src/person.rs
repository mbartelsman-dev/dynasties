use std::collections::HashSet;
use crate::event::Event;
use crate::feat::Feat;

pub struct Person {
    id: usize,
    name: Name,
    events: Vec<Event>,
    feats: HashSet<Feat>,
}

impl Person {
    fn new() -> Self {
        Person {
            id: 0,
            name: Name::new(),
            events: Vec::new(),
            feats: HashSet::new(),
        }
    }
}


struct Name {
    given: String,
    family: String,
}

impl Name {
    fn new() -> Self {
        Name {
            given: String::from("Jay"),
            family: String::from("Random"),
        }
    }
}
