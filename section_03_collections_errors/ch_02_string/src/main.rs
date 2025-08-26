fn main() {
    // String - owned, mutable
    let mut greeting = String::new();
    greeting.push_str("Hello");
    greeting.push(' ');
    greeting.push_str("Rust");
    println!("{}", greeting);

    // การสร้างจาก string literal
    let message = String::from("Learning Rust");
    println!("{}", message);

    // การต่อ string
    let full_message = format!("{} - {}", greeting, message);
    println!("{}", full_message);

    // การทำงานกับ UTF-8 - 1 ตัวอักษรภาษาไทยเป็น 3 bytes
    let thai_text = "สวัสดีครับ";
    println!("{}", thai_text);
    println!("length in bytes: {}", thai_text.len());
    println!("length in chars: {}", thai_text.chars().count());

    // การแนก string
    let words: Vec<&str> = "Rust is a safe language".split(' ').collect();
    println!("all words: {:?}", words);
}