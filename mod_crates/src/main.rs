//使用use关键字在根节点引入mod
use mod_crates::demo_mod::*;
//use关键字引入类库
use pub_lib::add;

fn main() {
    let result = add(1, 2);
    println!("1 + 2 = {}", result);
    //在mod中定义的pub方法
    mod_name()
}
