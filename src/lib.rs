#![no_std]
//! # Map-arduino
//! This is an library that is the rewrite of the arduino map function, but for every number type!
//! ## Usage
//! You use it the same way you would use the arduino map, just pick the type you want, in this example, it's u32:
//! ```rust
//! map_u32(value: u32, from_low: u32, from_high: u32, to_low: u32 , to_high: u32);
//! ```
//! However, it can be any standard number type available!
//!
//! Here's an example:
//!
//! ```rust
//! use map_arduino::{map_f32, map_u32, map_i32};
//!
//! let value_f32 = map_f32(5.0, 0.0, 10.0, 0.0, 5.0);
//! println!("f32 mapped value: {}", &value_f32);
//! assert_eq!(value_f32, 2.5);
//!
//! let value_u32 = map_u32(5, 0, 10, 0, 20);
//! println!("u32 mapped value: {}", &value_u32);
//! assert_eq!(value_u32, 10);
//!
//! let value_i32 = map_i32(5, 0, 10, 0, 20);
//! println!("i32 mapped value: {}", &value_i32);
//! assert_eq!(value_i32, 10);
//! ```
mod tests;

pub fn map_f32(val: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn map_f64(val: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    (val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn map_u8(val: u8, in_min: u8, in_max: u8, out_min: u8, out_max: u8) -> u8 {
    (val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn map_u16(val: u16, in_min: u16, in_max: u16, out_min: u16, out_max: u16) -> u16 {
    (val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn map_u32(val: u32, in_min: u32, in_max: u32, out_min: u32, out_max: u32) -> u32 {
    (val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn map_u64(val: u64, in_min: u64, in_max: u64, out_min: u64, out_max: u64) -> u64 {
    (val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn map_u128(val: u128, in_min: u128, in_max: u128, out_min: u128, out_max: u128) -> u128 {
    (val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn map_i8(val: i8, in_min: i8, in_max: i8, out_min: i8, out_max: i8) -> i8 {
    (val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn map_i16(val: i16, in_min: i16, in_max: i16, out_min: i16, out_max: i16) -> i16 {
    (val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn map_i32(val: i32, in_min: i32, in_max: i32, out_min: i32, out_max: i32) -> i32 {
    (val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn map_i64(val: i64, in_min: i64, in_max: i64, out_min: i64, out_max: i64) -> i64 {
    (val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn map_i128(val: i128, in_min: i128, in_max: i128, out_min: i128, out_max: i128) -> i128 {
    (val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}
