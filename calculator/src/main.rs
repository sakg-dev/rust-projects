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

    enum Operator{
        None,
        Addition,
        Subtraction,
        Multiplication,
        Divide
    }

    enum Token{
        Integer(f32),
        Operator(Operator)
    }

    impl Token {
        fn new(val: &str) -> Self {
            match val.parse::<f32>() {
                Ok(val) => Token::Integer(val),
                _ => Token::Operator(Operator::new(val))
            }
        }
    }

    impl Operator {
        fn new(symbol: &str) -> Self {
            match symbol {
                "+" => Operator::Addition,
                "-" => Operator::Subtraction,
                "*" => Operator::Multiplication,
                "/" => Operator::Divide,
                _ => Operator::None
            }
        }
    }

    let mut current_eval_val:Option<f32> = None;
    let mut idx = 0;
    for match_token in parsed_input.into_iter(){ // token is of Match type
        let token_str = match_token.as_str();
        let token = Token::new(token_str);

        match token {
            Token::Integer(val) => {
                if current_eval_val == None {
                    current_eval_val = Some(val)
                }
            },
            Token::Operator(op) => {
                let cloned_parsed_ipt = re.find_iter(input.as_str());
                let Some(right) = cloned_parsed_ipt.into_iter().nth(idx+1) else {
                    return
                };
                let right_int = right.as_str().parse::<f32>().unwrap();

                if let Some(val) = current_eval_val { 
                    let new_eval = match op {
                        Operator::Addition => val + right_int,
                        Operator::Subtraction => val - right_int,
                        Operator::Multiplication => val * right_int,
                        Operator::Divide => val / right_int,
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
