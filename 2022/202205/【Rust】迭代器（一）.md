# 【Rust】迭代器（一）

## 环境

- Time 2022-03-22
- Rust 1.59.0

## 示例

### iter

生成一个迭代器，可变版本 `iter_mut`，所有权版本 `into_iter`。

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    let iter = vec.iter();
    println!("{:?}", iter);
}
```

### next

获取下一个元素，没有返回 None。

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    let mut iter = vec.iter();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
}
```

### count

统计迭代器中的元素的数量。

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    println!("{:?}", vec.iter().count()); // 可以直接使用 len 方法
}
```

### last

获取最后一个元素。

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    println!("{:?}", vec.iter().last());
}
```

### nth

获取第几个元素。

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    println!("{:?}", vec.iter().nth(2)); // get 方法可以替代
}
```

### step_by

设置迭代步长。

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    let mut iter = vec.iter().step_by(2);
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
}
```

### chain

连接两个迭代器。

```rust
fn main() {
    let vec1 = vec![0, 1, 2, 3, 4];
    let vec2 = vec![5, 6, 7, 8, 9];
    let chain = vec1.iter().chain(&vec2);
    println!("{:?}", chain);
}
```

### zip

压缩两个迭代器。

```rust
fn main() {
    let vec1 = vec![0, 1, 2, 3, 4];
    let vec2 = vec![5, 6, 7, 8, 9];
    let zip = vec1.iter().zip(&vec2);
    for ele in zip {
        println!("{:?}", ele);
    }
}
```

## 总结

了解了迭代器中相关的一些方法。

## 附录
