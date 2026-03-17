/*
Ownership is a set of rules that govern how Rust manages memory during compilation.
Some languages have garbage collection, which automatically frees memory that is no longer needed.
Rust takes a different approach by using ownership, which is a set of rules that the compiler checks at compile time.


The Stack vs the Heap
The Stack: The stack is a data structure that stores values in order, with the last value added being the first one to be removed (last-in, first-out or LIFO). All data stored on the stack must have a known, fixed size at compile time. The stack is very fast and efficient, but it has limited capacity and can lead to stack overflow if too much data is stored.
The Heap: The heap is a data structure that allows for more flexible memory management, but requires explicit allocation and deallocation. Data stored on the heap can have a variable size, and the heap can grow or shrink as needed. However, accessing data on the heap is slower than accessing data on the stack, and it can lead to memory leaks if not managed properly.

Ownership helps with memory management on the heap by ensuring that there is a single owner for each piece of data, and that the data is automatically deallocated when the owner goes out of scope. This allows Rust to manage memory safely and efficiently without the need for a garbage collector.

Ownership Rules:
1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

TLDR: Ownership is a set of rules that govern how Rust manages memory during compilation.
It allows Rust to manage memory safely and efficiently without the need for a garbage collector,
by ensuring that there is a single owner for each piece of data, and that the data is automatically deallocated when the owner goes out of scope.
*/

