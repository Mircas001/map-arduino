// * thanks Arduino, using your math
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

