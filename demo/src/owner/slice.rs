pub fn first_word(s: String) -> String {
    let bytes = s.as_bytes();

    let mut idx = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            idx = i;
            break;
        }
    }

    println!("idx={}", idx);
    slice_demo();

    s[idx..].to_string()
}

fn slice_demo() {
    let s = String::from("hello world");

    let mut hello = &s[0..5];
    let world = &s[6..11];
    println!("hello={} world={}", hello, world);

    hello = "xxx";
    println!("hello={} world={}", hello, world);

    let a = [1, 2, 3, 4, 5];
    for (i, v) in a.iter().enumerate() {
        println!("i={} v={}", i, v)
    }

    let sub = &a[1..3];
    for v in sub.iter() {
        println!("v={}", v);
    }
}
