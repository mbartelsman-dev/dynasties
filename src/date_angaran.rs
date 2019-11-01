#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Date (i32, u32, u32);

impl Date {
    /// Dates will be defined in 12-month years

    const year: u64 = 31_556_952;
    const moon: u64 =  2_551_443;
    const day: u64 =      86_400;

    fn get_month(&mut self) {
        unimplemented!();
    }

    fn change(&mut self, year: i32, season: i32, day: i32) {
        unimplemented!();
    }

    /// Sets date with wrap-around for fields
    fn set_wrap(&mut self, year: i32, season: i32, day: i32) {

        let mut year = year;
        let mut season = season - 1;
        let mut day = day - 1;

        if day < 0 {
            season = season + day / 91 - 1;
            day = 91 - ((-day - 1) % 91);
        } else {
            season = season + day / 91;
            day = day % 91 + 1;
        }

        if season < 0 {
            year = year + season / 4 - 1;
            season = 4 - ((-season - 1) % 4);
        } else {
            year = year + season / 4;
            season = season % 4 + 1;
        }

        self.0 = year;
        self.1 = season as u32;
        self.2 = day as u32;
    }

    /// Sets date with truncation for fields
    fn set_trunc(&mut self, year: i32, season: i32, day: i32) {

        let mut year = year;
        let mut season = season;
        let mut day = day;

        if day == 0 { day = 1; }
        if season == 0 { season = 1; }
        if day > 91 { day = 91; }
        if season > 4 { season = 4; }

        self.0 = year;
        self.1 = season as u32;
        self.2 = day as u32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    #[test]
    fn set_wrap() {
        let mut date = Date(0,0,0);

        // Safe usage
        date.set_wrap(532, 2, 34);
        assert_eq!( Date(532, 2, 34), date );
        
        // Day wraps around
        date.set_wrap(0, 1, 92);
        assert_eq!( Date(0, 2, 1), date );

        // Season wraps around
        date.set_wrap(0, 5, 1);
        assert_eq!( Date(1, 1, 1), date );

        // Day and season wrap around
        date.set_wrap(1000, 9, 101);
        assert_eq!( Date(1002, 2, 10), date );

        // Day is zero, negative wrap
        date.set_wrap(532, 2, 0);
        assert_eq!( Date(532, 1, 91), date );

        // Day is negative, negative wrap
        date.set_wrap(532, 2, -1);
        assert_eq!( Date(532, 1, 90), date );

        // Day is negative, negative wrap, twice
        date.set_wrap(532, 2, -91);
        assert_eq!( Date(531, 4, 91), date );

        // Season is zero, negative wrap
        date.set_wrap(532, 0, 1);
        assert_eq!( Date(531, 4, 1), date );

        // Season is negative, negative wrap, twice
        date.set_wrap(532, -1, 1);
        assert_eq!( Date(531, 3, 1), date );

        // Season is negative, negative wrap, twice
        date.set_wrap(532, -6, 1);
        assert_eq!( Date(530, 2, 1), date );
    }

    #[test]
    fn set_trunc() {
        let mut date = Date(0,0,0);

        // Safe usage
        date.set_trunc(532, 2, 34);
        assert_eq!( Date(532, 2, 34), date );

        // Lower truncation
        date.set_trunc(532, 0, 0);
        assert_eq!( Date(532, 1, 1), date );

        // Upper truncation
        date.set_trunc(532, 5, 92);
        assert_eq!( Date(532, 4, 91), date );
        
    }
}
