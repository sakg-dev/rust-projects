use std::io;
use std::io::Write;
use regex::Regex;

fn main() {
    loop {
        calculate();
    }
}

fn calculate() {
    print!("Expression: ");
    io::stdout().flush().unwrap();
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().replace(" ", "").to_string();

    let re = Regex::new(r"\d+|[+\-*/]").unwrap();

    let parsed_input = re.find_iter(input.as_str());

    let mut current_eval_val:Option<f32> = None;

    let mut idx = 0;
    for token in parsed_input.into_iter(){ // token is of Match type
        let token_str = token.as_str();
        match token_str.parse::<f32>() { // to identify if its a no. or symbol, we take action on symbols
            Ok(val) => {
                if current_eval_val == None {
                    current_eval_val = Some(val);
                }
            },
            _ => {
                let cloned_parsed_ipt = re.find_iter(input.as_str()); // we need this bcz we need the element which is after the symbol and it consumes the iter and mess hence doing on loop
                let Some(right) = cloned_parsed_ipt.into_iter().nth(idx+1) else {
                    return
                };
                let right_int = right.as_str().parse::<f32>().unwrap();

                if let Some(val) = current_eval_val {
                    let new_eval = match token_str{
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
