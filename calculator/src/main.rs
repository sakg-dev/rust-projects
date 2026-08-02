use std::io;
use std::io::Write;

fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn parse_input(input: &String) {
    let ints = input.split(&['+', '-', '/', '*']);
    // let symbols = input.match_indices(&['+', '-', '/', '*']);
    _print_type_of(&ints);
}

fn main() {
    print!("Write the calculation here: ");
    io::stdout().flush().unwrap();
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().replace(" ", "").to_string();
    // println!("You wrote: '{}'", input.trim());

    let parsed_input = input.split(&['+', '-', '/', '*']);
    parse_input(&input);

    // let parsed_ipt_len = parsed_input.clone().count(); // need to clone as count consumes the iterator
    //if parsed_ipt_len < 3 {
    //    println!("Insufficient elements");
    //    return;
    //} 
    for part in parsed_input{
        println!("{}", part)
    }
}
