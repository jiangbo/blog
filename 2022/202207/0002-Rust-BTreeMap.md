# 0002-Rust-BTreeMap

## 环境

- Time 2022-04-26
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构，未考虑实现性能。
B-树是一种多路搜索树，在标准库中已有相应的实现。

### 目标

简单使用 BTreeMap 的方法。

## contains_key

```rust
fn main() {
    let mut map = BTreeMap::new();
    map.insert("name", "JiangBo");
    println!("{:?}", map.contains_key("name"));
}
```

## remove

```rust
fn main() {
    let mut map = BTreeMap::new();
    map.insert("name", "JiangBo");
    println!("{:?}", map.remove("name"));
    println!("{:?}", map.remove("name"));
}
```

## remove_entry

```rust
fn main() {
    let mut map = BTreeMap::new();
    map.insert("name", "JiangBo");
    println!("{:?}", map.remove_entry("name"));
    println!("{:?}", map.remove_entry("name"));
}
```

## retain

```rust
fn main() {
    let mut map = BTreeMap::new();
    map.insert("name", "JiangBo");
    map.insert("age", "44");
    map.retain(|_, &mut v| v == "JiangBo");
    println!("{:?}", map);
}
```

## append

```rust
fn main() {
    let mut map = BTreeMap::new();
    map.insert("name", "JiangBo");
    map.insert("age", "44");

    let mut map1 = BTreeMap::new();
    map1.insert("lang", "Rust");
    map1.insert("study", "map");

    map.append(&mut map1);
    println!("{:?}", map);
}
```

## range

可变版本：range_mut。

```rust
fn main() {
    let mut map = BTreeMap::new();
    map.insert(11, "JiangBo");
    map.insert(4, "44");

    println!("{:?}", map.range(8..));
}
```

### len

```rust
fn main() {
    let mut map = BTreeMap::new();
    map.insert("name", "JiangBo");
    map.insert("age", "44");

    println!("{:?}", map.len());
}
```

## 总结

了解了 `BTreeMap` 中包含的方法。

## 附录
