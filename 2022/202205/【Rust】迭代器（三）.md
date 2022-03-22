# 【Rust】迭代器（三）

## 环境

- Time 2022-03-22
- Rust 1.59.0

## 示例

### take_while

在找到返回 false 的值后，忽略剩余所有元素。

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    vec.iter()
        .take_while(|&e| e % 2 == 0)
        .for_each(|e| println!("{e:?}"));
}
```

### take

获取几个元素。

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    vec.iter().take(2).for_each(|e| println!("{e:?}"));
}
```

### map_while

在找到返回 false 的值后，忽略剩余所有元素。

```rust
fn main() {
    let vec = vec!["0", "1", "two", "NaN", "four", "4"];
    vec.iter()
        .map_while(|e| e.parse().ok())
        .for_each(|e: i32| println!("{e:?}"));
}
```

### scan

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    vec.iter()
        .scan(44, |e, &i| {
            *e += i;
            Some(*e)
        })
        .for_each(|e| println!("{e:?}"));
}
```

### flat_map

```rust
fn main() {
    let vec = ["jiang", "bo", "44"];
    vec.iter()
        .flat_map(|s| s.chars())
        .for_each(|c| println!("{c}"));
}
```

### flatten

```rust
fn main() {
    let vec = vec![vec!["jiang", "bo"], vec!["44"]];
    vec.iter().flatten().for_each(|e| println!("{e}"));
}
```

### inspect

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    vec.iter()
        .inspect(|e| println!("before: {e:?}"))
        .filter(|&e| e % 2 == 0)
        .for_each(|e| println!("{e}"));
}
```

### by_ref

```rust
fn main() {
    let mut words = vec!["hello", "world", "Rust"].into_iter();
    words.by_ref().take(2).for_each(|e| println!("{e}"));
    words.for_each(|e| println!("{e}"));
}
```

## 总结

了解了迭代器中相关的一些方法。

## 附录
