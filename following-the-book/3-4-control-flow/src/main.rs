// Control flow is the ability to run code conditionally or repeatedly.
//Rust has the usual control flow constructs you find in most languages, such as if expressions and loops.

fn main() {

    //------------------If Expressions------------------
    // The if expression is the most basic form of control flow in Rust. It allows you to execute code based on a condition.
    // If expressions start with the keyword if, followed by a condition, and then a block of code in curly braces. The condition must evaluate to a boolean value (true or false).
    // The else arm is optional, but it allows you to specify code that should run if the condition is false.
    // If not else is present and the condition is false, then rust will simply skip the block of code and continue executing the rest of the program.
    let number = 4;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    // You can also chain multiple conditions together using else if.
    let number_2 = 6;
    if number_2 < 5 {
        println!("number_2 was true");
    } else if number_2 == 6 {
        println!("number_2 was {number_2}");
    } else {
        println!("number_2 was false");
    }

    // Because if is an expression, you can use it on the right-hand side of a let statement to assign the result to a variable.
    let condition = true;
    let number_3 = if condition { 5 } else { 6 };
    println!("The value of number_3 is: {number_3}");
}
