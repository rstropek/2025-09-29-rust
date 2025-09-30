#![allow(dead_code, unused_variables)]

use rand::prelude::*;

struct ConsultingWork {
    what: String,
    hours: f32,
    rate: f32,
}

impl Default for ConsultingWork {
    fn default() -> Self {
        Self {
            what: String::from(""),
            hours: 0.0,
            rate: 120.0,
        }
    }
}

trait Billable {
    fn total(&self) -> f32;
}

impl Billable for ConsultingWork {
    fn total(&self) -> f32 {
        self.hours * self.rate
    }
}

// We can implement our own traits for structs/enums that we do not own!
// We can implement traits we do not own for our own types!
// We can implement our own traits for primitive types!
impl Billable for f32 {
    fn total(&self) -> f32 {
        *self
    }
}

// Trait object
fn create_billable() -> Box<dyn Billable> {
    let val = rand::rng().random_bool(0.5);
    if val {
        Box::new(ConsultingWork {
            what: String::from("Rust consulting"),
            hours: 10.0,
            rate: 150.0,
        })
    } else {
        Box::new(42.0)
    }
}

fn print_billable(item: &dyn Billable) {
    println!("Total billable amount: ${:.2}", item.total());
}

impl From<f32> for ConsultingWork {
    fn from(value: f32) -> Self {
        Self {
            what: String::from("Miscellaneous work"),
            hours: 1.0,
            rate: value,
        }
    }
}

trait Pointworthy {
    fn points(&self) -> u32;
}

impl<T: Billable> Pointworthy for T {
    fn points(&self) -> u32 {
        (self.total() / 10.0).round() as u32
    }
}

fn main() {
    // A can be constructed FROM B
    // B can be turned INTO A

    let cw = ConsultingWork::from(42.0);
    let cw: ConsultingWork = 42.0f32.into();

    let billable = create_billable();
    print_billable(billable.as_ref());

    let billable2 = 42.0;
    print_billable(&billable2);

    let billables: Vec<Box<dyn Billable>> = vec![
        Box::new(ConsultingWork {
            what: String::from("Rust consulting"),
            hours: 5.0,
            rate: 150.0,
        }),
        Box::new(100.0),
        Box::new(ConsultingWork {
            what: String::from("Web development"),
            hours: 8.0,
            rate: 100.0,
        }),
    ];
    for item in billables {
        print_billable(item.as_ref());
    }
}
