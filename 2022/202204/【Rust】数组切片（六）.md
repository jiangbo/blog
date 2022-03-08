# 【Rust】数组切片（六）

## 环境

- Time 2022-03-07
- Rust 1.59.0

## 概念

数组切片是引用数组中连续的一部分。

## 示例

### repeat

对元素进行重复。

```rust
fn main() {
    let arr = [0, 1];
    let vec = arr.repeat(4);
    println!("{vec:?}"); //[0, 1, 0, 1, 0, 1, 0, 1]
}
```

### reverse

反转。

```rust
fn main() {
    let mut arr = [0, 1, 2, 3, 4];
    arr.reverse();
    println!("{arr:?}"); //[4, 3, 2, 1, 0]
}
```

### rotate_left

旋转和交换。

```rust
fn main() {
    let mut arr = [0, 1, 2, 3, 4];
    arr.rotate_left(2);
    println!("{arr:?}"); // [2, 3, 4, 0, 1]
}
```

### split

分割，可变版本 `split_mut`，`rsplit_mut`。

```rust
fn main() {
    let arr = [0, 1, 2, 3, 4];
    let split = arr.split(|n| n == &2);
    let rsplit = arr.rsplit(|n| n == &2);
    split.for_each(|e| println!("{e:?}"));
    rsplit.for_each(|e| println!("{e:?}"));
}
```

### partition_point

提供一个闭包获取满足条件的切分点，数组需要有序。

```rust
fn main() {
    let arr = [0, 1, 2, 3, 4];
    let i = arr.partition_point(|&x| x < 2);
    println!("{i}");
}
```

### splitn

分割，并且进行次数限制，可变版本 `splitn_mut`，`rsplitn_mut`。

```rust
fn main() {
    let arr = [0, 1, 2, 3, 4];
    for ele in arr.splitn(2, |&e| e % 2 != 0) {
        println!("{ele:?}")
    }
    for ele in arr.rsplitn(2, |&e| e % 2 != 0) {
        println!("{ele:?}")
    }
}
```

### split_at

根据位置分割，可变版本 `split_at_mut`。

```rust
fn main() {
    let arr = [0, 1, 2, 3, 4];
    let (left, right) = arr.split_at(2);
    println!("left: {left:?}, right: {right:?}")
}
```

## 总结

了解了数组切片中相关的一些方法。

## 附录
