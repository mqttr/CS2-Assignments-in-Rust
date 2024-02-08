use std::env;

mod functions;


fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();
    
    if args.len() < 3 {
        println!("Operation and 2 floats are required as arguments.");
        std::process::exit(1);
    }

    let numbers: Vec<f64> = args[1..].iter().map(|x: &String| x.parse::<f64>())
                                .filter_map(Result::ok).collect::<Vec<f64>>();
    
    if 1 != args.len() - numbers.len() { // Difference should be 1 as only the operator should be removed
        println!("All arguments after operation must be float numbers.");
        std::process::exit(1);
    }

    let command: String = args[0].clone();

    let result: f64 = match command.to_lowercase().trim().as_ref() {
        "add"      => functions::add(&numbers),
        "subtract" => functions::subtract(&numbers),
        "multiply" => functions::multiply(&numbers),
        "divide"   => functions::divide(&numbers),
        "choose"   => *functions::choose(&numbers).unwrap(),
        _ => {
            println!("Command '{}' unrecognized please try again.", command.to_lowercase().trim());
            std::process::exit(1);
        }
    };

    println!("Answer: {:.2}", result);
}