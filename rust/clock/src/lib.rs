use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock::rollover(hours, minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::rollover(self.hours, self.minutes + minutes)
    }

    fn rollover(mut hours: i32, mut minutes: i32) -> Self {
        while minutes < 0 {
            minutes += 60;
            hours -= 1;
        }

        while hours < 0 {
            hours += 24;
        }
        Clock {
            hours: ( hours + ( minutes / 60 ) ) % 24,
            minutes: minutes % 60,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}
