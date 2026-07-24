# Map-arduino
This is an library that is the rewrite of the arduino map function, but for every number type! 

## Features:
- Arduino-style Map function for every standard number type avaialable in rust!
- Automated tests to see if the math is correct!
- `no-std` compatible! (So your transition from the Arduino IDE is easier!)
- Will work forever, probably!

## Installation
Not published in crates.io yet.              
~~Add it via cargo:~~
```bash
cargo add map-arduino
```

## Usage
You can use it to re-map from a range to another range, and the syntax is the same as arduino, which is roughly as follows:
```rust
map(value, from_low, from_high, to_low, to_high)
```

Here's an example:

```rust
use map_arduino::{map_f32, map_u32, map_i32};

let value_f32 = map_f32(5.0, 0.0, 10.0, 0.0, 5.0);
println!("f32 mapped value: {}", &value_f32);
assert_eq!(value_f32, 2.5);

let value_u32 = map_u32(5, 0, 10, 0, 20);
println!("u32 mapped value: {}", &value_u32);
assert_eq!(value_u32, 10);

let value_i32 = map_i32(5, 0, 10, 0, 20);
println!("i32 mapped value: {}", &value_i32);
assert_eq!(value_i32, 10);
```

## Credits 
[Arduino](https://docs.arduino.cc/language-reference/en/functions/math/map/)