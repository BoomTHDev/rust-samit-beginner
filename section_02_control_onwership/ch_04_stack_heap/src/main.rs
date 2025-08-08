// fn main() {
//     let x = 5;
//     let y = 10;

//     let sum = add(x, y);

//     println!("Some of {} and {} is {}", x, y, sum);
// }

// fn add(a: i32, b: i32) -> i32 {
//     let result = a + b;
//     result
// }

// fn main() {
//     let s1 = String::from("Hello");
//     let s2 = String::from("World");
//     println!("The original strings are: {}, {}", s1, s2);

//     let result = concatenate(s1, s2);
//     println!("The concatenated string is: {}", result);
// }

// fn concatenate(a: String, b: String) -> String {
//     let result = format!("{} {}", a, b);
//     result
// }

fn main() {
    let b1 = Box::new(5);
    println!("b1 = {}", b1);

    let game = create_game();
    println!("Playing {} with score {}", game.name, game.score);
}

struct Game {
    name: String,
    score: u32,
}

fn create_game() -> Game {
    let game_name = String::from("Minecraft");
    Game {
        name: game_name,
        score: 100,
    }
}
