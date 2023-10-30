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
