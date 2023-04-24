use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    let a = 10; // integer on Stack
    let b = Box::new(20); // integer on Heap
    let c = Rc::new(Box::new(30)); // Reference counter Boxed integer
    let d = Arc::new(Mutex::new(40)); // Atomic reference counter
    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d);
}
