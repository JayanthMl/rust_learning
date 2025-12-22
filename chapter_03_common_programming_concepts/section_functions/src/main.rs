fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value} {unit_label}");
}

fn main() {
    another_function(5);

    let y = 6; // statement

    // let x = (let y =6); // invalid because statements do no return values hence let statement cannot be assigned to another variable

    let x = {
        let y = 3;
        y + 1 // note the line doesn't end with a semicolon because this is an expression, if the semicolon were to be present it would be a statement and not an expression
    };

    print_labeled_measurements(5, 'j');
    
    let x = five();

    println!("The value of x is {x}");

    let a = plus_one(4);

    println!("The value of a is {a}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}


fn five() -> i32 {
    5
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}