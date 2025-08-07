fn main() {
    let status_code = 404;

    match status_code {
        200 => println!("Ok"),
        404 => {
            println!("Error: Page Not Found.");
            println!("Please check the URL and try again.");
            report_error_to_system(404);
        }
        500 => {
            println!("Error: Internal Server Error");
            println!("Our engineers have been notified.");
            report_error_to_system(500);
        }
        _ => println!("Error: Unknown Status Code"),
    }

    println!("---------------------------------------------------------");

    let http_method = "POST";

    match http_method {
        "GET" => println!("Requesting data..."),
        "POST" | "PUT" => println!("Sending data..."),
        "DELETE" => println!("Deleting data..."),
        _ => println!("Unknown Method"),
    }

    println!("---------------------------------------------------------");

    let day = 1;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        _ => println!("Other day"),
    }

    println!("---------------------------------------------------------");

    let number = 22;

    match number {
        1..=10 => println!("Number is between 1 and 10"),
        11..=20 => println!("Number is between 11 and 20"),
        21..=30 => println!("Number is between 21 and 30"),
        _ => println!("Number is outside the range of 1 to 30"),
    }

    println!("---------------------------------------------------------");

    let value: Option<i32> = Some(42);
    // let value: Option<i32> = None;

    match value {
        Some(v) => println!("Value is: {}", v),
        None => println!("No value provided"),
    }
}

fn report_error_to_system(code: u32) {
    println!("-> Reporting error code {} to monitoring system...", code);
}
