# 【Rust】部分移动

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/scope/move/partial_move.html>  

## 示例

### main.rs

```rust
fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 20,
    };

    // name 发生了移动，但是 age 没有
    let Person { name, ref age } = person;
    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // name 属性已经发生了移动，所有 person 不能整体使用了
    // println!("The person struct is {:?}", person);

    // age 还是可以使用
    println!("The person's age from person struct is {}", person.age);
}
```

## 总结

了解了 Rust 中的部分移动，如果一部分进行了移动，整体就不能使用了，没有移动的部分还是可以正常使用。

## 附录
