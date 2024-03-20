// everything i32 to avoid casting

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

const HOUR_MINUTES: i32 = 60;
const DAY_HOURS: i32 = 24;
const DAY_MINUTES: i32 = DAY_HOURS * HOUR_MINUTES;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = (hours * HOUR_MINUTES + minutes).rem_euclid(DAY_MINUTES);
        // handle negative input
        let total_minutes = (total_minutes + DAY_MINUTES).rem_euclid(DAY_MINUTES);
        let hours = (total_minutes.div_euclid(HOUR_MINUTES)).rem_euclid(DAY_HOURS);
        let minutes = total_minutes.rem_euclid(HOUR_MINUTES);

        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        return Clock::new(self.hours, self.minutes + minutes);
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}
