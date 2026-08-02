fn main() {
    let n = 25;
    // let fibonacci_series:[u32;n] = [0]; // ig we need vector as we need dynamic length, instead
    // lets use plain string and push..
    
    if n == 1 {
        println!("0");
        return;
    } else if n == 2 {
        println!("0, 1");
        return;
    }
    
    let mut fibonacci_series: String = String::from("0, 1");    
    let mut past_2_no:[u32;2] = [0, 1];

    for _t in 2..n {
        let sum_of_prev_2_no = past_2_no[0] + past_2_no[1];
        fibonacci_series.push_str(format!(", {}", sum_of_prev_2_no.to_string()).as_str());
        past_2_no[0] = past_2_no[1];
        past_2_no[1] = sum_of_prev_2_no;
    }
    println!("{}", fibonacci_series)
}
