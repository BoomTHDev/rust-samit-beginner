fn main() {
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
