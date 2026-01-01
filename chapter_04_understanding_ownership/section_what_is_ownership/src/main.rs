fn main() {
    let str = String::from("hello"); // create a string literal using the from function.
    
    println!("{str}");

    let mut s = String::from("hi");

    s.push_str(", world!");
    
    println!("{s}");

    let string = String::from("hello");
    let string1 = string;

    // The below is not possible as it's moved to str1, when str goes out of scope it doesn't free memory as it's moved(shallow copied) to str1 this ensures memory safety and there won't be two memory frees when str and str1 go out of scope but it'd be freed only when str1 goes out of scope as it's moved to str1 when we did str1 = str and str1 would free the memory when it goes out of scope.

    // println!("{string}, world!");

    let mut str  = String::from("hello");
    // The string `str` would drop the value `hello` as it is assigned to `ahoy` freeing memory assigned initially and things would be updated to the new value of `ahoy`.
    str = String::from("ahoy");
    
    println!("{str}, world");

    // The `clone` method does a deep copy meaning we are not just creating a pointer to the heap memory like we did before where we have the two variables on the the stack pointing to the same heap memory but this time we have the two variables s1 and s2 having memory allocated from on the heap.
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    // Doing the same does not matter on types such as integers which have a known size at compile time, are stored on the stack hence no differnence between deep and shallow copy
    let x = 6;
    let y = x;

    println!("x = {x}, y = {y}");

    // Ownership and Functions
    let s = String::from("hello");

    takes_ownership(s);
    
    let x = 5;

    makes_copy(x);

    // Return Values and scopes
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
    
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string =  String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s:String) -> (String, usize) {
    let length = s.len();

    (s, length)
}