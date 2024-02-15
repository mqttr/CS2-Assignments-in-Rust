


use regex::Regex;
use csv;

use std:: {fs, io::Write, fs::File, path::Path, collections::HashMap};

fn main() {
    let content = fs::read_to_string(Path::new("input.txt"))
        .expect("File not located or unable to be read.");

    let re_date: Regex = Regex::new(r"From (.+)\W").unwrap();
    let date: Vec<&str> = re_date.captures_iter(&content).map(|m| m.get(1).unwrap().as_str().trim()).collect();

    let date_split: Vec<Vec<&str>> = date.into_iter().map(|m| m.split(" ").filter(|m| !m.is_empty())
            .collect::<Vec<&str>>().clone()).collect();


    let csv_path = Path::new("out.csv");
    let _ = write_csv(csv_path, &date_split);

    

    let re_email: Regex = Regex::new(r"From: (.+)\W").unwrap();
    let emails: Vec<&str> = re_email.captures_iter(&content).map(|m| m.get(1).unwrap().as_str().trim()).collect();

    let mut email_count: HashMap<String, u32> = HashMap::new();
    for email in emails {
        email_count.insert(email.to_owned(), 1 + if email_count.contains_key(&email.to_owned())  { email_count[&email.to_owned()] } else { 0 } );
    }


    let file_path = Path::new("out.txt");
    let _ = write_file(&file_path, &email_count);

    dbg!(email_count);
}

fn write_file(file_path: &Path, lines: &HashMap<String, u32>) -> Result<(), ()>{
    let mut file_write = File::create(file_path).expect("Cannot create file");


    writeln!(file_write, "{: <40} - {}", "Email", "Count").unwrap();
    for (email, count) in lines.keys().zip(lines.values()) {
        writeln!(file_write, "{: <40} - {}", email, count).unwrap();
    }
    writeln!(file_write, "{}", "-".repeat(45)).unwrap();

    writeln!(file_write, "{: <40} - {}", "Total Emails", lines.values().sum::<u32>()).unwrap();
    
    Ok(())
}


fn write_csv(csv_path: &Path, matrix: &Vec<Vec<&str>>) -> Result<(), ()> {
    let mut wtr = csv::Writer::from_path(csv_path).expect("Hello");

    wtr.write_record(&["Email", "Day", "Date", "Month", "Year","Time"]).unwrap();


    for row in matrix {
        wtr.write_record([&row[0], &row[1], &row[3], &row[2], &row[5], &row[4]]).unwrap();
    }

    wtr.flush().unwrap();
    Ok(())
}