use std::{io::{self, Write}};

// USER INPUT FUNCTION
fn user_input() -> f32{

    // print!("{}", msg_to_user); // here was if you want to use &str paramerter in the function to tell users what to do
    io::stdout().flush().expect("...");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line.");

    input.trim().parse::<f32>().expect("Please enter a valid number")
}

// OPARATOR INPUT FUNCTION
fn oparator_input() -> char{

    // print!("{}", msg_to_user); // here was if you want to use &str paramerter in the function to tell users what to do


    // 'Flush stdout' ensure text appears immediately when using print! macro, is Flush is not use, text will appear after user has already enter input
    io::stdout().flush().expect("...");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().parse::<char>().expect("Please, enter a valid oparator!")
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

// enum _Oparators {
//     Add(char),
//     Subtract(char),
//     Multuply(char),
//     Divide(char),
// }

fn main() {


     loop {
    //first input
    print!("Enter first Number: ");
    let number1 = user_input();

    //second input
    print!("Enter second Number: ");
    let number2 = user_input();


   
        println!("OPTION");
        println!("1. Add(+)");
        println!("2. Subtract(-)");
        println!("3. Multiply(*)");
        println!("4. Divide(/)");
        println!("0. Exit");

        print!("Enter Oparator symbol: ");
        let oparator = oparator_input();

           match oparator {
        '0' => break,
        '+' => println!("{number1} + {number2} = {}", addition(number1, number2)),
        '-' => println!("{number1} - {number2} = {}", subtration(number1, number2)),
        '*' => println!("{number1} x {number2} = {}", multiplication(number1, number2)),
        '/' => println!("{number1} / {number2} = {}", division(number1, number2)),
        _ => println!("Can exit with 0."),
    }
    let mut user_default_choice = 2.0;

        while user_default_choice == 2.0 {
            println!("Type 1 to continues or 0 to exit: ");
            let choice = user_input();

            if choice == 1.0 {
                user_default_choice -= 1.0;
            break;
            } else if choice == 0.0 {
            break;
            } else {
                println!("Invalid!")
            }
        }

        if user_default_choice == 2.0 {
            break;
        } else {
            continue;
        }


    //After creating all functions(subtration, multiplication, and division), loop and match will be use
    }

}
