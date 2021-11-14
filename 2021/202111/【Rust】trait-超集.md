# 【Rust】trait-超集

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/trait/supertraits.html>  

## 示例

类似其它语言中的接口继承。

### main.rs

```rust
trait Person {
    fn name(&self) -> String;
}

// 使用冒号来定义超集（父 trait）
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// 同时实现两个 trait，并且一个有父 trait
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &impl CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

fn main() {}
```

## 总结

了解了 Rust 中的超集（父 trait），类似其它语言中的接口继承。

## 附录
