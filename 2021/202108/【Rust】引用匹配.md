# 【Rust】引用匹配

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/flow_control/match/destructuring/destructure_pointers.html>  

## 示例

### 引用匹配

```rust
fn main() {
    let reference = &4;
    match reference {
        // 匹配的引用，并且进行了解构
        &val => println!("Got a value via destructuring: {:?}", val),
    }
}
```

### 解引用匹配

```rust
fn main() {
    let reference = &4;
    match *reference {
        // 已经解引用了，直接可以获取到值
        val => println!("Got a value via dereferencing: {:?}", val),
    }
}
```

### 值匹配

```rust
fn main() {
    let value = 5;
    match value {
        // 匹配的值，获取到一个引用
        ref r => println!("Got a reference to a value: {:?}", r),
    }
}
```

### 可变值匹配

```rust
fn main() {
    let mut mut_value = 6;

    match mut_value {
        ref mut m => {
            // 可变值匹配，并修改值
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }
}
```

## 总结

了解了 Rust 中的引用的匹配和值的匹配并取得值的引用。

## 附录
