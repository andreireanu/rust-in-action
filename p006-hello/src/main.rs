fn green_world() {
    println!("Hello, world");
    let southern_germany = "Grüß Gott!"; // variable binding
    let japan = "ハロー・ワールド"; // variable binding
    let regions = [southern_germany, japan];
    for region in regions.iter() {
        println!("{}", &region); // ampersand “borrows”
    }
}

fn main() {
    green_world();
}
