# 【Rust】动态数组（四）

## 环境

- Time 2022-03-16
- Rust 1.59.0

## 概念

动态数组分配在栈上，长度可以变化。

## 示例

### insert

在给定的位置进行插入元素。

```rust
fn main() {
    let mut vec = vec![0, 1, 2, 3, 4];
    vec.insert(2, 44);
    println!("{vec:?}");
}
```

### retain

保留满足闭包条件的元素，其它元素会被删除。

```rust
fn main() {
    let mut vec = vec![0, 1, 2, 3, 4];
    vec.retain(|e| e % 2 == 0);
    println!("{vec:?}");
}
```

### dedup_by_key

根据结果移除连续的同一元素。

```rust
fn main() {
    let mut vec = vec![0, 1, 2, 3, 4];
    vec.dedup_by_key(|&mut e| e / 2);
    println!("{vec:?}");
}
```

### dedup

移除连续的同一个元素。

```rust
fn main() {
    let mut vec = vec![0, 1, 1, 2, 3, 3, 4];
    vec.dedup();
    println!("{vec:?}");
}
```

### dedup_by

根据连续的两个元素来确定是否移除元素。

```rust
fn main() {
    let mut vec = vec![0, 1, 2, 3, 4];
    vec.dedup_by(|&mut a, &mut b| b + 1 == a);
    println!("{vec:?}");
}
```

### push

从末尾新增一个元素。

```rust
fn main() {
    let mut vec = vec![0, 1, 2, 3, 4];
    vec.push(44);
    println!("{vec:?}");
}
```

### pop

从末尾删除一个元素。

```rust
fn main() {
    let mut vec = vec![0, 1, 2, 3, 4];
    vec.pop();
    println!("{vec:?}");
}
```

## 总结

了解了动态数组中相关的一些方法。

## 附录
