#[cfg(test)]
mod address {
    #[test]
    fn main() {
        let s = vec![1, 2, 3, 4];
        let s1 = &s;
        println!(
            "Addr of s(s1): {:p}({:p})\nAddr of &s: {:p}\nAddr of s1: {:p}",
            &s, s1, &&s, &s1
        );
        println!("sum of s: {}", sum(s1));
    }

    fn sum(data: &Vec<i32>) -> i32 {
        println!("Addr of data: {:p}\nAddr of &data: {:p}", data, &data);
        data.iter().sum()
    }
    /*
        打印&s显示的是s的地址,s1等于s,它们都指向s所以地址相同
        Addr of s(s1): 0xe829dff7f8(0xe829dff7f8),
        这行打印的是&s这个引用的地址
        Addr of &s: 0xe829dff898,
        这行打印的是s1的地址,可以发现虽然都是s的引用但是它们地址不同。
        说明Rust对于多个只读引用使用的策略是直接复制。
        Addr of s1: 0xe829dff810
        这行打印的是参数data指向的地址,data也指向s所以地址相同
        addr of data: 0xe829dff7f8,
        这行打印的是data这个指针的地址,可以发现3个指向s的指针,本身的地址都不相同。
        addr of &data: 0xe829dff6c8
        sum of s: 10
    */
}
