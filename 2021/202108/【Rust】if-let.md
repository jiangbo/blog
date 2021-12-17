# 【Rust】if-let

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/flow_control/if_let.html>  

## 示例

### match 和 if-let 的比较

```rust
fn main() {
    let optional = Some(7);

    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
        }
        _ => {}
    };

    if let Some(i) = optional {
        println!("This is a really long string and `{:?}`", i);
    };
}
```

### 不满足处理

```rust
fn main() {
    let letter: Option<i32> = None;
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }
}
```

### 其它条件

```rust
fn main() {
    let emoticon: Option<i32> = None;
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }
}
```

### 处理枚举值

```rust
enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn main() {
    let a = Foo::Bar;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // 匹配绑定
    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }
}
```

## 总结

了解了 Rust 中 `if-let` 语法，有时候比使用 `match` 方便。

## 附录
