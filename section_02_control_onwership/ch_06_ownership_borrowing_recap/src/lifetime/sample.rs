pub fn lifetime_example() {
    let string1 = String::from("Programming");
    let string2 = String::from("Rust");

    let result = longest(&string1, &string2);
    println!("The longest string is: {}", result);

    // Error case 1
    let s1 = String::from("Programming");
    let r;
    let s2 = String::from("Rust");

    {
        r = longest(&s1, &s2);
    }

    println!("The longest string is: {}", r);

    // Error case 2
    let str2 = String::from("Rust");
    // let result = longest(String::from("Programming").as_str(), &str2);
    // println!("{}", result);

    // Error case 3
    let result = bad_longest(&String::from("Programming"), &str2);
    println!("{}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// fn bad_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
fn bad_longest<'a>(x: &'a str, y: &'a str) -> String {
    // let local = String::from("temporary");
    // if x.len() > y.len() { x } else { local.as_str() }
    if x.len() > y.len() {
        x.to_string()
    } else {
        String::from("temporary")
    }
}
