# 【Rust】字符串切片（四）

## 环境

- Time 2022-03-24
- Rust 1.59.0

## 示例

### rsplit

```rust
fn main() {
    let name = "Jiang\nBo";
    name.rsplit('\n').for_each(|e| println!("{e:?}"));
}
```

### split_terminator

```rust
fn main() {
    let name = "\nJiang\nBo\n";
    name.split_terminator('\n').for_each(|e| println!("{e:?}"));
}
```

### rsplit_terminator

```rust
fn main() {
    let name = "\nJiang\nBo\n";
    name.rsplit_terminator('\n').for_each(|e| println!("{e:?}"));
}
```

### splitn

```rust
fn main() {
    let name = "\nJiang\nBo\n";
    name.splitn(3, '\n').for_each(|e| println!("{e:?}"));
}
```

### rsplitn

```rust
fn main() {
    let name = "\nJiang\nBo\n";
    name.rsplitn(3, '\n').for_each(|e| println!("{e:?}"));
}
```

### split_once

```rust
fn main() {
    let name = "\nJiang\nBo\n";
    println!("{:?}", name.split_once('\n'));
}
```

### rsplit_once

```rust
fn main() {
    let name = "\nJiang\nBo\n";
    println!("{:?}", name.rsplit_once('\n'));
}
```

### matches

```rust
fn main() {
    let name = "\nJiang\nBo\n";
    name.matches(char::is_alphabetic)
        .for_each(|e| println!("{e:?}"));
}
```

### rmatches

```rust
fn main() {
    let name = "\nJiang\nBo\n";
    name.rmatches(char::is_alphabetic)
        .for_each(|e| println!("{e:?}"));
}
```

## 总结

了解了字符串切片中相关的一些方法。

## 附录
