# 【Rust】迭代器（二）

## 环境

- Time 2022-03-22
- Rust 1.59.0

## 示例

### for_each

和 for 循环类型，遍历每个元素。

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    vec.iter().for_each(|e| println!("{e}"));
}
```

### map

和 for_each 类型，遍历每个元素，并且可以返回值，类似转换。

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    vec.iter().map(|e| e * 2).for_each(|e| println!("{e}"));
}
```

### filter

根据条件过滤元素。

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    vec.iter()
        .filter(|&e| e % 2 == 0)
        .for_each(|e| println!("{e}"))
}
```

### filter_map

filter 和 map 的结合。

```rust
fn main() {
    let vec = vec!["0", "two", "NaN", "four", "4"];
    vec.iter()
        .filter_map(|e| e.parse().ok())
        .for_each(|e: i32| println!("{e}"));
}
```

### enumerate

获取每个元素的索引。

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    vec.iter().enumerate().for_each(|e| println!("{e:?}"));
}
```

### peekable

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    let mut iter = vec.iter().peekable();
    println!("{:?}", iter.peek());
    println!("{:?}", iter.next());
    println!("{:?}", iter.peek());
}
```

### skip_while

在找到返回 false 的值后，剩余的所有元素将返回。

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    vec.iter()
        .skip_while(|&e| e % 2 == 0)
        .for_each(|e| println!("{e:?}"));
}
```

### skip

跳过几个元素。

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    vec.iter().skip(2).for_each(|e| println!("{e:?}"));
}
```

## 总结

了解了迭代器中相关的一些方法。

## 附录
