// * thanks Arduino, using your math
#![no_std]

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

#[cfg(test)]
mod math_tests {
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
}
