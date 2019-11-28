mod family {
    use petgraph::prelude::*;
    use crate::person::Person;
    use crate::relation::Relation;
    use crate::date::Date;

    struct Family {
        people: StableGraph<Person, Relation, Directed>,
        year: Date,
    }

    
}

mod date;
mod person;
mod relation;
mod event;
mod feat;
