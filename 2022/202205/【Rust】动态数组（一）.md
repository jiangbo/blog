# 【Rust】动态数组（一）

## 环境

- Time 2022-03-16
- Rust 1.59.0

## 概念

动态数组分配在栈上，长度可以变化。

## 示例

### new

新建一个动态数组，如果没有增加元素，不会分配堆空间。

```rust
fn main() {
    let vec: Vec<i32> = Vec::new();
    println!("{vec:?}");
}
```

### with_capacity

分配一个固定大小的空间。

```rust
fn main() {
    let mut vec = Vec::with_capacity(1);
    vec.push(4);
    println!("{vec:?}");
}
```

### vec

使用 vec 宏创建一个动态数组。

```rust
fn main() {
    let vec = vec![4];
    println!("{vec:?}");
}
```

### from_raw_parts

直接从另一个动态数组的原始信息创建。

```rust
use std::{mem, ptr};

fn main() {
    let v = vec![1, 2, 3];
    // 不会自动销毁
    let mut v = mem::ManuallyDrop::new(v);

    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();

    unsafe {
        for i in 0..len as isize {
            ptr::write(p.offset(i), 4 + i);
        }

        let rebuilt = Vec::from_raw_parts(p, len, cap);
        println!("{rebuilt:?}");
    }
}
```

### capacity

获取动态数组的容量。

```rust
fn main() {
    let mut vec = Vec::new();
    vec.push(0);
    let capacity = vec.capacity();
    println!("{capacity}");
}
```

## 总结

了解了动态数组中相关的一些方法。

## 附录
