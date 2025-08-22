fn main() {
    let mut numbers: Vec<i32> = Vec::new();

    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    println!("{:?}", numbers);

    let fruits = vec!["Apple", "Banana", "Orange"];
    println!("{:?}", fruits);
    println!("{:?}", fruits[1]);

    let fruits2: Vec<String> = vec![
        "Apple".into(),
        "Banana".into(),
        "Orange".into(),
        "300".into(),
        500.to_string(),
    ];
    println!("{:?}", fruits2);

    let value: i32 = fruits2[4].parse().unwrap();
    println!("{}", value * 2);

    match numbers.get(1) {
        Some(value) => println!("{}", value * 2),
        None => println!("Index out of bounds"),
    }

    for fruit in &fruits {
        println!("found fruit {}", fruit);
    }

    for number in &mut numbers {
        *number *= 2;
    }
    println!("result after multipying by 2: {:?}", numbers);

    // Array
    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    println!("{:?}", arr);

    let arr2: [String; 2] = ["BoomTH".to_string(), "Meen".to_string()];
    println!("{:#?}", arr2);
}
