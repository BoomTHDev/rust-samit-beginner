mod calculate;
use calculate::calculate_item_total;

fn main() {
    // define product 2 items
    let apple_price = 15.5;
    let apple_quantity = 5;
    let banana_price = 10.0;
    let banana_quantity = 7;

    let apple_total = calculate_item_total(apple_price, apple_quantity);
    let banana_total = calculate_item_total(banana_price, banana_quantity);

    println!("Total of apple: {:.2}", apple_total);
    println!("Total of banana: {:.2}", banana_total);

    let total = apple_total + banana_total;
    println!("Total all items: {:.2}", total);
}
