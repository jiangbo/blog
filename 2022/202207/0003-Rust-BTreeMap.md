# 0003-Rust-BTreeMap

## 环境

- Time 2022-04-26
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构，未考虑实现性能。
B-树是一种多路搜索树，在标准库中已有相应的实现。

### 目标

简单使用 BTreeMap 的方法。

## 获取值

```rust
fn main() {
    let mut map = BTreeMap::new();
    map.insert("name", "JiangBo");
    // 不存在则会恐慌
    println!("{:?}", map["name"]);
}
```

## for 循环元组

```rust
fn main() {
    let mut map = BTreeMap::new();
    map.insert("name", "JiangBo");
    for ele in map {
        println!("{ele:?}");
    }
}
```

## for 循环解构

```rust
fn main() {
    let mut map = BTreeMap::new();
    map.insert("name", "JiangBo");
    for (k, v) in map {
        println!("{k:?},{v:?}");
    }
}
```

## entry

```rust
fn main() {
    let mut map = BTreeMap::new();
    map.insert("name", "JiangBo");
    map.entry("age").or_insert("44");
    println!("{map:?}");
}
```

## split_off

```rust
fn main() {
    let mut map = BTreeMap::new();
    map.insert(11, "11");
    map.insert(22, "22");
    map.insert(33, "33");
    println!("{:?}", map.split_off(&22));
}
```

## into_keys

获取值所有权：into_values。

```rust
fn main() {
    let mut map = BTreeMap::new();
    map.insert(11, "11");
    map.insert(22, "22");
    map.insert(33, "33");
    println!("{:?}", map.into_keys());
}
```

### keys

获取值：into_values。

```rust
fn main() {
    let mut map = BTreeMap::new();
    map.insert(11, "11");
    map.insert(22, "22");
    map.insert(33, "33");
    println!("{:?}", map.keys());
}
```

## 总结

了解了 `BTreeMap` 中包含的方法。

## 附录
