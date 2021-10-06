#![allow(unused)]

use std::cmp::PartialOrd;

fn main() {
    // Generic struct
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // not valid:
    // let float = Point { x: 1.0, y: 4 };

    let integer_and_float = Point2 { x: 5, y: 4.0 };

    // Calling typed methods
    let p = Point { x: 5.2, y: 2.3 };
    println!("p.x = {}", p.x());
    println!("p.distance = {}", p.distance_from_origin());

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Implement a method only on a specific type
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


struct Point2<T, U> {
    x: T,
    y: U,
}

// fn largest(list: &[impl PartialOrd]) -> impl PartialOrd {
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// TODO: Implement with reference
// fn largest2<T: PartialOrd>(list: &[T]) -> &T {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }


