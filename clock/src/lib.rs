use std::fmt;

fn add_time(base_hours: i32, base_minutes: i32, hours: i32, minutes: i32) -> (i32, i32) {
    let total_minutes = base_minutes + minutes;
    let total_hours = base_hours + hours;
    let hour_adjustment_from_minutes = (total_minutes as f32 / 60.0).floor() as i32;
    let hours = (total_hours + hour_adjustment_from_minutes).rem_euclid(24);
    let minutes = (base_minutes + minutes).rem_euclid(60);

    (hours, minutes)
}

#[derive(Eq, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hours, minutes) = add_time(0, 0, hours, minutes);

        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (hours, minutes) = add_time(self.hours, self.minutes, 0, minutes);

        Self { hours, minutes }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}
