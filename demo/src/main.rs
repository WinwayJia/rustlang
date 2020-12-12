mod alg;
mod flow;
mod function;
mod json;
mod lib;
mod math;
mod owner;
mod std;
mod structure;
mod thread;
mod vector;

mod aaa {
    pub fn print() {
        println!("aaa::print")
    }
}

fn main() {
    path_mod();

    function::say_hi("hello");

    let mut arr: [i32; 5] = [4, 2, 1, 5, 3];
    alg::sort::partion(&mut arr);

    std::box_demo::demo();

    // enum
    structure::enum_demo::demo();

    thread::demo::demo();
}

fn more() {
    use flow::if_usage;
    if_usage::demo(12);
    if_usage::demo(1);

    println!("result: {}", if_usage::demo_let(true));
    println!("result: {}", if_usage::demo_let(false));

    use flow::loop_usage;
    loop_usage::loop_demo();
    loop_usage::loop_return();

    loop_usage::while_demo();
    loop_usage::for_demo();
    loop_usage::fab(5);
    loop_usage::fab(10);

    owner::ownership::demo();
    println!(
        "first_world return: {}",
        owner::slice::first_word("hello world".to_string())
    );

    structure::struct_demo();
    let rect = structure::Rectangle {
        width: 3.2,
        height: 9.6,
    };
    println!("area={}", rect.area());
    println!("square={:#?}", structure::Rectangle::square(3.2));
    let rect2 = structure::Rectangle::new(3.2, 6.3);
    println!("rect2={:#?}", rect2);

    // json::demo::demo();
}

fn path_mod() {
    lib::print();

    use aaa::print as aprint;
    aprint();

    use math::add;
    let sum = add::add(1, 2);
    println!("sum = {}", sum);

    use math::sub;
    sub::sub();

    math::version();
}
