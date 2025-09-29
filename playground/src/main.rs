#![allow(unused_variables, dead_code, unused_assignments)]

use std::collections::HashSet;

use rand::{prelude::*, rng};

mod utils;

fn main() {
    let a = 42i64;
    let b = get_a_number();

    let c = a + b;

    let user_input = "42";
    let user_input: i32 = user_input.parse().unwrap();

    let mut a = 42;
    a += 1;
    let a = a;
    let mut a = a;
    a += 1;

    let random_number = rng().random_range(0..=10);
    println!(
        "{}",
        match random_number {
            n if n > 5 => "Win".to_string(),
            n if n < 5 => "Lose".to_string(),
            _ => "Draw".to_string(),
        }
    );

    let add_result = utils::add(5, 10);
    let multiply_result = utils::more_detailed_utils::multiply(5, 10);

    let name = namegenerator::generate_name();
    println!("Generated name: {}", name);

    // Arrays
    let mut numbers = [1, 2, 3, 4, 5];
    let first = numbers[0];
    let len = numbers.len();
    numbers[0] = 42;
    for number in numbers.iter() {
        println!("{}", number);
    }

    //let mut numbers = vec![1, 2, 3, 4, 5];
    let mut numbers = Vec::with_capacity(5);
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    numbers.push(4);
    numbers.push(5);
    let first = numbers[0];
    let len = numbers.len();
    numbers[0] = 42;
    numbers.push(6);
    for number in numbers {
        println!("{}", number);
    }

    // Tuples
    let mut coordinate = (3.7, 5.2, "this is a coordinate");
    let x = coordinate.0;
    let (x, _, desc) = coordinate;
    coordinate.0 = 4.2;
    let coordinate = (1, 2, "something");

    // String
    let mut s = "Rainer".to_string();
    s.push(' ');

    let my_vec: Vec<i16> = (1i16..5).collect();
    let my_vec = (1i16..=5).collect::<Vec<i16>>();
    let my_data: i64 = "42".parse().unwrap();
    let my_data = "not a number".parse::<i64>().unwrap();
}

fn div(a: i32, b: i32) -> i32 {
    if b == 0 { 0 } else { a / b }
}

fn get_a_number() -> i64 {
    let res = {
        let mut sum = 0;
        let numbers = [1, 2, 3, 4, 5];
        for num in numbers {
            sum += num;
        }
        sum
    };
    res
}
