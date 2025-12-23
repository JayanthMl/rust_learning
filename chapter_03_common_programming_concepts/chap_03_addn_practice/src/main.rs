// Convert temperatures between Fahrenheit and Celsius.
// Generate the nth Fibonacci number.
// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

use std::io;

fn main() {
    println!("Enter   1 for temperature conversion\n \t2 for generating nth Fibonacci number\n \t3 for printing the lyrics to the chirstmas carol");
    
    let input: i32 = loop {
        let mut input_str = String::new();
        
        io::stdin()
            .read_line(&mut input_str)
            .expect("Failed to read line");
    
        match input_str.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Invalid input: enter number");
                continue;
            }
        };
    };

    if input == 1 {
        temperature_conversion();
    } else if input == 2 {
        fibonacci_number();
    } else if input == 3 {
        lyrics_print();
    } else {
        println!("Enter 1, 2, 3 ashte ");
    }
}

fn temperature_conversion() {
    // Celsius to Fahrenheit: F = C(9/5) + 32
    // Fahrenheit to Celcius: C = (F-32) (5/9)
    println!("Press 1 to convert celcius to fahrenheit");
    println!("Press 2 to convert fahrenheit to celcius");
    
    let input: i32 = loop {
        let mut input_str = String::new();
        
        io::stdin()
            .read_line(&mut input_str)
            .expect("Failed to read line");
    
        match input_str.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Invalid input: enter number");
                continue;
            }
        };
    };

    if input == 1 {
        let mut temperature_celcius = String::new();

        println!("Enter the temperature in celcius");

        io::stdin()
            .read_line(&mut temperature_celcius)
            .expect("Failed to read input");
        
        let temperature_celcius: f64 = temperature_celcius.trim().parse().expect("Please type a number");

        let fahrenheit_temperature = temperature_celcius * ( 9.0 / 5.0 ) + 32.0;

        println!("Temperature in Fahrenheit is:{fahrenheit_temperature}");

    } else if input == 2 {
        let mut temperature_fahrenheit = String::new();

        println!("Enter the temperature in fahrenheit");

        io::stdin()
            .read_line(&mut temperature_fahrenheit)
            .expect("Failed to read input");
        
        let temperature_fahrenheit: f64 = temperature_fahrenheit.trim().parse().expect("Please type a number");

        let celcius_temperature = ( temperature_fahrenheit - 32.0 ) * ( 5.0 / 9.0 );

        println!("Temperature in Celcius is:{celcius_temperature}");

    } else {
        println!("Enter 1, 2 ashte");
    }
    
}

fn fibonacci_number() {
    println!("Enter the number");

    let input: i32 = loop {
        let mut input_str = String::new();
        
        io::stdin()
            .read_line(&mut input_str)
            .expect("Failed to read line");
    
        match input_str.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Invalid input: enter number");
                continue;
            }
        };
    };
    
    let mut first = 0;
    let mut second = 1;

    for _ in 2..=input {
        let temp = first + second;
        first = second;
        second = temp;
    }

    if input == 0 {
        println!("Fibonacci number at position {} is: {}", input, first);
    } else {
        println!("Fibonacci number at position {} is: {}", input, second);
    }

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

    for day in 0..12 {
        println!("On the {} day of christmas, my true love gave me", day + 1);

        for gift in (0..=day).rev() {
            if day == 0 && gift == 0 {
                println!("and {}", gifts[gift]);
            } else {
                println!("{}", gifts[gift]);
            }
        }
        println!();
    }
}