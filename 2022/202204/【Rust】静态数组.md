# 【Rust】静态数组

## 环境

- Time 2022-02-25
- Rust 1.59.0

## 概念

一个固定长度的集合，存储在栈上。

## 示例

### 单个初始化

```rust
fn main() {
    let arr = [0, 1, 2, 3];
    println!("{:?}", arr);
}
```

### 批量初始化

```rust
fn main() {
    let arr = [0; 4];
    println!("{:?}", arr);
}
```

### 下标访问

```rust
fn main() {
    let arr = [1, 2, 3, 4];
    println!("{:?}", arr[3]);
}
```

### 访问越界

```rust
fn main() {
    let arr = [1, 2, 3, 4];
    // index out of bounds: the length is 4 but the index is 4
    // println!("{:?}", arr[4]);
}
```

### 按值迭代

```rust
fn main() {
    let arr = [1, 2, 3, 4];
    for ele in arr {
        println!("{}", ele);
    }
}
```

### 生成新数组

请阅读 Doc 文档查看性能警告。

```rust
fn main() {
    let arr1 = [1, 2, 3, 4];
    let arr2 = arr1.map(|i| i * 2);
    println!("{:?}", arr1);
    println!("{:?}", arr2);
}
```

## 总结

了解了静态数组的基本使用方式。

## 附录
