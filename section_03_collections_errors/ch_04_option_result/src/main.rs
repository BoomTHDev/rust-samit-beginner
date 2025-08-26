use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

#[allow(dead_code)]
fn find_student_score(name: &str) -> Option<i32> {
    let mut scores = HashMap::new();
    scores.insert("Anan", 85);
    scores.insert("Suda", 92);
    scores.insert("Wichai", 78);

    scores.get(name).copied()
}

#[allow(dead_code)]
fn main_option() {
    let student_name = "Anan";

    match find_student_score(student_name) {
        Some(score) => println!("{} Score: {}", student_name, score),
        None => println!("Student not found"),
    }

    if let Some(score) = find_student_score("Suda") {
        println!("Suda Score: {}", score);
    } else {
        println!("Suda not found");
    }

    let score = find_student_score("Antony");
    println!("Antony's score: {:?}", score.unwrap_or(0));

    let doebled_score = find_student_score("Wichai").map(|s| s * 2);
    println!("Wichai's doubled score: {}", doebled_score.unwrap_or(0));
}

fn read_file_content(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    match read_file_content("test.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(error) => println!("Error: {}", error),
    }

    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }

    let result = divide(10.0, 0.0).unwrap_or_else(|err| {
        println!("Error: {}", err);
        0.0
    });
    println!("Result: {}", result);

    let result = divide(10.0, 0.0).unwrap_or(0.0);
    println!("Result: {}", result);

    if let Ok(result) = divide(10.0, 2.0) {
        println!("Result: {}", result);
    }
}

// unwarp คือ ถ้ามีค่า = ปกติ ถ้าไม่มีค่า = error
// unwarp_or คือ ถ้ามีค่า = ปกติ ถ้าไม่มีค่า = กำหนด default value
// unwarp_or_else คือ ถ้ามีค่า = ปกติ ถ้าไม่มีค่า = กำหนด default value และ error
