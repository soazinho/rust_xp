use std::io;

fn main() {
    println!("Fibonacci Game");

    println!("Please input your nth number.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u128 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let result = find_nth_fibonacci_number(input);
    println!("The {input}th Fibonacci number is {result}.");
}

fn find_nth_fibonacci_number(value: u128) -> u128 {
    if value <= 0 {
        return 0;
    }
    if value == 1 {
        return 1;
    }

    find_nth_fibonacci_number(value - 1) + find_nth_fibonacci_number(value - 2)
}
