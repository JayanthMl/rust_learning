fn main() {
    let number = 63;

    // if condition
    if number != 0 {
        println!("Number something other than zero");
    }
    println!();

    // if else condition
    if number < 5 {
        println!("condition true");
    } else {
        println!("condition false");
    }
    println!();

    // nested if
    if number % 3 == 0 {
        println!("The number is divisible by 3");
    } else if number % 5 == 0 {
        println!("The number is divisible by 5");
    } else if number % 9 == 0 {
        println!("The number is divisible by 9");
    } else {
        println!("The number is not divisible by 3, 5, 9");
    }
    println!();

    // using if in a statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
    println!();

    // looping using "loop"
    let mut counter = 0;
    
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    println!();

    // labeled loop
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
    println!();

    // while loop
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!");
    println!();

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }
    println!();

    // for loop
    println!("using for loop");
    for element in a {
        println!("The value is: {element}");
    }
    println!();

    // for loop to loop thorugh a range
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!");
}
