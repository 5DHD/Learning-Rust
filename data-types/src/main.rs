/*
Every value in Rust is a data type.
Rust is a statically typed language,
which means that it must know the types of all variables at compile time.
However, Rust can often infer the type of a variable based on the value it is assigned.
*/

fn main() {
    // ----------------Scalar types: Represents a single value. (Ints, Floats, Booleans, Characters)------------

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

    // Integer overflow: When an integer exceeds its maximum or minimum value,
    // it wraps around to the opposite end of the range.
    // A u8 of 255 + 1 will wrap around to 0.

    //--------------------------------------------------------

    // Floating-point types: Represents numbers with a fractional component. Default is f64.
    // All types are signed.
    // f32: 32-bit floating-point number (single precision)
    let g: f32 = 3.14;

    // f64: 64-bit floating-point number (double precision)
    let h: f64 = 3.14159;

    //--------------------------------------------------------

    // Numeric Operations: Rust supports standard arithmetic operations like addition (+), subtraction (-), multiplication (*), division (/), and remainder (%).
    let sum : i32 = 4 + 5; // 4 + 5 = 9
    let difference : i32 = 50 - 25; // 50 - 25 = 25
    let product : i64 = 2 * 100; // 2 * 100 = 200
    let quotient : f64 = 10.6 / 2.0; // 10.6 / 2.0 = 5.3
    let truncated : i32 = -5 / 3; // -5 / 3 = -1 (integer division is truncated towards zero)
    let remainder : i32 = 10 % 3; // 10 % 3 = 1 (remainder of the division)

    // -------------------------------------------------------

    // Boolean type: Represents a value that can be either true or false.
    let is_rust_fun: bool = true;
    let is_rust_easy: bool = false;

    // -------------------------------------------------------

    // Character type: Represents a single Unicode scalar value (a character).
    let letter: char = 'A';
    let emoji: char = '😊';
    // Characters are enclosed in single quotes (' '), while strings are enclosed in double quotes (" ").

    //-----------------Compound types: Can group multiple values into one type. (Tuples, Arrays)----------------
    
    // Tuple: A fixed-size collection of values of different types.
    // Cannot grow or shrink in size after it is created.
    // A tuple can contain any number of values, and each value can be of a different type.
    // The values in a tuple are accessed using dot notation with the index of the value. First index is 0.
    let my_tuple: (i32, f64, char) = (500, 6.4, 'Z'); // Type annotation is optional, Rust can infer the types based on the values assigned.
    let (i, j, k) = my_tuple; // Destructuring the tuple into individual variables
    println!("The value of i is: {i}"); // 500
    
    let first_value = my_tuple.0; // Accessing the first value of the tuple using dot notation
    println!("The first value of the tuple is: {first_value}"); // 500

    //--------------------------------------------------------

    // Array: A fixed-size collection of values of the same type.
    // Arrays have a fixed length, which is determined at compile time and cannot be changed.
    let my_array: [i32; 5] = [1, 2, 3, 4, 5]; // Type annotation is optional, Rust can infer the types based on the values assigned. The square brackets [] indicate that this is an array, and the semicolon separates the type of the elements (i32) from the length of the array (5).
    println!("The first value of the array is: {}", my_array[0]); // 1  
    let second_value = my_array[1]; // Accessing the second value of the array using index notation
    println!("The second value of the array is: {second_value}"); // 2
}
