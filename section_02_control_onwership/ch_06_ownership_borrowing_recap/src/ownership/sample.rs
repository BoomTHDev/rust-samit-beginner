pub fn ownership_example() {
    // s1 is owner of String
    let s1 = String::from("Hello Rustaceans!");
    println!("{}", s1);

    // Move: ownership from s1 to s1
    let s2 = s1;
    println!("{}", s2);

    // Error! s1 cannot used
    // println!("{}", s1);

    takes_ownership(s2);

    // Error! s2 cannot used
    // println!("{}", s2);
} // Free s2 this scope

fn takes_ownership(str: String) {
    println!("{}", str);
} // Free str this scope
