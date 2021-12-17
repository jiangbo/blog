# 【Rust】特质-trait

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/trait.html>  

先简单地认为 trait 就是其它语言中的接口，可以为不同的类型定义同一种行为。

## 示例

### Person

```rust
struct Person {
    name: String,
}

trait Say {
    fn say_hello(&self);
}

impl Say for Person {
    fn say_hello(&self) {
        println!("{} say hello", self.name);
    }
}

fn main() {
    let person = Person {
        name: "jiangbo".to_string(),
    };

    person.say_hello();
}
```

### Dog

```rust
struct Person {
    name: String,
}

struct Dog {
    name: String,
}

trait Say {
    fn say_hello(&self);
}

impl Say for Person {
    fn say_hello(&self) {
        println!("{} say hello", self.name);
    }
}

impl Say for Dog {
    fn say_hello(&self) {
        println!("{} say wang wang", self.name);
    }
}

fn main() {
    let person = Person {
        name: "jiangbo".to_string(),
    };

    let dog = Dog {
        name: "wangcai".to_string(),
    };

    person.say_hello();
    dog.say_hello();
}
```

## 总结

了解了 Rust 中 `trait` 一般翻译为特质，或者直接叫英文，和其它语言中的接口类似。

## 附录
