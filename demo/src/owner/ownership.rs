pub fn demo() {
    println!("ownership demo");
    let s1 = String::from("hello");
    println!("s1={}", s1);
    // let s2 = s1;
    let s2 = s1.clone();
    println!("s1={}, s2={}", s1, s2);

    take_ownership(s1);
    // println!("s1={}", s1); // v1 value borrowed here after move
    borrow_ownership(&s2);
    println!("s2={}", s2);

    let mut s3 = s2.clone();
    change(&mut s3);
    println!("s3={}", s3);
}

fn take_ownership(s: String) {
    println!("in take_ownership: {}", s);
}

fn borrow_ownership(s: &String) {
    println!("in borrow_ownership: {}", s);
}

// fn change(s: &String) {
fn change(s: &mut String) {
    s.push_str(", world");
    println!("in change: {}", s);
}
