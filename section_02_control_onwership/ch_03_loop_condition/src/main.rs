fn main() {
    loop_function();

    println!("--------------------");

    while_function();

    println!("--------------------");

    for_function();
}

fn loop_function() {
    let mut counter = 0;

    loop {
        counter += 1;

        if counter == 3 {
            println!("Next");
            continue;
        }

        if counter == 5 {
            println!("Stop");
            break;
        }

        println!("รอบที่: {}", counter);
    }
}

fn while_function() {
    let mut number = 3;
    let max_number = 10;

    while number < 10 && number < max_number {
        if number == 5 {
            println!("Skipping number: {}", number);
            number += 1;
            continue;
        } else if number == 8 {
            println!("Number is {}. Stop loop", number);
            break;
        } else {
            println!("Working on current number: {}", number);
        }
        number += 1;
    }
}

fn for_function() {
    for i in 1..5 {
        println!("Current value: {}", i);
    }

    println!("--------------------");

    for i in 1..=5 {
        println!("Current value: {}", i);
    }

    println!("--------------------");

    for i in (1..=5).rev() {
        println!("Current value: {}", i);
    }

    println!("--------------------");

    for i in (1..=5).step_by(2) {
        println!("Current value: {}", i);
    }

    println!("--------------------");
}
