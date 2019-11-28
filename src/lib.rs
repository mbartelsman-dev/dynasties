mod family {
    use petgraph::prelude::*;
    use crate::person::Person;
    use crate::relation::Relation;
    use crate::date::Date;

    struct Family {
        people: StableGraph<Person, Relation, Directed>,
        year: Date,
    }
<<<<<<< HEAD

    
=======
>>>>>>> 327016295536c0663d3e162cc0f7bfade1059463
}

mod date;
mod person;
mod relation;
mod event;
mod feat;
