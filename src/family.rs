use std::collections::{BTreeMap, HashSet};

use super::date::Date;

/// A graph of people
struct Family {
    tree: BTreeMap<usize, Person>,
    next_id: usize,
}

impl Family {

    /// Makes a new empty family tree
    fn new() -> Self {
        Family {
            tree: BTreeMap::new(),
            next_id: 1,
        }
    }

    /// Returns the number of people in Family
    fn len(&self) -> usize {
        self.tree.len()
    }

    /// Inserts a Person object to the tree
    fn insert(&mut self, mut person: Person) {
        person.id = self.next_id;
        self.tree.insert(self.next_id, person);
        self.next_id += 1;
    }

    /// Removes a person from the Family, returning that person if found
    fn remove(&mut self, id: &usize) -> Option<Person> {
        self.tree.remove(&id)
    }

    /// Returns a reference to a given person in the Family
    fn get(&self, id: &usize) -> Option<&Person> {
        self.tree.get(&id)
    }

    /// Returns a mutable reference to a given person in the Family
    fn get_mut(&mut self, id: &usize) -> Option<&mut Person> {
        self.tree.get_mut(&id)
    }

    /// Creates a Person with parents and date of birth and inserts them into the Family
    fn make_child(&mut self, name: &str, parent_1: &usize, parent_2: &usize, birth: &Date){
        let mut child = Person::new();

        child.name = String::from(name);
        child.events.push(Event::Birth(*birth));
        child.relatives.push(Relative::Parent(*parent_1));
        child.relatives.push(Relative::Parent(*parent_2));

        // Add as child to parent 1
        if let Some(p) = self.tree.get_mut(parent_1) {
            p.relatives.push(Relative::Child(self.next_id))
        }

        // Add as child to parent 2
        if let Some(p) = self.tree.get_mut(parent_2) {
            p.relatives.push(Relative::Child(self.next_id))
        }

        self.insert(child);
    }
}

struct Person {
    id: usize,
    name: String,
    relatives: Vec<Relative>,
    events: Vec<Event>,
    feats: HashSet<Feat>,
}

impl Person {

    /// Returns a new person with empty fields
    fn new() -> Person {
        Person {
            id: 0,
            name: String::from(""),
            relatives: Vec::new(),
            events: Vec::new(),
            feats: HashSet::new(),
        }
    }

    /// Adds a new event to the preson's life
    fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    /// Ads a new feature to the person
    fn add_feat(&mut self, feat: Feat) {
        self.feats.insert(feat);
    }
}

enum Relative {
    Parent(usize),
    Child(usize),
    Sibling(usize),
    /* ... */
    Other(String, usize),
}

enum Event {
    Birth(Date),
    Death(Date),
    /* ... */
    Other(String, Date),
}

#[derive(PartialEq, Eq, Hash)]
enum Feat {
    Valiant,
    Craven,
}
