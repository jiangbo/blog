# 【Rust】字符串（三）

## 环境

- Time 2022-03-24
- Rust 1.59.0

## 示例

### remove

```rust
fn main() {
    let mut name = String::from("JiangBo");
    name.remove(4);
    println!("{name:?}");
}
```

### retain

```rust
fn main() {
    let mut name = String::from("JiangBo");
    name.retain(|c| c.is_ascii_uppercase());
    println!("{name:?}");
}
```

### insert

```rust
fn main() {
    let mut name = String::from("JiangBo");
    name.insert(5, '-');
    println!("{name:?}");
}
```

### insert_str

```rust
fn main() {
    let mut name = String::from("JiangBo");
    name.insert_str(5, "----");
    println!("{name:?}");
}
```

### as_mut_vec

```rust
fn main() {
    let mut name = String::from("JiangBo");
    let name = unsafe { name.as_mut_vec() };
    println!("{name:?}");
}
```

### len

```rust
fn main() {
    let name = String::from("JiangBo");
    println!("{:?}", name.len());
}
```

### is_empty

```rust
fn main() {
    let name = String::from("JiangBo");
    println!("{:?}", name.is_empty());
}
```

### split_off

```rust
fn main() {
    let mut name = String::from("JiangBo");
    let name = name.split_off(5);
    println!("{:?}", name);
}
```

### clear

```rust
fn main() {
    let mut name = String::from("JiangBo");
    name.clear();
    println!("{:?}", name);
}
```

## 总结

了解了字符串中相关的一些方法。

## 附录
