fn main() {
    let mut a: i32 = 5;
    println!("before change a is {}", a);
    a = change_value(a);
    // 由于基本类型`i32`实现了`copy`trait,所以其可以自由的"进出"函数。
    println!("before chang a is {}", a);
    let str: String = String::from("hello");
    // 类型String无法实现`copy`trait,如果将其传入函数,值将随着函数作用域的结束而Drop.
    // 函数执行之后,将无法访问str
    println!("str has {} characters", count_character(str.clone()));
    println!("after count str is {}", str);
    //内存clone是"昂贵"的，这时我们可以使用borrow
    println!(
        "str has {} characters,count by ref",
        count_char_by_ref(&str)
    );
    println!("after ref count str is {}", str)
}

fn change_value(arg: i32) -> i32 {
    arg * 2
}

//参数直接使用String类型
fn count_character(str: String) -> usize {
    str.len()
} //离开函数作用域,str变量失效,堆内存上的数据也随之销毁

// 参数使用&str引用类型
fn count_char_by_ref(str: &str) -> usize {
    str.len()
} //离开函数作用域,str变量失效,但是由于参数传入过程没有发生所有权转移,所以堆内存上的值没有被销毁
