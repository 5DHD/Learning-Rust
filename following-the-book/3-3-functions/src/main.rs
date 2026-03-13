// Functions are prevelent in Rust, and they are defined using the `fn` keyword.
// The `main` function is special: it is the entry point of every Rust program. The `main` function is always the first code that runs in a Rust program, and it is where you can start writing your code.
// Rust uses snake_case

fn main() {
    println!("Hello, world!");

    // We can also call other functions by their name followed by parentheses `()`. If the function has parameters, we can pass arguments inside the parentheses.
    another_function();

    // To call a function with parameters, we need to provide the arguments in the same order as the parameters are defined in the function signature.
    func_with_parameters(5, 10);
}

// A function is defined by entering the `fn` keyword, followed by the function name, parentheses `()`, and a body enclosed in curly braces `{}`.
// The function body contains the code that will be executed when the function is called.

fn another_function() {
    println!("Another function.");
}

// Rust does not care where you define your functions, as long as they are defined somewhere in the same scope.
// You can define functions before or after the `main` function, and it will work just fine.

// We can define functions that take parameters.
// Parameters are specified inside the parentheses `()` after the function name, and they consist of a name and a type, separated by a colon `:`.
// You can have as many parameters as you want, and they are separated by commas `,`.

fn func_with_parameters(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}