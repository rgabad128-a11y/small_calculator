use std::io::{self, Write};

fn main() {
    println!("Small Calculator");
    println!("================\n");
    loop{
        let mut input1 = String::new();
        
        print!("Enter the first number>>> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input1)
            .expect("Error try again ;(");

        let clean_input1 = input1.trim();
        //Here we have put the cleaning process before converting to f64 so that we can put the exit command from the program before converting the data to f64 so that we do not have a conflict problem

        if clean_input1 == "q" {
            return;
        }


        let num1: f64 = match clean_input1.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Syntax Error");
                continue;
            }
        };
        //Just a command to deal with user errors without crashing or a general error message

        let mut operator = String::new();

        print!("Enter the operator>>> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut operator)
            .unwrap();
        let clean_operator = operator.trim();

        let mut input2 = String::new();

        print!("Enter the second number>>> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input2)
            .unwrap();

        let num2: f64 = match input2.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Syntax Error");
                    continue;
            }
        };

        let result = match clean_operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("Error you cannot divide by zero");
                    continue;
                }
                num1 / num2
            }
            "%" => num1 % num2,
            _ => {
                println!("Syntax Error");
                continue;
            }
        };
        println!("The result>>> {} {} {} = {}", num1, clean_operator, num2, result);
    }
}
