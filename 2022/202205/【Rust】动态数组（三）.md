# 【Rust】动态数组（三）

## 环境

- Time 2022-03-16
- Rust 1.59.0

## 概念

动态数组分配在栈上，长度可以变化。

## 示例

### truncate

从给定的位置进行截取，后面的元素将被丢弃。

```rust
fn main() {
    let mut vec = vec![0, 1, 2, 3, 4];
    vec.truncate(2);
    println!("{vec:?}");
}
```

### drain

和 `truncate` 类似，不过不是丢弃后面的元素，而是返回。

```rust
fn main() {
    let mut vec = vec![0, 1, 2, 3, 4];
    let drain = vec.drain(2..);
    println!("{drain:?}");
}
```

### as_slice

从动态数组转为切片，和 `&s[..]` 等效，可变版本 `as_mut_slice`。

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    let slice = vec.as_slice();
    println!("{:?}", slice);
}
```

### as_ptr

转成原始指针，可变版本 `as_mut_ptr`。

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    let ptr = vec.as_ptr();

    for i in 0..vec.len() {
        unsafe {
            println!("{:?}", *ptr.add(i));
        }
    }
}
```

### set_len

一般在 FFI 方面使用居多。

```rust
fn main() {
    let mut vec = vec![0, 1, 2, 3, 4];
    unsafe {
        vec.set_len(7); // 超过了长度，未知输出
        println!("{vec:?}");
    }
}
```

### swap_remove

删除指定的元素并返回，然后将最后一个元素移动到删除元素的位置。

```rust
fn main() {
    let mut vec = vec![0, 1, 2, 3, 4];
    vec.swap_remove(2);
    println!("{vec:?}");
}
```

### remove

删除指定的元素并返回，然后之后的元素依次向前移动。

```rust
fn main() {
    let mut vec = vec![0, 1, 2, 3, 4];
    vec.remove(2);
    println!("{vec:?}");
}
```

## 总结

了解了动态数组中相关的一些方法。

## 附录
