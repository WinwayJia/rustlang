pub fn say_hi(name: &str) {
    println!("Hi, {}", name);

    hi(name);
    hello(name);
    say_what(name, hello);

    closured()
}

fn hi(name: &str) -> () {
    println!("Hi, {}.", name);
}

fn hello(name: &str) {
    println!("Hello, {}.", name);
}

fn say_what(name: &str, func: fn(&str)) {
    func(name)
}

// 闭包

fn closured() {
    let i: i32 = 30;
    fn function(i: i32) -> i32 {
        i + 1
    }

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    let one = || 1;
    println!("closure returning one: {}", one());
}
