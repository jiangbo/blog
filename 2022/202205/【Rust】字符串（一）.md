# 【Rust】字符串（一）

## 环境

- Time 2022-03-24
- Rust 1.59.0

## 示例

### new

```rust
fn main() {
    let s = String::new();
    println!("{:?}", s.capacity());
}
```

### with_capacity

```rust
fn main() {
    let s = String::with_capacity(4);
    println!("{:?}", s.capacity());
}
```

### from_utf8

```rust
fn main() {
    let heart = vec![240, 159, 146, 150];
    let heart = String::from_utf8(heart).unwrap();
    println!("{heart:?}");
}
```

### from_utf8_lossy

```rust
fn main() {
    let heart = vec![240, 159, 146, 150];
    let heart = String::from_utf8_lossy(&heart);
    println!("{heart:?}");
}
```

### from_utf8_unchecked

```rust
fn main() {
    let heart = vec![240, 159, 146, 150];
    let heart = unsafe { String::from_utf8_unchecked(heart) };
    println!("{heart:?}");
}
```

### into_bytes

```rust
fn main() {
    let name = String::from("JiangBo");
    let name = name.into_bytes();
    println!("{name:?}");
}
```

### as_str

```rust
fn main() {
    let name = String::from("JiangBo");
    let name = name.as_str();
    println!("{name:?}");
}
```

### make_ascii_lowercase

```rust
fn main() {
    let mut name = String::from("JiangBo");
    name.as_mut_str().make_ascii_lowercase();
    println!("{name:?}");
}
```

### push_str

```rust
fn main() {
    let mut name = String::from("Jiang");
    name.push_str("Bo");
    println!("{name:?}");
}
```

## 总结

了解了字符串中相关的一些方法。

## 附录
