# 0006-Rust-BTreeSet

## 环境

- Time 2022-04-26
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构，未考虑实现性能。
B-树是一种多路搜索树，在标准库中已有相应的实现。

### 目标

简单使用 `BTreeSet` 的方法。

## remove

```rust
fn main() {
    let mut set = BTreeSet::from([0, 1, 2, 3, 4]);
    println!("{:?}", set.remove(&4));
}
```

## take

```rust
fn main() {
    let mut set = BTreeSet::from([0, 1, 2, 3, 4]);
    println!("{:?}", set.take(&4));
}
```

## retain

```rust
fn main() {
    let mut set = BTreeSet::from([0, 1, 2, 3, 4]);
    set.retain(|e| e % 2 == 0);
    println!("{:?}", set);
}
```

## append

```rust
fn main() {
    let mut set1 = BTreeSet::from([0, 1, 2, 3, 4]);
    let mut set2 = BTreeSet::from([5, 6, 2, 3, 4]);
    set1.append(&mut set2);
    println!("{:?}", set1);
}
```

## split_off

```rust
fn main() {
    let mut set = BTreeSet::from([0, 1, 2, 3, 4]);
    println!("{:?}", set.split_off(&2));
}
```

## iter

```rust
fn main() {
    let set = BTreeSet::from([0, 1, 2, 3, 4]);
    for ele in set {
        println!("{ele:?}");
    }
}
```

## len

```rust
fn main() {
    let set = BTreeSet::from([0, 1, 2, 3, 4]);
    println!("{:?}", set.len());
}
```

### is_empty

```rust
fn main() {
    let set = BTreeSet::from([0, 1, 2, 3, 4]);
    println!("{:?}", set.is_empty());
}
```

### replace

```rust
fn main() {
    let set1 = BTreeSet::from([0, 1, 2, 3, 4]);
    let set2 = BTreeSet::from([5, 6, 2, 3, 4]);
    println!("{:?}", set1.max(set2));
}
```

## 总结

了解了 `BTreeSet` 中包含的方法。

## 附录
