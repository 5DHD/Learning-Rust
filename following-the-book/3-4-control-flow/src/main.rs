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
    // Both arms must have the same variable type, otherwise you will get a type mismatch error.
    let condition = true;
    let number_3 = if condition { 5 } else { 6 };
    println!("The value of number_3 is: {number_3}");

    //-------------------Loops------------------
    // Rust has three kinds of loops: loop, while, and for.
    // The loop keyword creates an infinite loop. You can use the break keyword to exit the loop when a certain condition is met.
    loop {
        println!("This is an infinite loop.");
        break;
    }

    // You can return a value from a loop by using break with an expression. The value of the expression will be returned as the result of the loop.
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {result}");

    // You can optionally label loops to disambiguate between multiple loops. This is done by writing a single quote followed by the label name before the loop keyword.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // The while loop runs while a condition is true. It’s often used when the number of iterations is not known and the loop needs to run until some condition changes.
    let mut number_4 = 3;
    while number_4 != 0 {
        println!("{number_4}!");
        number_4 -= 1;
    }
    println!("LIFTOFF!!!");

    // The for loop is used to iterate over a collection of items. It’s often used when the number of iterations is known or when you want to iterate over a range of values.
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
    
    // You can also use a for loop to iterate over a range of values using the .. syntax. The range 1..4 will generate the numbers 1, 2, and 3 (but not 4).
    for num in 1..4 {
        println!("the value is: {num}");
    }
}

