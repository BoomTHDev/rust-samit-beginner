use std::collections::HashMap;

fn main() {
    // การสร้าง HashMap
    let mut scores = HashMap::new();
    scores.insert("Team A", 95);
    scores.insert("Team B", 87);
    scores.insert("Team C", 92);

    // การดึงข้อมูล
    match scores.get("Team A") {
        Some(score) => println!("Score Team A: {}", score),
        None => println!("Not found Team A"),
    }

    // การวนลูป
    for (team, score) in &scores {
        println!("{}, Score: {}", team, score);
    }

    println!("----------------------------------------------");

    // การอัพเดทข้อมูล
    scores.entry("Team A").insert_entry(90); // เพิ่มข้อมูล
    scores.entry("Team D").or_insert(0); // เพิ่มถ้าไม่มี
    scores.entry("Team A").and_modify(|score| *score += 5); // เพิ่ม 5 ถ้ามี

    for (team, score) in &scores {
        println!("{}, Score: {}", team, score);
    }
}
