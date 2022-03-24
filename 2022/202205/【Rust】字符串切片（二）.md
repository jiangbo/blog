# 【Rust】字符串切片（二）

## 环境

- Time 2022-03-24
- Rust 1.59.0

## 示例

### get_unchecked

```rust
fn main() {
    let name = "JiangBo";
    println!("{:?}", unsafe { name.get_unchecked(5..) });
}
```

### get_unchecked_mut

```rust
fn main() {
    let mut name = String::from("JiangBo");
    println!("{:?}", unsafe { name.get_unchecked_mut(5..) });
}
```

### split_at

```rust
fn main() {
    let name = "JiangBo";
    println!("{:?}", name.split_at(5));
}
```

### split_at_mut

```rust
fn main() {
    let mut name = String::from("JiangBo");
    println!("{:?}", name.split_at_mut(5));
}
```

### chars

```rust
fn main() {
    let name = "JiangBo";
    name.chars().for_each(|c| println!("{c}"));
}
```

### char_indices

```rust
fn main() {
    let name = "JiangBo";
    name.char_indices().for_each(|e| println!("{e:?}"));
}
```

### bytes

```rust
fn main() {
    let name = "JiangBo";
    name.bytes().for_each(|e| println!("{e:?}"));
}
```

### split_whitespace

```rust
fn main() {
    let name = "Jiang Bo";
    name.split_whitespace().for_each(|e| println!("{e:?}"));
}
```

### split_ascii_whitespace

```rust
fn main() {
    let name = "Jiang Bo";
    name.split_ascii_whitespace()
        .for_each(|e| println!("{e:?}"));
}

```

## 总结

了解了字符串切片中相关的一些方法。

## 附录
