use crate::date::Date;

pub enum Event {
    Birth(Date),
    Death(Date),
}
