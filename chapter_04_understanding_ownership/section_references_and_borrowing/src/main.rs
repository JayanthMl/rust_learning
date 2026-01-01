fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}");

    let mut s = String::from("change");

    // The below example shows that we only borrow the mutable reference value once, we cannot have multiple reference to that and since we are attempting to create two mutable references it fails
    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{r1}, {r2}");

    let mut s2 = String::from("mutable");

    // We cannot have a mutable reference if we are having a immutable one to the same value
    let r3 = &s2; // no problem as they are immutable references
    let r4 = &s2; // no problem as they are immutable references

    println!("{r3}, {r4}");
    // r3, r4 will not be used after this point

    let r5 = &mut s2; // no problem as the scope of the immutable reference has ended

    println!("{r5}");

    change_mut(&mut s);

    println!("{s}");

    // Dangling References

    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// This cannot be done as we are borrowing a immutable reference
// fn change(some_string: &String) {
//     some_string.push_str("is the only constant");
// }

// This is the correct way to mutate a reference
fn change_mut(some_string: &mut String) {
    some_string.push_str(" is the only constant");
}

fn dangle() -> String {
    let s = String::from("Hello"); // s is a new string

    //&s    // we return a reference to the string s

    // hence we can just return the string, it works because ownership is moved out and nothing is deallocated
    s
} // s goes out of scope here and is dropped, so it's memory goes away