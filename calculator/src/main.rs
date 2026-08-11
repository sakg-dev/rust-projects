use std::io;
use std::io::Write;
use regex::Regex;

fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    print!("Write the calculation here: ");
    io::stdout().flush().unwrap();
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().replace(" ", "").to_string();
    // println!("You wrote: '{}'", input.trim());

    //let parsed_input = input.split(&['+', '-', '/', '*']);
    let re = Regex::new(r"\d+|[+\-*/]").unwrap();
    let parsed_input = re.find_iter(input.as_str());
    
    enum Operators {
        None,
        Add,
        Subtract,
        Multiply,
        Divide
    }

    let mut current_operator: Operators = Operators::None;
    let mut current_eval_val:Option<f32> = None;

    let mut idx = 0;
    for part in parsed_input.into_iter(){
        //println!("{}", part.as_str().parse::<u32>().unwrap());
        match part.as_str().parse::<u32>() {
            Ok(val) => {
                // println!("{} is an integer", val);
                if current_eval_val == None {
                    current_eval_val = Some(val as f32);
                }
            },
            _ => {
                let mut cloned_parsed_ipt = re.find_iter(input.as_str());
                let Some(right) = cloned_parsed_ipt.into_iter().nth(idx+1) else {
                    println!("!!!Something is wrong!!!");
                    return
                };
                let right_int = right.as_str().parse::<f32>().unwrap();
                let symbol = part.as_str();

                // println!("{} is a symbol and {:?} is on its right side", symbol, right_int);

                if let Some(val) = current_eval_val {
                    let new_eval = match symbol{
                        "+" => val + right_int,
                        "-" => val - right_int,
                        "*" => val * right_int,
                        "/" => val / right_int,
                        _ => val
                    };
                    current_eval_val = Some(new_eval);
                }
            }
        }
        idx += 1;
    }
    match current_eval_val{
        Some(evaluated_val) => println!("{} = {}", input, evaluated_val),
        None => println!("Something went wrong")
    }
}
