
fn add<T, U>(a: T, b:T) -> U 
where 
    T: std::ops::Add<Output = T>,
    U:std::convert::From<T>
{
    U::from(a + b) 
}

fn main() {
    let x  = add::<f32, f64>(4f32, 43f32);
    println!("{}", x);
}
