use std::io;

fn main() {
    println!("Fibonacci Game");

    loop {
        println!("Please input fahrenheit");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let result = fahrenheit_to_celsius(input);
        println!("Result: {}", result);
    }
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
