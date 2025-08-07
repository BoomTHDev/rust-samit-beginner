fn main() {
    println!("message from main function");

    another_function();
    print_sum(5, 10);
    let result = subtract(10, 5);
    println!("Result from subtract function is {}", result);
    let result = multiply(10, 5);
    println!("Result from multiply function is {}", result);
    block_function();
}

fn another_function() {
    println!("message from another function");
}

fn print_sum(x: i32, y: i32) {
    println!("The sum of {} and {} is {}", x, y, x + y);
}

fn subtract(x: i32, y: i32) -> i32 {
    return x - y;
}

fn multiply(a: i32, b: i32) -> i32 {
    let mul = a * b;
    mul
}

fn block_function() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("Value of y: {}", y);
}
