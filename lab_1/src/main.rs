use std::io;
use std::io::Write;

use std::collections::HashMap;

use text_io::read;


fn main() {
    let mut candidates: HashMap<String, u32> = HashMap::new();
    candidates.insert("Bianca".to_string(), 0);
    candidates.insert("Edward".to_string(), 0);
    candidates.insert("Felicia".to_string(), 0);

    loop {
        let main_option: u32 = vote_menu();
        match main_option {
            0 => {
                println!("{}", "-".repeat(15));
                for candidate in candidates.keys() {
                    print!("{} - {}, ", candidate, candidates[candidate])
                }
    
                println!("Total - {}", candidates.values().sum::<u32>());
    
                println!("{}", "-".repeat(15));
                std::process::exit(0);
            },
            1 => {
                let vote: String = candidate_menu(candidates.keys().cloned().collect::<Vec<String>>());
                candidates.insert(vote.clone(), 1 + if candidates.contains_key(&vote) { candidates[&vote] } else { 0 });
            },
            _ => panic!(),
        }
    }

    
}

fn vote_menu() -> u32 {
    // Returns the user selected option by integer (0, 1)

    println!("{}", "-".repeat(15));
    println!("VOTE MENU");
    println!("{}", "-".repeat(15));
    println!("v: Vote");
    println!("x: Exit");
    println!("{}", "-".repeat(15));

    print!("Option: ");
    io::stdout().flush().unwrap();

    loop {
        let option: String = read!();

        match option.trim().to_lowercase().as_ref() {
            "x" => return 0,
            "v" => return 1,
            _ => {
                print!("Invalid (v/x): ");
                io::stdout().flush().unwrap();
            },
        }
    }
}

fn candidate_menu(candidates: Vec<String>) -> String {
    println!("{}", "-".repeat(15));
    println!("CANDIDATE MENU");
    println!("{}", "-".repeat(15));
    

    for (idx, candidate) in candidates.iter().enumerate() {
        println!("{}: {}", idx+1, candidate);
    }

    print!("Candidate: ");
    io::stdout().flush().unwrap();
    let mut vote: String = read!();

    loop {
        let vote_index: usize = vote.trim().parse::<usize>().unwrap() -1;
        println!("{}", vote_index);
        if (0..candidates.len()).contains(&vote_index) {
            println!("Voted {}.", candidates[vote_index]);
            return candidates[vote_index].clone();
        }

        print!("Invalid(1-{}): ", candidates.len());
        io::stdout().flush().unwrap();
        vote = read!();
    };


}