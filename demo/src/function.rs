pub fn say_hi(name: &str) {
    println!("Hi, {}", name);

    hi(name);
    hello(name);
    say_what(name, hello);
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
