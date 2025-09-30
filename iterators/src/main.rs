struct FibonacciIterator {
    curr: u32,
    next: u32,
}

impl Iterator for FibonacciIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        let current = self.curr;
        self.curr = self.next;
        self.next = new_next;

        Some(current)
    }
}

fn main() {
    let numbers = vec![1, 2, 3];
    for next_value in numbers {
        println!("Next value: {}", next_value);
    }

    let mut fib = FibonacciIterator { curr: 0, next: 1 };
    println!("5th element in Fibonacci sequence: {}", fib.nth(4).unwrap());
    let fib = FibonacciIterator { curr: 0, next: 1 };
    for next_value in fib {
        println!("Next Fibonacci value: {}", next_value);
        if next_value > 100 {
            break;
        }
    }
    
    let fib = FibonacciIterator { curr: 0, next: 1 };
    let sum: u32 = fib.take(10).sum();
    println!("Sum of first 10 Fibonacci numbers: {}", sum);

    let numbers: Vec<u16> = (0..1000).collect();
    let numbers = numbers.iter();
    let numbers = numbers.filter(|&x| x % 2 == 0);
    let numbers = numbers.map(|x| x * x);
    for n in numbers {
        println!("Even number squared: {}", n);
    }
}
