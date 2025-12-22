use std::io;

fn main() {
    // Scalar Types
    // integer values range from 8-bit to 128-bit u8(unsigned) i8(signed) isize to check size of signed and usize to check size of unsigned
    let x: u8 = 255;

    println!("value of x: {x}");

    // floating point values by default f64(64-bit) but can also use f32(32-bit)
    let y = 3.0;

    let z: f32 = 2.5;

    println!("Y: {y} Z:{z}");
    
    // numeric operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("Sum:{sum}, difference:{difference}, product:{product}, quotient:{quotient}, truncated:{truncated}, remainder:{remainder}");

    // boolean type
    let t = true;
    
    let f: bool = false;

    // character type
    let c = 'Z';

    let a: char = 'z';
    
    let heart_eyed_cat = '😻';

    // Compund Types
    // tuple type
    let tup: (i32, f64, u8) = (500, 6.9, 3); // anotating the types in optional

    let (x, y, z) = tup; // one way of accessing the tuple elements

    println!("The value or x:{x}, y:{y}, z:{z}");

    let five_hundred = tup.0; // another way to access the tuple elements

    let six_point_four = tup.1;

    let one = tup.2;

    // array type
    let arr = [1, 2, 3, 4, 5]; // declaration of array

    let arr1: [i32; 5] = [1, 2, 3, 4, 5]; // declaration of array with type annotation

    let arr2 = [3; 5]; // declaration of array with element and size(the array would a 5 of 3's [3, 3, 3, 3, 3])

    let first_arr = arr[0]; // accesssing the 1st element of the array arr

    println!("Enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read input");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[index];

    println!("The value of the element at index {index} is: {element}"); // this code runs but will break when the index entered goes out of bounds and throws an error(index out of bounds)

}
