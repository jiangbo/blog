# 【Rust】哈希映射（二）

## 环境

- Time 2022-03-24
- Rust 1.59.0

## 示例

### into_values

```rust
fn main() {
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    map.into_values().for_each(|e| println!("{e:?}"));
}
```

### iter

```rust
fn main() {
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    map.iter().for_each(|e| println!("{e:?}"));
}
```

### iter_mut

```rust
fn main() {
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    map.iter_mut().for_each(|e| println!("{e:?}"));
}
```

### into_iter

```rust
fn main() {
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    map.into_iter().for_each(|e| println!("{e:?}"));
}
```

### len

```rust
fn main() {
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    println!("{:?}", map.len());
}
```

### is_empty

```rust
fn main() {
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    println!("{:?}", map.is_empty());
}
```

### drain

```rust
fn main() {
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    map.drain().for_each(|e| println!("{e:?}"));
}
```

### retain

```rust
fn main() {
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    map.retain(|_, v| *v % 2 == 0);
    println!("{map:?}");
}
```

### clear

```rust
fn main() {
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    map.clear();
    println!("{map:?}");
}
```

## 总结

了解了哈希映射中相关的一些方法。

## 附录
