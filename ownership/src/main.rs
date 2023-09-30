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
}

fn change_value(arg: i32) -> i32 {
    arg * 2
}

fn count_character(str: String) -> usize {
    str.len()
}
