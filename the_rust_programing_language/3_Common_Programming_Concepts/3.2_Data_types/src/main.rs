
fn main() {
    // Integer Type
    let x_s_i8: i8 = -128;
    let x_e_i8: i8 = 127;
    println!("range in i8 type: {} ~ {}", x_s_i8, x_e_i8);

    let x_s_i16: i16 = -32768;
    let x_e_i16: i16 = 32767;
    println!("range in i16 type: {} ~ {}", x_s_i16, x_e_i16);

    let x_s_i32: i32 = -2147483648;
    let x_e_i32: i32 = 2147483647;
    println!("range in i32 type: {} ~ {}", x_s_i32, x_e_i32);

    let x_s_i64: i64 = -9223372036854775808;
    let x_e_i64: i64 = 9223372036854775807;
    println!("range in i64 type: {} ~ {}", x_s_i64, x_e_i64);

    let x_s_i128: i128 = -170141183460469231731687303715884105728;
    let x_e_i128: i128 = 170141183460469231731687303715884105727;
    println!("range in i128 type: {} ~ {}", x_s_i128, x_e_i128);

    let x_s_isize: isize = -9223372036854775808;
    let x_e_isize: isize = 9223372036854775807;

    println!("range in isize type: {} ~ {}", x_s_isize, x_e_isize);

    let x_s_u8: u8 = 0;
    let x_e_u8: u8 = 255;
    println!("range in u8 type: {} ~ {}", x_s_u8, x_e_u8);

    let x_s_u16: u16 = 0;
    let x_e_u16: u16 = 65535;
    println!("range in u16 type: {} ~ {}", x_s_u16, x_e_u16);

    let x_s_u32: u32 = 0;
    let x_e_u32: u32 = 4294967295;
    println!("range in u32 type: {} ~ {}", x_s_u32, x_e_u32);

    let x_s_u64: u64 = 0;
    let x_e_u64: u64 = 18446744073709551615;
    println!("range in u64 type: {} ~ {}", x_s_u64, x_e_u64);

    let x_s_u128: u128 = 0;
    let x_e_u128: u128 = 340282366920938463463374607431768211455;
    println!("range in u128 type: {} ~ {}", x_s_u128, x_e_u128);

    let x_s_usize: usize = 0;
    let x_e_usize: usize = 18446744073709551615;
    println!("range in usize type: {} ~ {}", x_s_usize, x_e_usize);

    // Floating-Point Type
    let x_f32: f32 = 3.14;
    let x_f64: f64 = 3.14;
    println!("f32: {}, f64: {}", x_f32, x_f64);

    // Numeric Operations
    let sum = 5 + 10;
    let diff = 95.5 - 4.3;
    let prod = 4 * 30;
    let quot = 56.7 / 32.2;
    let rem = 43 % 5;
    println!("sum: {}, diff: {}, prod: {}, quot: {}, rem: {}", sum, diff, prod, quot, rem);

    // Boolean Type
    let t = true;
    let f: bool = false;
    println!("t: {}, f: {}", t, f);

    // Character Type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);

    // Compound Types
    // Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup: {:?}", tup);
    println!("tup.0: {}, tup.1: {}, tup.2: {}", tup.0, tup.1, tup.2);

    // Array Type
    let arr = [1, 2, 3, 4, 5];
    println!("arr: {:?}", arr);
    println!("arr[0]: {}, arr[1]: {}, arr[2]: {}", arr[0], arr[1], arr[2]);
}