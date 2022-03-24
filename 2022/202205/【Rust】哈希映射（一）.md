# 【Rust】哈希映射（一）

## 环境

- Time 2022-03-24
- Rust 1.59.0

## 示例

### new

```rust
fn main() {
    let map: HashMap<&str, i32> = HashMap::new();
    println!("{:?}", map.capacity());
}
```

### with_capacity

```rust
fn main() {
    let map: HashMap<&str, i32> = HashMap::with_capacity(4);
    println!("{:?}", map.capacity());
}
```

### with_hasher

```rust
fn main() {
    let s = RandomState::new();
    let map: HashMap<&str, i32> = HashMap::with_hasher(s);
    println!("{:?}", map.capacity());
}
```

### with_capacity_and_hasher

```rust
fn main() {
    let s = RandomState::new();
    let map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher(4, s);
    println!("{:?}", map.capacity());
}
```

### capacity

```rust
fn main() {
    let map: HashMap<&str, i32> = HashMap::with_capacity(4);
    println!("{:?}", map.capacity());
}
```

### keys

```rust
fn main() {
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    map.keys().for_each(|e| println!("{e:?}"));
}
```

### into_keys

```rust
fn main() {
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    map.into_keys().for_each(|e| println!("{e:?}"));
}
```

### values

```rust
fn main() {
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    map.values().for_each(|e| println!("{e:?}"));
}
```

### values_mut

```rust
fn main() {
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    map.values_mut().for_each(|e| println!("{e:?}"));
}
```

## 总结

了解了哈希映射中相关的一些方法。

## 附录
