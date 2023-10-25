mod math_parser;

fn main() {
    let result = evaluate("4*10-43+2*12");
    println!("{:?}", result);

    let result = evaluate("8*40-47+2*13");
    println!("{:?}", result);

    let result = evaluate("567*670-565+654*12");
    println!("{:?}", result);

    let result = evaluate("4*70-43+2*12");
    println!("{:?}", result);
}

fn evaluate(input: &str) -> f64 {
    let mut result = 0.;
    let terms = math_parser::split_plus_minus(input);
    for term in terms {
        let result1 = split_then_multiply(&term);
        result = result + result1;
    }
    result
}

fn split_then_multiply(input_string: &str) -> f64 {
    let mut result = 1.;
    let multiplicands: Vec<&str> = input_string.split("*").collect();
    for num_str in multiplicands {
        let num = num_str.trim().parse::<f64>().expect("invalid input");
        result = result * num;
    }
    return result;
}
