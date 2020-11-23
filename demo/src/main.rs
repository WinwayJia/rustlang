mod flow;
mod function;
mod lib;
mod math;
mod owner;

mod aaa {
    pub fn print() {
        println!("aaa::print")
    }
}

fn main() {
    path_mod();

    function::say_hi("hello");

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
