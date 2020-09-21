use std::mem::transmute;

const BIAS: i32 = 127;
const BASE: f32 = 2.0;

fn main() {
    let float: f32 = -42.22;
    let (sign_bits, exponent_bits, mantissa_bits) = deconstruct_f32(float);
    let (sign, exponent, mantissa) = decode_f32_parts(sign_bits, exponent_bits, mantissa_bits);
    let reconstructed_float = f32_from_parts(sign, exponent, mantissa);

    println!("{}", char_times('-', 100));
    println!("IEEE 754 DECODING:");
    println!("{}", char_times('-', 100));
    println!(
        "{} parts in bits -> [sign: {:b}, exponent: {:8b}, mantissa: {:023b}]",
        float, sign_bits, exponent_bits, mantissa_bits
    );
    println!(
        "{} parts in decimal -> [sign: {}, exponent: {}, mantissa: {}]",
        float, sign, exponent, mantissa
    );
    println!("Reconstructed float got us {}", reconstructed_float);
    println!("{}", char_times('-', 100));
}

fn deconstruct_f32(n: f32) -> (u32, u32, u32) {
    let n_: u32 = unsafe { transmute(n) };
    let sign: u32 = (n_ >> 31) & 1;
    let exponent: u32 = (n_ >> 23) & 0xff;
    let mantissa: u32 = 0b_0000000_01111111_11111111_11111111 & n_;

    (sign, exponent, mantissa)
}

fn decode_f32_parts(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    let signed_1 = (-1.0_f32).powf(sign as f32);

    let exponent = (exponent as i32) - BIAS;
    let exponent = BASE.powf(exponent as f32);

    let mut mantissa: f32 = 1.0;
    for i in 0..23_u32 {
        let one_at_bit_i = 1 << i;
        if (one_at_bit_i & fraction) != 0 {
            mantissa += 2_f32.powf((i as f32) - 23.0);
        }
    }

    (signed_1, exponent, mantissa)
}

fn f32_from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}

fn char_times(c: char, n: u32) -> String {
    (0..n).map(|_| c).collect::<String>()
}
