#[derive(Eq, PartialEq, Debug)]
pub struct Date {
    year: i32,
    season: u32,
}

impl Date {
    fn new() -> Date {
        Date {
            year: 0,
            season: 0,
        }
    }

    fn set_date(&mut self, year: i32, season: u32) {

    }

    fn incr_date(&mut self) -> &mut Date {
        let date = Date {
            year: self.year + (self.season as i32 + 1) / 4,
            season: (self.season + 1) % 4,
        };
        self.year = date.year;
        self.season = date.season;
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn incr_date() {
        let mut date = Date::new();

        date.incr_date();
        assert_eq!(date, Date{ year: 0, season: 1 });

        date.incr_date().incr_date();
        assert_eq!(date, Date{ year: 0, season: 3 });

        date.incr_date();
        assert_eq!(date, Date{ year: 1, season: 0 });
    }
}
