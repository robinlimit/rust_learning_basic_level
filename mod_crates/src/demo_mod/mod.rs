/*
    文件夹名称就是mod名称，只要文件夹中新建了mod.rs文件。
    下一级的mod，可以在此文件中“注册”
*/

//下一级mod在此注册
pub mod sub;

pub fn mod_name() {
    println!("mod name: demo_mod")
}
