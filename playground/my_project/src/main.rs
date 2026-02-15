use my_project::timings::{time1,time2};
use std::time::Duration;
fn main() {
    let _t1 = time1();
    let _t2 = time2(Duration::new(5, 0));
    println!("Hello, world!");
}
