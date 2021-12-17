# 【Rust】特质-trait

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/trait.html>  

## 示例

trait 一般翻译成特质，之后的内容不翻译，类似其它语言中的接口。

### main.rs

```rust
struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    // Self 表示实现的类型，注意关联函数和方法的区别
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &str;
    fn noise(&self) -> &str;
    // 可以进行默认实现
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep {
            name: name,
            naked: false,
        }
    }

    fn name(&self) -> &str {
        self.name
    }

    fn noise(&self) -> &str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }

    fn talk(&self) {
        // 可以覆盖默认的方法
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    // 类型注解必须要
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();
}
```

## 总结

了解了 Rust 中 trait 的定义方式和实现方式。

## 附录
