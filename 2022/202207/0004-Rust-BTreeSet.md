# 0004-Rust-BTreeSet

## 环境

- Time 2022-04-26
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构，未考虑实现性能。
B-树是一种多路搜索树，在标准库中已有相应的实现。

### 目标

简单使用 `BTreeSet` 的方法。

## new

```rust
fn main() {
    let mut set = BTreeSet::new();
    println!("{:?}", set);
}
```

## insert

```rust
fn main() {
    let mut set = BTreeSet::new();
    println!("{:?}", set.insert(4));
    println!("{:?}", set);
}
```

## from

```rust
fn main() {
    let set = BTreeSet::from([0, 1, 2, 3, 4]);
    println!("{set:?}");
}
```

## difference

```rust
fn main() {
    let set1 = BTreeSet::from([0, 1, 2, 3, 4]);
    let set2 = BTreeSet::from([5, 6, 2, 3, 4]);
    for ele in set1.difference(&set2) {
        println!("{ele}");
    }
}
```

## sub

```rust
fn main() {
    let set1 = BTreeSet::from([0, 1, 2, 3, 4]);
    let set2 = BTreeSet::from([5, 6, 2, 3, 4]);
    for ele in &set1 - &set2 {
        println!("{ele}");
    }
}
```

## symmetric_difference

```rust
fn main() {
    let set1 = BTreeSet::from([0, 1, 2, 3, 4]);
    let set2 = BTreeSet::from([5, 6, 2, 3, 4]);
    for ele in set1.symmetric_difference(&set2) {
        println!("{ele}");
    }
}
```

## xor

```rust
fn main() {
    let set1 = BTreeSet::from([0, 1, 2, 3, 4]);
    let set2 = BTreeSet::from([5, 6, 2, 3, 4]);
    for ele in &set1 ^ &set2 {
        println!("{ele}");
    }
}
```

### intersection

```rust
fn main() {
    let set1 = BTreeSet::from([0, 1, 2, 3, 4]);
    let set2 = BTreeSet::from([5, 6, 2, 3, 4]);
    for ele in set1.intersection(&set2) {
        println!("{ele}");
    }
}
```

### and

```rust
fn main() {
    let set1 = BTreeSet::from([0, 1, 2, 3, 4]);
    let set2 = BTreeSet::from([5, 6, 2, 3, 4]);
    for ele in &set1 & &set2 {
        println!("{ele}");
    }
}
```

## 总结

了解了 `BTreeSet` 中包含的方法。

## 附录
