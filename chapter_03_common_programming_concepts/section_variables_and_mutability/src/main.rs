fn main() {
    let mut x = 5; // use the *mut* keyword to make a variable immutable.
    
    println!("The value of x is: {x}");
    
    x = 6;
    
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // we use const keyword to make a constant, constants cannot be mutable, they need the type to be specified and remain same throughout it's scope.

    println!("Const value printing: {THREE_HOURS_IN_SECONDS}");

    let y = 5;

    let y = y + 1; // the previous value of y is being shadowed here and it would print 6 and not 5.

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let spaces = "  ";
    let spaces = spaces.len();
    // The above section works because we are shadowing but wouldn't work it we were to mutate it using mut like the example below.

    let mut space = "   ";
    space = space.len(); // This throws a compile time error as the types are different, and we cannot mutate a variable's type in Rust.
}
