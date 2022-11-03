use std::ops::{Add};
use std::time::{Duration};

fn main() {
    let x = 10;
    let y = 10;
    println!("Add with lifetime x + y = {}", add_with_lifetimes(&x, &y));
    println!("Generic add x + y = {}", add(x, y));

    let floats = add(1.2, 3.4);
    let ints = add(10, 20);
    let durations = add(
        Duration::new(5,0),
        Duration::new(10,0),
    );
    println!("{}", floats);
    println!("{}", ints);
    println!("{:?}", durations);
}


fn add_with_lifetimes<'a, 'b>(x: &'a i32, y: &'b i32) -> i32 {
    x + y
}

fn add<T: Add<Output=T>>(i: T, j: T) -> T{
    i + j
}