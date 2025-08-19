pub fn borrowing_example() {
    let s1 = String::from("Hello Rustaceans!");

    // Borrowing: s1 borrowing by s2
    let s2 = &s1;

    // s1 can use
    println!("{}", s1);
    println!("{}", s2);

    // let len = calculate_length(&s1);
    let len = calculate_length(s2);

    println!("Length: {}", len);

    // Kind of borrow to can change of string
    let mut s = String::from("Hello");
    change_string(&mut s);
    println!("Modified String: {}", s);

    // Cannot have mut and imut ref
    let mut r1 = String::from("Rust is Awesome!");

    // Many Imutable referrence - OK
    let r2 = &r1;
    let r3 = &r1;

    println!("r2: {}", r2);
    println!("r3: {}", r3);
    // Free r2, r3 (NLL - Non-Lexical Lifetimes)

    let r4 = &mut r1;
    r4.push_str(" Programming");
    println!("r4: {}", r4);

    let r5 = &mut r1;
    r5.push_str(" is fun!");
    println!("r5: {}", r5);
}

fn calculate_length(s: &String) -> i32 {
    s.len() as i32 // Can read but cannot edit value
}

fn change_string(s: &mut String) {
    s.push_str(", Rust!");
}
