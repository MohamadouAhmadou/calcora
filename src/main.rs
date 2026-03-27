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

// MULTIPLICATION FUNCTION
fn multiplication(a: f32, b: f32) -> f32 {
    return a * b;
}

// DIVISION FUNCTION
fn division(a: f32, b: f32) -> f32{
   // division by zero will be handle later
    return a / b;
}

fn main() {
     loop {
    //first input
    println!("Enter the first number: ");
    let number1 = user_input();

    //second input
    println!("Enter the second number: ");
    let number2 = user_input();


   
        println!("OPTION");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("0. Exit");

        println!("Choose oparator: ");
        let oparator = user_input();

           match oparator {
        0.0 => break,
        1.0 => println!("The sum of {} and {} is {}.", number1, number2, addition(number1, number2)),
        2.0 => println!("The subtracton of {} and {} is {}.", number1, number2, subtration(number1, number2)),
        3.0 => println!("The multiplication of {} and {} is {}.", number1, number2, multiplication(number1, number2)),
        4.0 => println!("The division of {} and {} is {}.", number1, number2, division(number1, number2)),
        _ => println!("Can exit with 0."),
    }
    }
  
    

    //After creating all functions(subtration, multiplication, and division), loop and match will be use
}
