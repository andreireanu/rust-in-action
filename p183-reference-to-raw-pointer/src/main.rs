
fn main() {use std::mem::size_of;

    let a: i64 = 42;
    let a_ptr = &a as *const i64;
    println!("a: {} ({:p}), size: {:?} bytes", a, a_ptr, size_of::<*const i64>());

}
