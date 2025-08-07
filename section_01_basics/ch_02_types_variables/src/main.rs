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

    println!("-------------------------");

    let x = 5;
    println!("The value of x is: {}", x);

    println!("-------------------------");

    let name = "Alice";
    let age = 30;
    println!("My name is {} and I am {} years old.", name, age);

    println!("-------------------------");

    let immutable_var = 10;
    println!("Immutable variable value: {}", immutable_var);

    let mut mutable_var = 15;
    println!("Original value: {}", mutable_var);
    mutable_var = 25;
    mutable_var += 5;
    println!("Mutable variable value: {}", mutable_var);

    println!("-------------------------");

    const MAX_POINTS: u32 = 100_000;
    const MIN_POINTS: u32 = 0;
    println!("The maximum points are: {}", MAX_POINTS);
    println!("The minimum points are: {}", MIN_POINTS);

    println!("-------------------------");

    let x = 5;
    println!("The value of x is: {}", x);
    let x = "Hello";
    println!("The value of x is now: {}", x);

    println!("-------------------------");

    let outer_var = "I'm outside";
    {
        let inner_var = "I'm inside";
        println!("{}", outer_var);
        println!("{}", inner_var);
    }

    println!("{}", outer_var);

    println!("-------------------------");

    let distance: Kilometers = 42;
    println!("Distance in kilometers: {}", distance);

    println!("-------------------------");

    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {:.1}", result),
        Err(error) => println!("Error: {}", error),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("Result: {:.1}", result),
        Err(error) => println!("Error: {}", error),
    }
}

// Type Aliases
type Kilometers = i32;

#[allow(dead_code)]
#[allow(unused_variables)]
fn unused_function() {
    let unused_var = 42;
    println!("This function is not used, but Rust won't complain.");
}

fn divide(num1: f64, num2: f64) -> Result<f64, &'static str> {
    if num2 == 0.0 {
        Err("Cannot divide by zero!")
    } else {
        Ok(num1 / num2)
    }
}
