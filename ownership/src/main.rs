#![allow(dead_code, unused_variables)]

struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    // let outer_vector;
    {
        let vector_of_points = vec![
            Point { x: 1, y: 2 },
            Point { x: 3, y: 4 },
            Point { x: 5, y: 6 },
        ];
        println!(
            "The 2nd point is at ({}, {})",
            vector_of_points[1].x, vector_of_points[1].y
        );

        let mut another_vector_of_points = vector_of_points;
        println!(
            "The 2nd point is at ({}, {})",
            another_vector_of_points[1].x, another_vector_of_points[1].y
        );

        let even_another_vector_of_points = &another_vector_of_points;
        println!(
            "The 2nd point is at ({}, {})",
            another_vector_of_points[1].x, another_vector_of_points[1].y
        );
        println!(
            "The 2nd point is at ({}, {})",
            even_another_vector_of_points[1].x, even_another_vector_of_points[1].y
        );

        // Does not work
        //outer_vector = &another_vector_of_points;

        let mutable_borrow = &mut another_vector_of_points;
    }

    {
        // Does not work
        //println!("{}", outer_vector[1].x);

        let p1 = Point { x: 10, y: 20 };
        let p2 = p1;
        // I can no longer access p1

        consume(p2);
        // I can no longer access p2

        let mut p1 = create();
        let borrowed_p1 = &p1;
        print(&p1);
        print(borrowed_p1);

        let mut_borrowed_p1 = &mut p1;
        double(&mut p1);
    }

    {
        let mut numbers = vec![1, 2, 3, 4, 5];
        println!("{:p}", &numbers);
        println!("{:p}", &numbers[0]);
        for n in &mut numbers {
            *n *= 2;
            // Print the memory address of n
            println!("{:p}", n);
        }
        for n in &numbers {
            println!("{}", n);
            println!("{:p}", n);
        }
        for n in numbers {
            println!("{}", n);
            println!("{:p}", &n);
        }
    }

    {
        let points = vec![
            Point { x: 1, y: 2 },
            Point { x: 3, y: 4 },
            Point { x: 5, y: 6 },
        ];

        let p1 = &points[1];
        
        drop(points);
    }

    {
        let p1 = Point3D { x: 1, y: 2, z: 3 };
        let p2 = p1; // NOT a transfer of ownership, because Point3D is Copy
    }
}

fn consume(p: Point) {
    println!("Point is at ({}, {})", p.x, p.y);
}

fn create() -> Point {
    let p = Point { x: 100, y: 200 };
    p
}

fn print(p: &Point) {
    println!("Point is at ({}, {})", p.x, p.y);
}

fn double(p: &mut Point) {
    p.x *= 2;
    p.y *= 2;
}
