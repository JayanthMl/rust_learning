// Convert temperatures between Fahrenheit and Celsius.
// Generate the nth Fibonacci number.
// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

use std::io;

fn read_i32() -> i32 {
    loop {
        let mut input_str = String::new();

        io::stdin()
            .read_line(&mut input_str)
            .expect("Failed to read line");

        match input_str.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid input: enter number");
            }
        };
    };
}

fn celsius_to_fahrenheit(num: f64) -> f64 {
    num * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(num: f64) -> f64 {
    (num - 32.0) * 5.0 / 9.0
}

fn read_f64() -> f64 {
    loop {
        let mut input_str = String::new();
        io::stdin()
            .read_line(&mut input_str)
            .expect("Failed to read line");

        match input_str.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid number, try again");
        }
    }
}

fn temperature_conversion() {
    // Celsius to Fahrenheit: F = C(9/5) + 32
    // Fahrenheit to Celsius: C = (F-32) (5/9)
    println!("Press 1 to convert celsius to fahrenheit");
    println!("Press 2 to convert fahrenheit to celsius");
    
    let input = read_i32();

    match input {
        1 => {
            println!("Enter temperature in celsius:");
            let temperature = read_f64();
            let result = celsius_to_fahrenheit(temperature);
            println!("Temperature in Fahrenheit: {result}");
        }
        2 => {
            println!("Enter temperature in Fahrenheit:");
            let temperature = read_f64();
            let result = fahrenheit_to_celsius(temperature);
            println!("Temperature in celsius: {result}");
        }
        _ => {
            println!("Enter 1 or 2");
        }
    }
    
}

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    
    let mut first = 0;
    let mut second = 1;
    
    for _ in 2..=n {
        let temp = first + second;
        first = second;
        second = temp;
    }
    second
}

fn fibonacci_number() {
    println!("Enter the number");

    let input = read_i32();
    
    if input < 0 {
        println!("Please enter a non-negative number");
        return;
    }

    let result = fibonacci(input);
    println!("Fibonacci number at position {input} is: {result}");

}

fn lyrics_print() {
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];

    for day in 0..12 {
        println!("On the {} day of christmas, my true love gave me", days[day]);

        for gift in (0..=day).rev() {
            if day != 0 && gift == 0 {
                println!("and {}", gifts[gift]);
            } else {
                println!("{}", gifts[gift]);
            }
        }
        println!();
    }
}

fn main() {
    println!("Enter   1 for temperature conversion\n \t2 for generating nth Fibonacci number\n \t3 for printing the lyrics to the chirstmas carol");
    
    let input = read_i32();

    const TEMP: i32 = 1;
    const FIB: i32 = 2;
    const LYRICS: i32 = 3;

    match input {
        TEMP => temperature_conversion(),
        FIB => fibonacci_number(),
        LYRICS => lyrics_print(),
        _ => println!("Enter 1, 2, 3"),
    }

}
