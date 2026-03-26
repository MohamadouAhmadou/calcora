use std::io;

// USER INPUT FUNCTION
fn user_input() -> f32{
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line.");

    input.trim().parse::<f32>().expect("Please enter a valid number")
}

// ADDITION FUNCTON
fn addition(a: f32, b: f32) -> f32{
    return a + b;
}

// SUBTRACTION FUNCTION
fn subtration(a: f32, b: f32) -> f32 {
    return a - b;
} 


fn main() {
    //first input
    println!("Enter the first number: ");
    let number1 = user_input();

    //second input
    println!("Enter the second number: ");
    let number2 = user_input();

    println!("The sum of {} and {} is {}.", number1, number2, addition(number1, number2));
    println!("The subtracton of {} and {} is {}.", number1, number2, subtration(number1, number2));

    //After creating all functions(subtration, multiplication, and division), loop and match will be use
}
