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

