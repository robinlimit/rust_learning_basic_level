use demo_1::demo_mod::*;
use pub_lib::add;

fn main() {
    let result = add(1, 2);
    println!("1 + 2 = {}", result);
    mod_name()
}
