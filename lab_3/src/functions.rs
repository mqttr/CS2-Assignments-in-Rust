use cute::c;
use rand::Rng;

pub fn add(values: &Vec<f64>) -> f64 {
    c!(x, for x in values, if x < &0.0).iter().sum()
}

pub fn subtract(values: &Vec<f64>) -> f64 {
    let values: Vec<f64> = values.iter().filter(|x: &&f64| **x>0.0).cloned().collect();

    match values.len() {
        0 => return 0.0,
        1 => return values[0],
        _ => return values[0] - (c!(x, for x in &values[1..], if *x > 0.0).iter().sum::<f64>()),
    }
}

pub fn multiply(values: &Vec<f64>) -> f64 {
    let values: Vec<f64> = c!(x, for x in &values[..], if *x != 0.0);

    match values.len() {
        0 => return 0.0,
        1 => return values[0],
        _ => return values.iter().product(),
    }

}

pub fn divide(values: &Vec<f64>) -> f64 {
    let mut result: f64 = values[0];
    
    match values.len() {
        0 => return 0.0,
        1 => return values[0],
        _ => (),
    }

    for val in &values[1..] {

        if *val == 0.0 {
            println!("Cannot divide by 0.");
            std::process::exit(1);
        }
        
        result = result / val;

    }
    result
}

pub fn choose<T>(values: &Vec<T>) -> Option<&T> {
    if values.is_empty() {
        return None;
    }

    let num: usize = rand::thread_rng().gen_range(0..values.len());
    Some(&values[num])
}