# trait

## 概念

> 类似于其他语言中的接口概念，用于定义多个类型的共同行为，表达他们共有的一种能力。\
> 例如，狮子和老虎都有捕猎的能力，我们可以定义一个捕猎的trait，两种动物共同实现即可。

## 基本写法

- 定义
  
  以下代码定义了一个Summary trait，注意方法没有方法体，方法结束有分号。
  
  ```rust
  pub trait Summary {
      fn summarized(&self)->String;
  }
  ```
- 实现trait

  以下代码是实现trait的最基本形式

  ```rust
  struct NewsArtical{
      headline:String,
      ...
  }
  impl Summary for NewsArtical{
      fn summarized(&self)->String{
          format!("headline:{}",self.headline)
      }
  }
  ```

  **注意：当要实现的trait和实现trait的类都不在同一个文件中时，无法进行trait实现。只有二者其一定义在此文件中时才正常实现trait**

  如果一个trait中的方法有方法体，则此方法体为其默认实现。若某个类型没有重载这个方法，则执行默认实现。

  ```rust
  pub trait Summary {
      fn summarized(&self)->String{
          format!("this is default")
      }
  }

  impl Summary for NewArtical{
      //此处没有重载summarized方法
  }
  
  ```
  ## 使用泛型参数来实现trait

- 用途
    当多个类型都需要实现某个trait时，可以使用泛型参数减少重复代码。

- 写法
  
  为T类型实现`Parse`trait，T类型需要满足约束(`sized`，`clone`)

  ```rust
  impl<T> Parse for T
  where T:sized + clone
  {
    todo!()
  }
  ```

- 案例
  
  - 以下trait定义了某个字符串解析器的方法parse,接收字符串引用，返回Self
  
    new_parse.rs

    ```rust
    pub trait Parse {
        fn parse(s: &str) -> Self
    }
    ```
- 下面分别为u8和f64类型实现trait

    - 其中使用了`map_or()`和`unwrap_or()`方法处理`Option<>`
    - **由于返回值为u8，所以编译器可以推断`.parse()`方法的返回类型，可以简化`.parse::<u8>()`这样的写法**
    - 通过两个实现可以发现，两种类型的实现最大的区别在于正则表达式和匹配失败情况下的默认返回值。这样实现两遍增加了很多重复代码。

    lib.rs

    ```rust
    use regex::Regex //使用外部正则表达式的库

    impl Parse for u8{
        fn parse(s: &str) -> Self{
            let rx: Regex = Regex::new(r"^[0-9]+(.[0-9]+)?").unwrap();//使用正则表达式匹配整数以及小数字符
            if let Some(capture) = rx.captures(s){
                capture.get(0).map_or(0,|s| s.to_str().parse().unwrap_or(0))
            }else{
                0
            }
        }
    }

    impl Parse for f64{
        fn parse(s: &str) -> Self{
            let rx: Regex = Regex::new(r"^[0-9]+(.[0-9]+)?").unwrap();//使用正则表达式匹配整数以及小数字符
            if let Some(capture) = rx.captures(s){
                capture.get(0).map_or(0,|s| s.to_str().parse().unwrap_or(0.1))
            }else{
                0.1
            }
        }
    }

    #[test]
    fn test(){
        assert_eq!(u8::parse("123abc"),123);
        assert_eq!(u8::parse("1234abc"),0);//1234超出了u8,255的最大值,所以parse()返回None，unwrap_or(0)返回默认值0
        assert_eq!(u8::parse("abcd"),0);
        assert_eq!(f64::parse("123.123abcd"),123.123);
        assert_eq!(f64::parse("abcd"),0.1);
    }
    ```
- 使用泛型参数减少重复代码

    T类型有两个约束，可以调用`.parse()`方法，同时要有调用`Default::default()`方法可以返回默认值

    lib.rs

    ```rust
    impl<T> Parse for T
    where T: FromStr + Default
    {
        let rx: Regex = Regex::new(r"^[0-9]+(.[0-9]+)?").unwrap();
        // 生成一个创建缺省值的闭包，这里主要是为了简化后续代码
        // Default::default() 返回的类型根据上下文能推导出来，是 Self
        // 而我们约定了 Self，也就是 T 需要实现 Default trait        
        let d = || Default::default();
        if let Some(capture) = rx.captures(s){
            capture.get(0).map_or(d(),|s| s.to_str().parse().unwrap_or(d()))
        }else{
            d()
        }
    }

    #[test]
    fn test(){
        assert_eq!(u8::parse("123abc"),123);
        assert_eq!(f64::parse("123.123abcd"),123.123);
    }
    ```
