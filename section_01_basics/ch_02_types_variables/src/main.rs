fn main() {
    println!("-------------------------");

    // Statically Typed และการอนุมานชนิดข้อมูล (Type Inference)
    let a: i32 = 100;
    let b = -200;

    println!("integer with explicit type: {}", a);
    println!("integer with inferred type: {}", b);

    println!("-------------------------");

    // Scalar Types: Floating-Points
    let temp: f64 = 3.14159;
    let presure: f32 = 10.984648;

    println!("floating-point number: {}", temp);
    println!("floating-point number: {}", presure);

    println!("-------------------------");

    // Signed integer สามารถติดลบได้
    let balance: i32 = -500;
    println!("Signed interger: {}", balance);

    // Unsigned integer เก็บได้แค่ค่าบวกและศูนย์
    let age: u8 = 0;
    println!("Unsigned integer: {}", age);

    println!("-------------------------");

    // Boolean type
    let is_active: bool = true;
    let is_logged_in: bool = false;

    println!("Is active: {}", is_active);
    println!("Is logged in: {}", is_logged_in);

    println!("-------------------------");

    // Character type
    let letter: char = 'R';

    println!("Letter: {}", letter);

    println!("-------------------------");

    // Character type string
    let greeting: String = "Hello, world!".to_string();
    println!("Greeting: {}", greeting);
}
