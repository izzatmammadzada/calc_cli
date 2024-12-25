use std::io::{self, Write};

fn read_number(prompt: &str) -> i32 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input...");
    input
        .trim()
        .parse()
        .expect("Please enter a valid number...")
}

fn valid_option() -> i32 {
    let mut option = 0;

    while option < 1 || option > 4 {
        print!("Please choose an operation number from the list (1-4): ");
        io::stdout().flush().unwrap();

        let mut oinput = String::new();
        io::stdin()
            .read_line(&mut oinput)
            .expect("Failed to read the operation number...");

        let parsed_input: Result<i32, _> = oinput.trim().parse();
        if parsed_input.is_err() {
            println!("Invalid input. Please enter a valid number...");
            continue;
        }

        // Safe to unwrap because we checked `is_err`
        option = parsed_input.unwrap();

        if option < 1 || option > 4 {
            println!("Invalid option. Please enter a number between 1 and 4.");
        }
    }

    option
}

fn main() {
    print!("WELCOME TO THE SIMPLE CALCULATOR!\n");
    let fnumber = read_number("Please enter the first number: ");
    let snumber = read_number("Please enter the second number: ");

    print!("\nOPERATION LIST:\n");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");

    let operation = valid_option();

    if operation == 1 {
        let result = fnumber as f64 + snumber as f64;
        println!("ADD: {} + {} = {:.2}", fnumber, snumber, result);
    } else if operation == 2 {
        let result = fnumber as f64 - snumber as f64;
        println!("SUBTRACT: {} - {} = {:.2}", fnumber, snumber, result);
    } else if operation == 3 {
        let result = fnumber as f64 * snumber as f64;
        println!("MULTIPLY: {} * {} = {:.2}", fnumber, snumber, result);
    } else {
        if snumber == 0 {
            println!("Division by zero is not allowed!");
        } else {
            let result = fnumber as f64 / snumber as f64;
            println!("DIVIDE: {} / {} = {:.2}", fnumber, snumber, result);
        }
    }
}
