# 【Rust】数组切片（一）

## 环境

- Time 2022-02-25
- Rust 1.59.0

## 概念

引用数组中连续的一部分。

## 示例

### 二分查找-命中

如果查找到了，返回 Ok 和找到的位置索引。

```rust
fn main() {
    let arr = [1, 2, 3, 4];
    let result = arr.binary_search(&2);
    match result {
        Ok(index) => println!("find index: {index}"),
        Err(index) => println!("not find index: {index}"),
    }
}
```

### 二分查找-未命中

如果未查找到，返回 Err 和应该插入的位置索引。

```rust
fn main() {
    let arr = [1, 2, 3, 4];
    let result = arr.binary_search(&0);
    match result {
        Ok(index) => println!("find index: {index}"),
        Err(index) => println!("not find index: {index}"),
    }
}
```

### 根据比较函数搜索

```rust
fn main() {
    let arr = [1, 2, 3, 4];
    let result = arr.binary_search_by(|v| v.cmp(&2));
    match result {
        Ok(index) => println!("find index: {index}"),
        Err(index) => println!("not find index: {index}"),
    }
}
```

### 根据自定义属性搜索

```rust
fn main() {
    let arr = [(1, 10), (2, 20), (3, 30), (4, 40)];
    let result = arr.binary_search_by_key(&20, |&(_, b)| b);
    match result {
        Ok(index) => println!("find index: {index}"),
        Err(index) => println!("not find index: {index}"),
    }
}
```

## 总结

了解了数组切片中二分查找的使用方式。

## 附录