fn main() {
    // ------------- Variable Scope -------------
    let l = "hello"; // l is valid from this point forward
    println!("{l}");

    {
        let t = "hello"; // t is valid from this point forward
        println!("{t}");
    } // t goes out of scope here

    // The string type is an unknown size at compile time, so it is stored on the heap.
    // The previous number types were stored on the stack because they have a known size at compile time.
    // The string type is stored on the heap because it can grow or shrink as needed, and it is accessed through a pointer on the stack.

    // String literals are stored on the stack, but they are immutable and have a fixed size at compile time.
    // Here we are creating a String type from a string literal, which is stored on the heap, and we can modify it because it is mutable.
    let mut s1 = String::from("hello");
    s1.push_str(", world!"); // push_str() appends a literal to a String. It takes a string slice as a parameter, which is a reference to a string.
    println!("{s1}");

    // Other systems programming languages like C and C++ require manual memory management by freeing memory in the code, which can lead to bugs and security vulnerabilities if not done correctly.
    //Rust's ownership system automatically manages memory, ensuring that it is freed when it is no longer needed, without the need for a garbage collector or manual memory management.

    {
        let s2 = String::from("hello"); // s2 is valid from this point forward
        // do stuff with s2
        println!("{s2}");
    } // s2 goes out of scope here and the memory is automatically freed

    // ------------- Variables and Data Interactions -------------

    let x = 5;
    let y = x; 
    // x is copied to y, because integers are stored on the stack and have a known size at compile time

    // Now the string version of the above code:
    let s3 = String::from("hello");
    let s4 = s3;
    // The string is stored in the heap and the variable holds the pointer to memory in the heap, the length and the capacity, all of which are stored on the stack.
    // When we assign s3 to s4, we are copying the pointer, the length and the capacity, but not the data on the heap.

    // To ensure memory safety, Rust will consider s3 as no longer valid after the assignment, and it will not allow us to use s3 anymore.
    // This is called a move, and it prevents us from having two variables that point to the same data on the heap, which could lead to double free errors if both variables were to go out of scope and try to free the same memory.
    // s3 was "moved" to s4, and s3 is no longer valid, so we cannot use s3 anymore.

    // ------------- Scope and Assignment -------------
    let mut s5 = String::from("hello");
    s5 = String::from("ahoy");

    println!("{s5}, world!");

    // When you assign a new value to an existing variable, the old value is dropped and the new value is assigned to the variable. In this case, the old value "hello" is dropped and the new value "ahoy" is assigned to s5.
    // The memory for the old value is automatically freed when it goes out of scope, which happens when we assign a new value to s5.

    // ------------- Variables and Data Ineracting with Clone -------------
    // If we do want to copy the heap data along with the pointer, length and capacity, we can use the clone method, which creates a deep copy of the data on the heap.
    // This can be expensive in terms of performance, so it should be used judiciously.
    let s6 = String::from("hello");
    let s7 = s6.clone();
    println!("s6 = {s6}, s7 = {s7}");

    // ------------- Stack-Only Data: Copy -------------
    // This code is valid even though we are copying the value of a, because integers are stored on the stack and have a known size at compile time, so they implement the Copy trait, which allows for a bitwise copy of the value.
    let a  = 5;
    let b = a;
    println!("a = {a}, b = {b}");

    // Rust has a special annotation called the Copy trait, which can be applied to types that have a known size at compile time and can be copied by simply copying bits. Types that implement the Copy trait do not require a call to the clone method to create a copy of the value, and they do not have ownership semantics like heap-allocated data.
    // Copy cannot be used on types that have the Drop trait, which means that they have a destructor that needs to be called when the value goes out of scope. This is because the destructor may need to free resources or perform other cleanup tasks, and it cannot be safely called on a value that has been copied by simply copying bits.

    // What types have the Copy trait?
    // Integer types (u32), floating-point types (f64), the char type, the bool type, and tuples that only contain types that implement the Copy trait.

    // ------------- Ownership and Functions -------------
    // Passing a value to a function is similar to assigning it to a new variable.
    // Passing a variable to a function will move the ownership of the value to the function, and the variable will no longer be valid after the function call.
    // In some cases it may only copy the value, such as with integers, which implement the Copy trait.
   
    let s8 = String::from("hello"); // s8 comes into scope
    takes_ownership(s8); // s8's value moves into the function and is no longer valid here

    fn takes_ownership(some_string: String) {
    println!("{some_string}");
    } // some_string goes out of scope and is dropped here

    // -------------- Ownership and Functions with Return Values -------------
    // Returning values from a function will also transfer ownership of the value back to the caller.
    
    let s9 = gives_ownership();        // gives_ownership moves its return value into s9

    let s10 = String::from("hello");    // s10 comes into scope

    let s11 = takes_and_gives_back(s10); // s10 is moved into takes_and_gives_back, which also moves its return value into s11  

    fn gives_ownership() -> String {   // gives_ownership will move its return value into the function that calls it

        let some_string = String::from("yours"); // some_string comes into scope

        some_string // some_string is returned and moves out to the calling function
    }

    // This function takes a String and returns a String.
    fn takes_and_gives_back(a_string: String) -> String {
        // a_string comes into scope

        a_string  // a_string is returned and moves out to the calling function
    }

    // ------------------REFERENCES AND BORROWING------------------
    // References allow you to refer to some value without taking ownership of it. This is called borrowing.
    // References are immutable by default, but you can make them mutable if you need to change the value they refer to.
    // Mutable references have some restrictions to ensure that there can only be one mutable reference to a value at a time, and that there cannot be any immutable references to the same value while a mutable reference exists.

    let s12 = String::from("Hallo");

    // & is used to create a reference to a value, which allows us to borrow the value without taking ownership of it.
    let len = calculate_length(&s12); // We pass a reference to s12

    // Because s12 still owns the value, the value will not be dropped when the function ends, so we can use s12 after the function call.

    println!("The length of '{s12}' is {len}.");

    fn calculate_length(s: &String) -> usize {
        s.len()
    } // s goes out of scope here, but because it does not have ownership of what it refers to, it is not dropped.

    // Mutable references allow us to change the value that we are borrowing.
    let mut s13 = String::from("Guten Morgen");

    change(&mut s13); // We pass a mutable reference to s13

    println!("{s13}");

    fn change(some_string: &mut String) {
        some_string.push_str(", Welt!");
    }

    // NOTE: YOU CAN ONLY HAVE ONE MUTABLE REFERENCE TO A VALUE IN A SCOPE.
    /*
    This code will not compile because we are trying to create two mutable references to the same value,
    which is not allowed in Rust because it can lead to data races if both references were to be used at the same time.
    Rust's ownership system ensures that there can only be one mutable reference to a value at a time, and that there cannot be any immutable references to the same value while a mutable reference exists.
    
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{r1}, {r2}");
     */

    // However, we can use scope to allow for multiple mutable references to the same value, as long as they are not used at the same time.
     let mut s14 = String::from("hello");

    {
        let r1 = &mut s14;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s14;
} 

// cheeseburger y fries