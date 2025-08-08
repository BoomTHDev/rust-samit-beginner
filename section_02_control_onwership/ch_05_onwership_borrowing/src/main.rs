// fn main() {
//     let s1 = String::from("hello");

//     // let s2 = s1;
//     // println!("{s2}, world");

//     take_onwership(s1);

//     let x = 5;
//     makes_copy(x);
//     println!("{}", x);
// }

// fn take_onwership(some_string: String) {
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }

// fn main() {
//     let s = String::from("hello");

//     let len = calculate_length(&s);
//     println!("{}", len);
//     println!("s is {}", s);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
