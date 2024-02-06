use text_io::read;
use std::io;
use std::io::Write;

fn get_scores(&student_count: &usize) -> Vec<i32> {
    loop{
        print!("Enter {} scores: ", student_count.to_string());
        io::stdout().flush().unwrap();
        let raw_scores: String = read!("{}\n");
        let scores: Vec<i32> = raw_scores.trim().split(' ') // OVERALL: Turns String to vec<i32> of the words separated by any amount spaces
                    .filter(|s: &&str| !s.is_empty()) // Removes empty entries caused by empty spaces
                    .map(|x: &str| x.parse().unwrap()).collect(); // turns vec<&str> to iter<i32> to vec<i32>
        if scores.len() >= student_count {
            return scores[..student_count].to_vec();
        }
    };
}

fn get_grades(scores: &Vec<i32>) -> Vec<String> {
    let mut grades: Vec<String> = Vec::new(); 
    let high: i32 = *scores.iter().max().unwrap();

    for score in scores {
        grades.push(get_grade(&score, &high));
    }

    grades
}

fn get_grade(score: &i32, high: &i32) -> String {
    let mut grade = String::new();

    match *score {
        s if s > high - 10 => grade = String::from("A"),
        s if s > high - 20 => grade = String::from("B"),
        s if s > high - 30 => grade = String::from("C"),
        s if s > high - 40 => grade = String::from("D"),
        _ => grade = String::from("F"),
    }
    grade
}

fn get_average(scores: &Vec<i32>) -> f64 {
    let sum: i32 = Iterator::sum(scores.iter());
    f64::from(sum) / (scores.len() as f64)
}

fn main() {
    print!("Total Number of Students: ");
    io::stdout().flush().unwrap();
    let student_count: String = read!("{}\n");
    let student_count: usize = student_count.trim().to_lowercase().parse().unwrap();

    let scores: Vec<i32> = get_scores(&student_count);
    let grades: Vec<String> = get_grades(&scores);
    for (idx, (score, grade)) in scores.iter().zip(grades.iter()).enumerate() {
        println!("Student {} score is {} and grade is {}.", idx+1, score, grade);
    }

    let high: i32 = *scores.iter().max().unwrap();
    let average: f64 = get_average(&scores);
    println!("The average score is {:.2} and a grade of {}.", average, get_grade(&(average.round() as i32), &high));
}
