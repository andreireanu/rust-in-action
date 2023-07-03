fn main() {
    let a = 10; 
    let b: i32 = 20;
    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a,b), add(c,d));
    println!("Result: {}", e);

    // p037 basic operations on numbers
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;

    let addition = twenty + twenty_one + twenty_two;
    println!("Addition: {}", addition);

    let one_mill: i64 = 1_000_000;
    println!("{}", one_mill.pow(2));

}

fn add (i: i32, j:i32) -> i32 {
    i + j
}