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
}
