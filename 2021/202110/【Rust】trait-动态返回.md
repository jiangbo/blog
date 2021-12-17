# 【Rust】trait-动态返回

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/trait/dyn.html>  

## 示例

由于 rust 在编译时需要知道占用多少栈空间，所以不能直接返回一个 trait，因为不同的 trait 实现可能占用不同的空间。解决方式是将对象分配到堆上，然后只返回一个引用，引用所占空间是明确的，在堆上分配需要使用 Box。

### main.rs

```rust
struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> &str;
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );
}
```

## 总结

了解了 Rust 中动态返回，这个是实现多态性的必须功能。

## 附录