## 带关联类型的trait

> trait可以通过方法定义公共行为。同时也可以通过"关联类型"，将定义trait时无法确定的类型，留给trait实现者确定

- 写法

    通过实现`type Error`，我们可以定义`parse()`方法出错时返回都Err(x)中，x的类型，实现了相当的灵活性

    **注意：方法中需要写成`Self::Error`，`Self::`不能省略**

    ```rust
    pub trait Parse{
        type Error;
        fn parse(s: &str) -> Result<Self, Self::Error>
    }

    impl Parse for u8{
        type Error = String;
        todo!()
    }
    ```

- 案例

    `parse()`方法返回默认值不是一个很好的处理，在Rust中这种“可恢复”的错误应当通过返回`Result<_>`处理。 \
    但是在trait定义时，我们没法确定`Result<Self, _>`中的第二个参数，在实现时我们可以确定类型。

    ```rust
    impl<T> Parse for T
    where
        T: FromStr,
    {
        type Error = String;
        fn parse(s: &str) -> Result<Self, Self::Error> {
            let rx = Regex::new(r"^[0-9]+(.[0-9]+)?").unwrap();
            if let Some(capture) = rx.captures(s) {
                capture
                    .get(0)
                    .map_or(Err("failed to parse string".to_string()), |s| {
                        s.as_str()
                            .parse()
                            .map_err(|_err| "failed to parse string".to_string())
                    })
            } else {
                Err("failed to parse string".to_string())
            }
        }
    }
    ```
    ## 使用trait限定泛型参数的类型范围

- 标准语法

    ```rust
    pub fn largest<T:Summary>(tmp:T)->Tn

    pub fn notify<T:Summary>(item1:T,item2:T)
    ```

- 语法糖

    ```rust
    pub fn largest(tmp: impl Summary)->String
    //此写法等同于上面的泛型写法，但是最好不要多用
    ```

- 多个trait约束

    ```rust
    pub fn notify<T:Summary + Display>(item:T)
    ```

- 使用Where来提高多个泛型参数多个trait时可读性

    ```rust
    pub fn notify<T,V>(item1:T,item2:V)
        where T:Summary + Display,
            V:Clone + Debug
    //注意where语句之后再开始写方法体
    {
        todo!();
    }
    ```
## 返回实现了trait的类型，使用trait限定返回值泛型的类型范围

- 标准语法
  
    ```rust
    pub returns_summarizable()->impl Summary{
        todo!();
    }
    ```

- 使用限制

    只有在方法体只可能返回一个类型时才可以使用trait限定返回类型
    
## 在定义trait时使用泛型

> 不同于实现trait时使用泛型参数，定义trait时我们也可以使用泛型参数，增加trait的灵活性。

- 如何定义

    以下这个trait是标准库中用于提供+法运算的，我们可以通过其实现运算符重载。

    在方法名之后使用泛型参数`<Rhs = Self>`，代表add()方法的第二个参数默认与第一个参数类型一致，但是也可以实现两个参数类型不一样的+法

    ```rust
    pub trait Add<Rhs = Self> {
        type Output;
        #[must_use]
        fn add(self, rhs: Rhs) -> Self::Output;
    }
    ```

- 如何实现

    以下例子定义了一个`struct`用来描述数学中的复数，通过操作符重载，我们实现了复数与实数的加法

    main.rs

    ```rust
    //定义复数（实部、虚部）
    pub struct Complex{
        real: f64,
        imagine: f64,
    }
    //构造方法
    impl Complex {
        pub fn new(real: f64, imagine: f64) -> Self{
            Self { real, imagine}
        }
    }
    //实现与实数(f64)的+法
    impl Add<&f64> for &Complex {
        type Output = Complex;

        fn add(self, rhs: &f64) -> Self::Output {
            let real = self.real + rhs;
            Complex::new(real, self.imagine)
        }
    }

    impl Add for &Complex {
        type Output = Complex;

        fn add(self, rhs: Self) -> Self::Output {
            let real = self.real + rhs.real;
            let imagine = self.imagine + rhs.imagine;
            Complex::new(real, imagine)
        }
    }

    fn main(){
        let c1 =Complex::new(1.0, 2.0);
        let c2 = Complex::new(2.0, 3.0);
        let f = 0.64f64;

        println!("{:?}",&c1 + &f);
        println!("{:?}",&c2 + &c1);
    }
    //Complex { real: 1.6400000000000001, imagine: 2.0 }
    //Complex { real: 3.0, imagine: 5.0 }
    ```


