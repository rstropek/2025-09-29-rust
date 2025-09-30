#![allow(dead_code, unused_variables)]

enum Colors {
    Red = 1,
    Green = 2,
    Blue = 4,
    Violet = 8,
}

struct Guest {
    name: String,
    age: u8,
}

enum HotelRoom {
    Empty,
    Occupied(Guest),
    UnderMaintenance(String),
}

impl HotelRoom {
    fn is_occupied(&self) -> bool {
        match self {
            HotelRoom::Occupied(_) => true,
            _ => false,
        }
    }
}

fn main() {
    {
        let my_color = Colors::Green;

        match my_color {
            Colors::Red => println!("The color is Red"),
            Colors::Green => println!("The color is Green"),
            Colors::Blue => println!("The color is Blue"),
            Colors::Violet => println!("The color is Violet"),
        }
    }

    {
        let room = HotelRoom::Occupied(Guest {
            name: String::from("Alice"),
            age: 30,
        });
        // match room {
        //     HotelRoom::Empty => println!("The room is empty."),
        //     HotelRoom::Occupied(guest) => {
        //         println!("The room is occupied by {} who is {} years old.", guest.name, guest.age)
        //     }
        //     HotelRoom::UnderMaintenance(reason) => {
        //         println!("The room is under maintenance due to: {}", reason)
        //     }
        // }
        if let HotelRoom::Occupied(guest) = room {
            println!("Guest details - Name: {}, Age: {}", guest.name, guest.age);
        }

        // let guest = Guest { name: String::from("Bob"), age: 25 };
        // if let Guest { age: 25, name } = guest {
        //     println!("Guest is 25 years old, name starts with: {}", &name[0..1]);
        // }
    }

    {
        let optional: Option<i32> = Option::None;
        let optional: Option<i32> = Option::Some(42);

        match optional {
            Option::Some(value) if value > 40 => println!("The value is greater than 40: {}", value),
            Option::Some(value) => println!("The value is: {}", value),
            Option::None => println!("No value present"),
        }

        if let Option::Some(value) = optional {
            println!("The value inside Option is: {}", value);
        }

        let mut stack = vec![Some(1), Some(2), None, Some(3)];
        while let Some(item) = stack.pop() {
            if let Some(value) = item {
                println!("Popped value: {}", value);
            } else {
                println!("Popped None");
            }
        }
    }

    {
        let optional: Option<i32> = Option::None;
        println!("100 * 2 = {:?}", optional.map(|x| x * 2));

        let mut optional: Option<String> = Option::Some(String::from("hello"));
        let len = optional.as_ref().map(|s| s.len());
        println!("Length of string inside Option: {:?}", len);
        
        if let Some(s) = optional.as_mut() {
            s.push_str(" world");
        }
        
        let cloned = optional.as_ref().cloned();
        
        let mut optional: Option<String> = Option::Some(String::from("hello"));
        println!("Before take: {:?}", optional);
        let taken = optional.take();
        println!("After take: {:?}, taken value: {:?}", optional, taken);
        
        let mut optional: Option<String> = Option::Some(String::from("hello"));
        let prev = optional.replace("world".to_string());
    }

    {
        let result = div(10, 2);
        match result {
            Ok(value) => println!("Division result: {}", value),
            Err(err) => println!("Error: {}", err),
        }

        if let Ok(value) = div(10, 0) {
            println!("Division result: {}", value);
        }
    }
}

fn get_len_of_string(s: &String) -> usize {
    s.len()
}

fn div(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
}