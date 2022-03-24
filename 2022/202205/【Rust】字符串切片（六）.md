# 【Rust】字符串切片（六）

## 环境

- Time 2022-03-24
- Rust 1.59.0

## 示例

### strip_suffix

```rust
fn main() {
    let name = "\nJiang\nBo\n";
    println!("{:?}", name.strip_suffix('\n'));
}
```

### parse

```rust
fn main() {
    let name = "44";
    println!("{:?}", name.parse::<i32>());
}
```

### is_ascii

```rust
fn main() {
    let name = "\nJiang\nBo\n";
    println!("{:?}", name.is_ascii());
}
```

### eq_ignore_ascii_case

```rust
fn main() {
    let name = "JiangBo";
    println!("{:?}", name.eq_ignore_ascii_case("jiangbo"));
}
```

### make_ascii_uppercase

```rust
fn main() {
    let mut name = String::from("JiangBo");
    name.make_ascii_uppercase();
    println!("{:?}", name);
}
```

### make_ascii_lowercase

```rust
fn main() {
    let mut name = String::from("JiangBo");
    name.make_ascii_lowercase();
    println!("{:?}", name);
}
```

### escape_debug

```rust
fn main() {
    println!("{}", "❤\n!".escape_debug());
}
```

### escape_default

```rust
fn main() {
    println!("{}", "❤\n!".escape_default());
}
```

### escape_unicode

```rust
fn main() {
    println!("{}", "❤\n!".escape_unicode());
}
```

## 总结

了解了字符串切片中相关的一些方法。

## 附录
