//使用use关键字在根节点引入mod
use mod_crates::demo_mod::{mod_name, sub};
//use关键字引入类库
use pub_lib::add;
use sub::sub;

fn main() {
    let add_result = add(1, 2);
    println!("1 + 2 = {}", add_result);
    //使用在子模块demo_mod中定义的pub方法
    mod_name();
    //使用在子模块demo_mod的子模块sub中定义的方法
    let sub_result = sub(-1, -2);
    println!("-1-(-2) = {}", sub_result)
}
