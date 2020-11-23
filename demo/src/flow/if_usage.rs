pub fn demo(x: i32) {
    if x > 10 {
        println!("x({}) is greater than 10", x)
    } else {
        println!("x({}) is less than 10", x)
    }

    if x % 4 == 0 {
        println!("number is divisible by 4");
    } else if x % 3 == 0 {
        println!("number is divisible by 3");
    } else if x % 5 == 0 {
        println!("number is divisible by 5");
    } else {
        println!("number is not divisible by 3,4,5");
    }
}

// 在 let 语句中使用 if
pub fn demo_let(condition: bool) -> i32 {
    let number = if condition { 5 } else { 6 };

    number
}
