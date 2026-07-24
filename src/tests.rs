#[cfg(test)]
use crate::*;

use super::*;

#[test]
fn check_f32_whole() {
    let result = map_f32(25.0, 0.0, 50.0, 0.0, 100.0);
    assert_eq!(result, 50.0);
}
#[test]
fn check_f64_whole() {
    let result = map_f64(25.0, 0.0, 50.0, 0.0, 100.0);
    assert_eq!(result, 50.0);
}

#[test]
fn check_f32_half() {
    let result = map_f32(25.0, 0.0, 50.0, 0.0, 25.0);
    assert_eq!(result, 12.5);
}
#[test]
fn check_f64_half() {
    let result = map_f64(25.0, 0.0, 50.0, 0.0, 25.0);
    assert_eq!(result, 12.5);
}

#[test]
fn check_u8() {
    let result = map_u8(5, 0, 10, 0, 20);
    assert_eq!(result, 10);
}
#[test]
fn check_u16() {
    let result = map_u16(25, 0, 50, 0, 100);
    assert_eq!(result, 50);
}
#[test]
fn check_u32() {
    let result = map_u32(25, 0, 50, 0, 100);
    assert_eq!(result, 50);
}
#[test]
fn check_u64() {
    let result = map_u64(25, 0, 50, 0, 100);
    assert_eq!(result, 50);
}
#[test]
fn check_u128() {
    let result = map_u128(25, 0, 50, 0, 100);
    assert_eq!(result, 50);
}

#[test]
fn check_i8_positive() {
    let result = map_i8(5, 0, 10, 0, 20);
    assert_eq!(result, 10);
}
#[test]
fn check_i16_positive() {
    let result = map_i16(25, 0, 50, 0, 100);
    assert_eq!(result, 50);
}
#[test]
fn check_i32_positive() {
    let result = map_i32(25, 0, 50, 0, 100);
    assert_eq!(result, 50);
}
#[test]
fn check_i64_positive() {
    let result = map_i64(25, 0, 50, 0, 100);
    assert_eq!(result, 50);
}
#[test]
fn check_i128_positive() {
    let result = map_i128(25, 0, 50, 0, 100);
    assert_eq!(result, 50);
}

#[test]
fn check_i8_negative() {
    let result = map_i8(-5, 0, -10, 0, -20);
    assert_eq!(result, -10);
}
#[test]
fn check_i16_negative() {
    let result = map_i16(-25, 0, -50, 0, -100);
    assert_eq!(result, -50);
}
#[test]
fn check_i32_negative() {
    let result = map_i32(-25, 0, -50, 0, -100);
    assert_eq!(result, -50);
}
#[test]
fn check_i64_negative() {
    let result = map_i64(-25, 0, -50, 0, -100);
    assert_eq!(result, -50);
}
#[test]
fn check_i128_negative() {
    let result = map_i128(-25, 0, -50, 0, -100);
    assert_eq!(result, -50);
}

#[test]
fn check_i8_integer() {
    let result = map_i8(3, -5, 5, 0, 10);
    assert_eq!(result, 8);
}
#[test]
fn check_i16_integer() {
    let result = map_i16(3, -5, 5, 0, 10);
    assert_eq!(result, 8);
}
#[test]
fn check_i32_integer() {
    let result = map_i32(3, -5, 5, 0, 10);
    assert_eq!(result, 8);
}
#[test]
fn check_i64_integer() {
    let result = map_i64(3, -5, 5, 0, 10);
    assert_eq!(result, 8);
}
#[test]
fn check_i128_integer() {
    let result = map_i128(3, -5, 5, 0, 10);
    assert_eq!(result, 8);
}
