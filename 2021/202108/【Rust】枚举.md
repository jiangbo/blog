# 【Rust】枚举

## 环境

- Rust 1.55.0
- VSCode 1.59.1

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/custom_types/enum.html>

`enum` 关键字允许创建一个从数个不同取值中选其一的枚举类型（enumeration）。  
任何一个在 `struct` 中合法的取值在 `enum` 中也合法。

## 示例

```rust
enum WebEvent {
    // 单元类型的枚举
    PageLoad,
    PageUnload,
    // 元组类型的枚举
    KeyPress(char),
    Paste(String),
    // C 类型的枚举
    Click { x: i64, y: i64 },
}
```

### 单元类型

```rust
enum WebEvent {
    // 单元类型的枚举
    PageLoad,
    PageUnload,
}
```

### 元组类型

```rust
enum WebEvent {
    // 元组类型的枚举
    KeyPress(char),
    Paste(String),
}
```

### C 类型

```rust
enum WebEvent {
    // C 类型的枚举
    Click { x: i64, y: i64 },
}
```

### 简单使用

```rust
enum WebEvent {
    // 单元类型的枚举
    PageLoad,
    PageUnload,
    // 元组类型的枚举
    KeyPress(char),
    Paste(String),
    // C 类型的枚举
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
```

## 总结

了解了 Rust 中的枚举类型，有三种类型的枚举风格，简单使用了枚举类型。

## 附录
