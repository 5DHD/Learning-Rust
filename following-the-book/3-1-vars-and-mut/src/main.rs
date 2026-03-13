// constants can be declared in any scope
// constants are different form immutable varriables
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // mut must be present to allow a var to be changed
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // the second declaration of a var overshadows the first
    let y = 5;

    let y = y + 1;

    {
        // this is a new scope, so we can declare a new var with the same name
        // this is called shadowing so this scopes y is different from the outer y
        let y = y * 2;
        println!("In the inner scope y is {y}");
    }

    println!("The value of y is {y}");

    // shadowing also allows us to change the type of a var
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is {spaces}");

}
