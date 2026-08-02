use std::io;
use std::io::Write;
use num_bigint::BigUint;

// - unwrap() is used to extract the value from Result or panick is err
// - print! macro prints the string without new line
// - flush() flushes the output stream(the destination) so that all buffered content reaches its destination. We need this in print! bcz rust stores texts in buffer that gets refreshed or flushes on new line(hence we dont need in println!), but not on same line hence manually doing it. We get it from std::io::Write

fn main() {
    print!("Enter the nth term: ");
    io::stdout().flush().unwrap();
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).expect("Input not an integer :/");
    let n:u32 = input.trim().parse().unwrap();

    if n == 0 {
        return;
    } else if n == 1 {
        println!("0");
        return;
    } else if n == 2 {
        println!("0, 1");
        return;
    }

    let mut past_2_no:[BigUint;2] = [BigUint::ZERO, BigUint::ONE];
    print!("0, 1");
    io::stdout().flush().unwrap();
    for _t in 2..n {
        let sum_of_prev_2_no = &past_2_no[0] + &past_2_no[1];
        print!(", {}", sum_of_prev_2_no);
        io::stdout().flush().unwrap();
        past_2_no[0] = past_2_no[1].clone();
        past_2_no[1] = sum_of_prev_2_no;
    }
}
