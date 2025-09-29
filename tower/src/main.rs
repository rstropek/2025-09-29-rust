const START: i32 = 2;
const HEIGHT: i32 = 8;

enum Direction {
    Up,
    Down,
}

fn construct(start: i32, height: i32, direction: Direction) -> i32 {
    // let (operator, separator): (Box<dyn Fn(i32, i32) -> i32>, &str) = match direction {
    //     Direction::Up => (Box::new(|a: i32, b: i32| a * b), "*"),
    //     Direction::Down => (Box::new(|a: i32, b: i32| a / b), "/"),
    // };



    let mut result: i32 = 0;
    let mut _start: i32 = start;

    for i in 2..height {
        result = match direction {
            Direction::Up => _start * i,
            Direction::Down => _start / i,
        };
        let separator = match direction {
            Direction::Up => "*",
            Direction::Down => "/",
        };
        println!("{:10} {} {} = {}", _start, separator, i, result);
        _start = result;
    }

    result
}

fn main() {
    let intermediate_result: i32 = construct(START, HEIGHT, Direction::Up);
    construct(intermediate_result, HEIGHT, Direction::Down);
}
