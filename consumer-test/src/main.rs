use rust_simd::{
    vector_add,
    vector_add_zig,
    vector_add_zig_simd,
};

fn main() {
    let a = [
        1.0f32, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
    ];

    let b = [
        10.0f32, 20.0, 30.0, 40.0,
        50.0, 60.0, 70.0, 80.0,
    ];

    let expected = [
        11.0f32, 22.0, 33.0, 44.0,
        55.0, 66.0, 77.0, 88.0,
    ];

    let mut rust = [0.0f32; 8];
    let mut zig_scalar = [0.0f32; 8];
    let mut zig_simd = [0.0f32; 8];

    vector_add(&a, &b, &mut rust);
    vector_add_zig(&a, &b, &mut zig_scalar);
    vector_add_zig_simd(&a, &b, &mut zig_simd);

    assert_eq!(rust, expected);
    assert_eq!(zig_scalar, expected);
    assert_eq!(zig_simd, expected);

    println!("external consumer test: PASS");
}
