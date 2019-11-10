mod person {

    use std::collections::HashSet;
    use crate::id::Id;
    use crate::name::Name;
    use crate::event::Event;
    use crate::relation::Relation;
    use crate::feat::Feat;
    pub struct Person {
        id: Id,
        name: Name,
        events: Vec<Event>,
        relations: Vec<Relation>,
        feats: HashSet<Feat>,
    }

    impl Person {
        // TODO: !!!
    }

    #[cfg(test)]
    mod test {
        use super::*;
        
    }
}

mod id {
    pub struct Id (usize);

    impl Id {
        // TODO:
    }

    #[cfg(test)]
    mod test {
        use super::*;
        
    }
}

mod name {
    pub struct Name {
        given: String,
        family: String,
    }

    impl Name {
        // TODO: !!!
    }

    #[cfg(test)]
    mod test {
        use super::*;
        
    }
}

mod event {

    use crate::date::Date;
    pub enum Event {
        Birth(Date),
        Death(Date),
    }

    impl Event {
        // TODO: !!!
    }

    #[cfg(test)]
    mod test {
        use super::*;
        
    }
}

mod date {
    pub struct Date (u32,u32);

    impl Date {
        // TODO: !!!
    }

    #[cfg(test)]
    mod test {
        use super::*;
        
    }
}

mod relation {

    use crate::id::Id;
    pub enum Relation {
        Parent(Id),
        Child(Id)
    }

    impl Relation {
        // TODO: !!!
    }

    #[cfg(test)]
    mod test {
        use super::*;
        
    }
}

mod feat {
    pub enum Feat {
        Infertile,
    }

    impl Feat {
        // TODO: !!!
    }

    #[cfg(test)]
    mod test {
        use super::*;
        
    }
}
