use std::io;
enum Operation{
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}
fn calculate (operation: Operation) -> f64{
    match operation {
        Operation::Add(a,b ) => a+b,
        Operation::Subtract(a, b)=> a-b,
        Operation::Multiply(a, b)=> a*b,
        Operation::Divide(a, b)=>{
            if b != 0.0{
                a/b
            }else {
                panic!("cannot divide by zero")
            }
        }
        
    }


}
fn main() {
    let mut input = String::new();

    println!("Enter the first number:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num1: f64 = input.trim().parse().expect("Please enter a valid number");
    input.clear();

    println!("Enter the operation (+, -, *, /):");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let operation = input.trim(); // Just borrow it here
    let operation = operation.to_string(); // Convert to string so we can clear input safely
    input.clear();

    println!("Enter the second number:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num2: f64 = input.trim().parse().expect("Please enter a valid number");

    let result = match operation.as_str() {
        "+" => calculate(Operation::Add(num1, num2)),
        "-" => calculate(Operation::Subtract(num1, num2)),
        "*" => calculate(Operation::Multiply(num1, num2)),
        "/" => calculate(Operation::Divide(num1, num2)),
        _ => panic!("Invalid operation!"),
    };

    println!("The result is: {}", result);
}

