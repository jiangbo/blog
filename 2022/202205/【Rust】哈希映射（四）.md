# 【Rust】哈希映射（四）

## 环境

- Time 2022-03-24
- Rust 1.59.0

## 示例

### get_mut

```rust
fn main() {
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    if let Some(v) = map.get_mut("c") {
        *v += 3
    }
    println!("{:?}", map.get("c"));
}
```

### insert

```rust
fn main() {
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    map.insert("d", 4);
    println!("{:?}", map);
}
```

### remove

```rust
fn main() {
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    map.remove("b");
    println!("{:?}", map);
}
```

### remove_entry

```rust
fn main() {
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    let entry = map.remove_entry("b");
    println!("{:?}", entry);
}
```

### or_insert

```rust
fn main() {
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    map.entry("d").or_insert(4);
    println!("{:?}", map);
}
```

### or_insert_with

```rust
fn main() {
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    map.entry("d").or_insert_with(|| 4);
    println!("{:?}", map);
}
```

### or_insert_with_key

```rust
fn main() {
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    map.entry("d").or_insert_with_key(|k| k.len());
    println!("{:?}", map);
}
```

### and_modify

```rust
fn main() {
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    map.entry("a").and_modify(|k| *k += 4);
    println!("{:?}", map);
}
```

### or_default

```rust
fn main() {
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    map.entry("d").or_default();
    println!("{:?}", map);
}
```

## 总结

了解了哈希映射中相关的一些方法。

## 附录
