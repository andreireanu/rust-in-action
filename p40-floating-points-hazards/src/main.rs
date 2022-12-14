fn main() {

    // assert!( 0.1 + 0.2 == 0.3); -> CRASHES

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("  0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("        0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("  0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("        0.3: {:x}", (xyz.2).to_bits());
    println!();

    let result: f32 = 0.1 + 0.2;
    let desired: f32 = 0.3;
    let absolute_difference = (desired - result).abs();
    assert!(absolute_difference <= f32::EPSILON);

    let x:f32 = 1.0 / 0.0;
    assert!(!x.is_finite());
    
    assert!(abc.0 + abc.1 == abc.2);
    assert!(xyz.0 + xyz.1 == xyz.2); // -> CRASHES
}
