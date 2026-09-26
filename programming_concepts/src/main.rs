fn main() {
    println!("Hello, world!");

    // Error at build time: String to u32
    // let spaces_str = "   ";
    // let spaces_num = spaces_str.len();

    // Shadowing
    let mut spaces = "   ";
    let spaces: usize = spaces.len();

    println!("{}", spaces);

    print_labeled_measurement(5, 'h');

    // (let y = 6) is not an expression, so it cannot be assigned to a variable
    // let x = (let y = 6);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
