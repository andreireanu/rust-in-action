fn main() {

    let n: f32 = -42.42;
    let n_bits = n.to_bits();
    
    let sign_bit = n_bits >> 31;
    println!("Isolated sign bit (1 bit): {}", sign_bit);

    let exponent = n_bits >> 23 & 0xff ;
    println!("Isolated exponent bits (23 bit): {}", exponent);

    let mantissa = n_bits & & 0x7fffff;
    println!("Isolated mantissa bits (8 bit): {}", mantissa);


}
