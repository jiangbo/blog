# 【Rust】闭包-输入参数

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/fn/closures/input_parameters.html>  

## 示例

当将闭包作为参数传递时，必须指明完整的类型，通过 Fn、FnMut、FnOnce 三种 trait 来指定。

### Fn

Fn 可以通过不可变的引用捕获变量。

```rust
fn apply<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn main() {
    let name = "jiangbo";

    let say = || println!("hello: {}", name);
    apply(say);
}
```

### FnMut

可以通过可变引用捕获。

```rust
fn apply<F>(mut f: F)
where
    F: FnMut(),
{
    f();
}

fn main() {
    let mut name = "jiangbo".to_owned();
    let say = || {
        name.push_str("44");
        println!("hello: {:?}", name);
    };
    apply(say);
}
```

### FnOnce

可以通过值捕获。

```rust
fn apply<F>(f: F) -> String
where
    F: FnOnce() -> String,
{
    f()
}

fn main() {
    let name = "jiangbo".to_owned();
    let return_name = || name;
    apply(return_name);
    // apply(return_name);
}
```

### 带参数和返回值

```rust
fn apply<F>(f: F) -> String
where
    F: FnOnce(&'static str) -> String,
{
    f("44")
}

fn main() {
    let mut name = "jiangbo".to_owned();
    let say = |str| {
        name.push_str(str);
        name
    };
    let name = apply(say);
    println!("name: {}", name);
}
```

## 总结

了解了 Rust 中将闭包作为参数传递的几种方式。

## 附录
