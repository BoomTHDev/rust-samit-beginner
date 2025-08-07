fn main() {
    let number = 10;

    if number % 4 == 0 {
        println!("ตัวเลข {} เป็นเลขคู่ที่หารด้วย 4 ลงตัว", number);
    } else if number % 3 == 0 {
        println!("ตัวเลข {} เป็นเลขคี่ที่หารด้วย 3 ลงตัว", number);
    } else {
        println!("ตัวเลข {} ไม่สามารถหารด้วย 4 หรือ 3 ลงตัว", number);
    }

    println!("-----------------------------------------------");

    let condition = true;
    let value = if condition { 5 } else { 6 };
    println!("ค่าที่ได้จาก if-else คือ: {}", value);
}
