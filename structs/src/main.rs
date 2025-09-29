mod clocks {
    use std::fmt::Display;

    #[derive(Clone, Default, Debug)]
    pub struct WallClockTime {
        hours: u8,
        minutes: u8,
    }

    impl Display for WallClockTime {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:02}:{:02}", self.hours, self.minutes)
        }
    }

    impl WallClockTime {
        pub fn new(hours: u8, minutes: u8) -> Self {
            Self { hours, minutes }
        }

        pub fn add_hours(&mut self, hours: u8) {
            self.hours = (self.hours + hours) % 24;
        }

        pub fn add_hours_clone(&self, hours: u8) -> Self {
            let mut new_time = self.clone();
            new_time.add_hours(hours);
            new_time
        }

        pub fn get_hours(&self) -> u8 {
            self.hours
        }

        pub fn get_minutes(&self) -> u8 {
            self.minutes
        }
    }
}

use clocks::WallClockTime;

fn main() {
    let mut time = WallClockTime::new(10, 30);
    time.add_hours(5);
    println!("Time: {:02}:{:02}", time.get_hours(), time.get_minutes());

    let time: WallClockTime = Default::default();
    println!(
        "Default Time: {:02}:{:02}",
        time.get_hours(),
        time.get_minutes()
    );

    let time_copy = time.clone();
    println!(
        "Copied Time: {:02}:{:02}",
        time_copy.get_hours(),
        time_copy.get_minutes()
    );

    println!("{:?}", time_copy);
}
