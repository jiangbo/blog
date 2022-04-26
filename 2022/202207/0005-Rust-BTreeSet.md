# 0005-Rust-BTreeSet

## 环境

- Time 2022-04-26
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构，未考虑实现性能。
B-树是一种多路搜索树，在标准库中已有相应的实现。

### 目标

简单使用 `BTreeSet` 的方法。

## union

```rust
fn main() {
    let set1 = BTreeSet::from([0, 1, 2, 3, 4]);
    let set2 = BTreeSet::from([5, 6, 2, 3, 4]);
    for ele in set1.union(&set2) {
        println!("{ele}");
    }
}
```

## or

```rust
fn main() {
    let set1 = BTreeSet::from([0, 1, 2, 3, 4]);
    let set2 = BTreeSet::from([5, 6, 2, 3, 4]);
    for ele in &set1 | &set2 {
        println!("{ele}");
    }
}
```

## clear

```rust
fn main() {
    let mut set = BTreeSet::from([0, 1, 2, 3, 4]);
    set.clear();
    println!("{set:?}");
}
```

## contains

```rust
fn main() {
    let set = BTreeSet::from([0, 1, 2, 3, 4]);
    println!("{:?}", set.contains(&4));
}
```

## get

```rust
fn main() {
    let set = BTreeSet::from([0, 1, 2, 3, 4]);
    println!("{:?}", set.get(&4));
}
```

## is_disjoint

```rust
fn main() {
    let set1 = BTreeSet::from([0, 1, 2, 3, 4]);
    let set2 = BTreeSet::from([5, 6, 2, 3, 4]);
    println!("{:?}", set1.is_disjoint(&set2));
}
```

## is_subset

```rust
fn main() {
    let set1 = BTreeSet::from([0, 1, 2, 3, 4]);
    let set2 = BTreeSet::from([5, 6, 2, 3, 4]);
    println!("{:?}", set1.is_subset(&set2));
}
```

### is_superset

```rust
fn main() {
    let set1 = BTreeSet::from([0, 1, 2, 3, 4]);
    let set2 = BTreeSet::from([5, 6, 2, 3, 4]);
    println!("{:?}", set1.is_superset(&set2));
}
```

### replace

```rust
fn main() {
    let mut set = BTreeSet::from([0, 1, 2, 3, 4]);
    println!("{:?}", set.replace(4));
}
```

## 总结

了解了 `BTreeSet` 中包含的方法。

## 附录
