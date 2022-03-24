# 【Rust】哈希映射（三）

## 环境

- Time 2022-03-24
- Rust 1.59.0

## 示例

### hasher

```rust
fn main() {
    let hasher = RandomState::new();
    let map: HashMap<i32, i32> = HashMap::with_hasher(hasher);
    println!("{:?}", map.hasher());
}
```

### reserve

```rust
fn main() {
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    map.reserve(4);
    println!("{:?}", map.capacity());
}
```

### try_reserve

```rust
fn main() {
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    map.try_reserve(4).unwrap();
    println!("{:?}", map.capacity());
}
```

### shrink_to_fit

```rust
fn main() {
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    map.shrink_to_fit();
    println!("{:?}", map.capacity());
}
```

### shrink_to

```rust
fn main() {
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    map.shrink_to(4);
    println!("{:?}", map.capacity());
}
```

### entry

```rust
fn main() {
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    println!("{:?}", map.entry("c"));
}
```

### get

```rust
fn main() {
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    println!("{:?}", map.get("c"));
}
```

### get_key_value

```rust
fn main() {
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    println!("{:?}", map.get_key_value("c"));
}
```

### contains_key

```rust
fn main() {
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    println!("{:?}", map.contains_key("c"));
}
```

## 总结

了解了哈希映射中相关的一些方法。

## 附录
