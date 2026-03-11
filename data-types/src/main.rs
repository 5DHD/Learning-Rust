/*
Every value in Rust is a data type.
Rust is a statically typed language,
which means that it must know the types of all variables at compile time.
However, Rust can often infer the type of a variable based on the value it is assigned.
*/

fn main() {
    // Scalar types: Represents a single value.
    // Integer: Numbers without a fractional component.
    // The sign is only necessary for signed integers, while unsigned integers can only represent non-negative values.
    // 8-bit: i8 (signed), u8 (unsigned) | 0-255 
    let a: u8 = 4;
    
    // 16-bit: i16 (signed), u16 (unsigned) | 0-65535
    let b: u16 = 45;

    // 32-bit: i32 (signed), u32 (unsigned) | 0-4294967295
    let c: i32 = -100;

    // 64-bit: i64 (signed), u64 (unsigned) | 0-18446744073709551615
    let d: u64 = 10000000000;

    // 128-bit: i128 (signed), u128 (unsigned) | 0-340282366920938463463374607431768211455
    let e: i128 = -500;

    // isize or usize: Pointer-sized signed/unsigned integer (depends on the architecture)
    let f: usize = 100;

    //--------------------------------------------------------

    
}
