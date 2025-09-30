use std::{fs::File, io::Read};

fn main() {
    // Read the file
    let mut f = File::open("data.txt").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    let mut column_1: Vec<u32> = Vec::new();
    let mut column_2: Vec<u32> = Vec::new();
    contents.lines()
        .map(|line| line.split_once("   ").unwrap())
        .for_each(|(c1, c2)| {
            column_1.push(c1.parse().unwrap());
            column_2.push(c2.parse().unwrap());
        });

    column_1.sort();
    column_2.sort();

    let result: i32 = column_1.iter()
        .enumerate()
        .map(|(i, v)| (column_2[i] as i32 - *v as i32).abs())
        .sum();
    println!("Result: {}", result);

    let result: i32 = column_1.iter()
        .zip(column_2)
        .map(|(c1, c2)| (*c1 as i32 - c2 as i32).abs())
        .sum();
    println!("Result: {}", result);
}
