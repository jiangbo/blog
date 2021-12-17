# 【Rust】结构体可见性

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/mod/struct_visibility.html>  

## 示例

结构体的字段默认也是私有的，可以使用 `pub` 来公开。在模块外部不能使用私有的字段，而在模块内部可以，这就是封装。

### 公共字段

```rust
mod my {
    // 公开字段
    pub struct OpenBox<T> {
        pub contents: T,
    }
}

fn main() {
    // 可以直接创建结构体
    let open_box = my::OpenBox {
        contents: "public information",
    };

    // 可以直接访问字段
    println!("The open box contains: {}", open_box.contents);
}
```

### 私有字段

```rust
mod my {

    // 私有字段
    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // 公开的构造方法
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents }
        }
    }
}

fn main() {
    // 编译错误，不能直接构建
    //let closed_box = my::ClosedBox { contents: "classified information" };

    // 通过构造方法创建
    let _closed_box = my::ClosedBox::new("classified information");

    // 编译错误，不能直接访问字段
    //println!("The closed box contains: {}", _closed_box.contents);
}
```

## 总结

了解了 Rust 中结构体的可见性问题，结构体和其中的字段的可见性是分开的。

## 附录
