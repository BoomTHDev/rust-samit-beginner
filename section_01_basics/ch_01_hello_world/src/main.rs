fn main() {
    println!("Hello, world!");

    print!("Hello, ");
    print!("world!\n");
    print!("\taaa\n");

    println!("-- Base Output --");

    println!("Welcome {} {}", "BoomTH", "Dev");

    println!(
        "{0} is a friend of {1}. And {1} is also a friend of {0}.",
        "Alice", "Bob"
    );

    println!("-- Named Arguments --");
    println!("My name is {name}", name = "Boom");
    println!(
        "I am {old} year old. I live in {city}.",
        old = 21,
        city = "Bangkok"
    );

    println!("-- Formatting Numbers --");
    println!("Pi is approximately {:.2}", 3.14159);
    println!("Pi is approximately {value:.2}", value = 3.14159);
    println!("Number with padding: {:05}", 32);
    println!("Binary: {:b}", 32);
    println!("Hexadecimal: {:x}", 32);
    println!("Octal: {:o}", 32);

    println!("-- Formatting Text --");
    println!("Left aligned: '{:<10}'", "Hello");
    println!("Right aligned: '{:>10}'", "Hello");
    println!("Center aligned: '{:^11}'", "Hello");

    println!("-- Escape Characters --");
    println!("Quote: \"Hello World\"");
    println!("Backslash: \\");
    println!("New line:\nSecond line");
    println!("Tab:\tTabbed Text");
}
