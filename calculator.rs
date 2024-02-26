use std::io;

// Define an enum called Operation to represent arithmetic operations
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Define a function called calculate to perform arithmetic based on the operation
fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}

fn main() {
    // Prompt the user to input the first number
    println!("Enter the first number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num1: f64 = input.trim().parse().expect("Please enter a valid number");

    // Prompt the user to input the operation
    println!("Enter the operation (+, -, *, /):");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let operation: char = input.trim().chars().next().expect("Please enter a valid operation");

    // Prompt the user to input the second number
    println!("Enter the second number:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num2: f64 = input.trim().parse().expect("Please enter a valid number");

    // Create an Operation enum instance based on user input
    let operation_enum = match operation {
        '+' => Operation::Add(num1, num2),
        '-' => Operation::Subtract(num1, num2),
        '*' => Operation::Multiply(num1, num2),
        '/' => Operation::Divide(num1, num2),
        _ => panic!("Invalid operation"),
    };

    // Call the calculate function with the created Operation enum instance
    let result = calculate(operation_enum);

    // Print the result to the console
    println!("Result: {}", result);
}
