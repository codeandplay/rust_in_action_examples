use std::mem;

// in decimal: one hundred twenty three.
// if written in big endian: 123. most significant bit on the left.
// if written in little endian: 321, most significant bit on the right.

fn main() {
    let big_endian: [u8; 4] = [0xaa, 0xbb, 0xcc, 0xdd];
    let little_endian: [u8; 4] = [0xdd, 0xcc, 0xbb, 0xaa];

    let a: i32 = unsafe { mem::transmute(big_endian) };
    let b: i32 = unsafe { mem::transmute(little_endian) };

    println!("{} vs {}", a, b);

    // if big endian: 3, if little endian: 192.
    let c: u8 = 0b0000_0011;
    println!("c: {}", c);
}
