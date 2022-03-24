# 【Rust】字符串切片（五）

## 环境

- Time 2022-03-24
- Rust 1.59.0

## 示例

### match_indices

```rust
fn main() {
    let name = "\nJiang\nBo\n";
    name.match_indices(char::is_alphabetic)
        .for_each(|e| println!("{e:?}"));
}
```

### rmatch_indices

```rust
fn main() {
    let name = "\nJiang\nBo\n";
    name.rmatch_indices(char::is_alphabetic)
        .for_each(|e| println!("{e:?}"));
}
```

### trim

```rust
fn main() {
    let name = "\nJiang\nBo\n";
    println!("{:?}", name.trim());
}
```

### trim_start

```rust
fn main() {
    let name = "\nJiang\nBo\n";
    println!("{:?}", name.trim_start());
}
```

### trim_end

```rust
fn main() {
    let name = "\nJiang\nBo\n";
    println!("{:?}", name.trim_end());
}
```

### trim_matches

```rust
fn main() {
    let name = "\nJiang\nBo\n";
    println!("{:?}", name.trim_matches('\n'));
}
```

### trim_start_matches

```rust
fn main() {
    let name = "\nJiang\nBo\n";
    println!("{:?}", name.trim_start_matches('\n'));
}
```

### trim_end_matches

```rust
fn main() {
    let name = "\nJiang\nBo\n";
    println!("{:?}", name.trim_end_matches('\n'));
}
```

### strip_prefix

```rust
fn main() {
    let name = "\nJiang\nBo\n";
    println!("{:?}", name.strip_prefix('\n'));
}
```

## 总结

了解了字符串切片中相关的一些方法。

## 附录
