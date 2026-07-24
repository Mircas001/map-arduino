use map_arduino::{map_f32, map_i32, map_u32};

// * This example uses println, but you get the idea

fn main() {
    let value_f32 = map_f32(5.0, 0.0, 10.0, 0.0, 5.0);
    println!("f32 mapped value: {}", &value_f32);
    assert_eq!(value_f32, 2.5);

    let value_u32 = map_u32(5, 0, 10, 0, 20);
    println!("u32 mapped value: {}", &value_u32);
    assert_eq!(value_u32, 10);

    let value_i32 = map_i32(5, 0, 10, 0, 20);
    println!("i32 mapped value: {}", &value_i32);
    assert_eq!(value_i32, 10);
}
