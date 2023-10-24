fn main() {
    let result = split_then_multiply("+6*+2*+9*+6");
    println!("{result}");
}

fn split_then_multiply(input_string: &str) -> f64 {
    let mut result = 1.;
    let multiplicands: Vec<&str> = input_string.split("+").collect();
    for num_str in multiplicands {
        let num = num_str.trim().parse::<f64>().expect("invalid input");
        result = result * num;
    }
    return result;
}
