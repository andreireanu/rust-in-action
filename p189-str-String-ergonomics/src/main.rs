
fn is_strong(password: String) -> bool {
    password.len() > 5
}

fn is_strong_generic<T: AsRef<str>>(password: T) -> bool {
    password.as_ref().len() > 5
}


fn main() {
    let pw1 = String::from("justok");
    println!("Pw1 with is_strong: {}", is_strong(pw1));
  
    let pw2 = String::from("String");
    let pw3 = "&str";
    println!("Pw2 with is_strong_generic: {}", is_strong_generic(pw2));
    println!("Pw3 with is_strong_generic: {}", is_strong_generic(pw3));

}
