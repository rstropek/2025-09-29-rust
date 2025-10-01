fn main() {
    fn add(x: i32, y: i32) -> i32 { x + y }
    // let f = add;
    
    let f = |x: i32, y: i32| -> i32 { x + y };
    let f2 = |x: i32, y: i32| -> i32 { x * y };
    
    calc_and_print(2, 3, add);
    calc_and_print(2, 3, f);
    calc_and_print(2, 3, f2);
    
    let correction_factor = 5;
    let f3 = |x: i32, y: i32| -> i32 { x * y + correction_factor };
    calc_and_print(2, 3, f3);

    // NOTE: Ownership + borrowing rules apply!!!!!
}

fn calc_and_print(x: i32, y: i32, op: impl Fn(i32, i32) -> i32) {
    let result = op(x, y);
    println!("Result: {}", result);
}

fn calc_and_print_2(x: i32, y: i32, op: Box<dyn Fn(i32, i32) -> i32>) {
    let result = op(x, y);
    println!("Result: {}", result);
}
