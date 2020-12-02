enum IpAddrKind {
    V4,
    V6,
}

pub fn demo() -> f64 {
    let some_number = Some(5);
    // let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    /*
    println!(
        "some_number: {}, {}",
        some_number.unwrap(),
        absent_number.unwrap()
    );
    */

    // let mut ans = div(10.2, 10.0)?;
    try_division(1, 1);
    try_division(1, 0);
    0.01
}

pub fn option() {}

pub enum MathError {
    DivisionByZero,
    NegativeLogarithm,
    NegativeSquareRoot,
}

pub type MathResult = Result<f64, MathError>;

fn div(x: f64, y: f64) -> MathResult {
    if y == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(x / y)
    }
}

fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        // 失败表示成 `None` 取值
        None
    } else {
        // 结果 Result 被包装到 `Some` 取值中
        Some(dividend / divisor)
    }
}

// 此函数处理可能失败的除法
fn try_division(dividend: i32, divisor: i32) {
    // `Option` 值可以进行模式匹配，就和其他枚举类型一样
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => println!("{} / {} = {}", dividend, divisor, quotient),
    }
}
