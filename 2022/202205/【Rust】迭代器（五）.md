# 【Rust】迭代器（五）

## 环境

- Time 2022-03-23
- Rust 1.59.0

## 示例

### find_map

```rust
fn main() {
    let vec = ["lol", "NaN", "2", "5"];
    let result: Option<i32> = vec.iter().find_map(|s| s.parse().ok());
    println!("{result:?}");
}
```

### position

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    let result = vec.iter().position(|e| e > &2);
    println!("{result:?}");
}
```

### rposition

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    let result = vec.iter().rposition(|e| e > &2);
    println!("{result:?}");
}
```

### max

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    let result = vec.iter().max();
    println!("{result:?}");
}
```

### min

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    let result = vec.iter().min();
    println!("{result:?}");
}
```

### rev

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    vec.iter().rev().for_each(|e| println!("{e:?}"));
}
```

### sum

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    let result: i32 = vec.iter().sum();
    println!("{result:?}");
}
```

### product

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    let result: i32 = vec.iter().product();
    println!("{result:?}");
}
```

## 总结

了解了迭代器中相关的一些方法。

## 附录
