extern crate time;
use time::Time;
//use time::Duration;

fn main() {
    //let time = Time::now();
    let time = time::time!(23:59:59);

    println!("Hello, world! {}", time);
}
