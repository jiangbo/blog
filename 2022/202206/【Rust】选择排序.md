# 【Rust】选择排序

## 环境

- Time 2022-03-29
- Rust 1.59.0

## 演示

思想：在每一次遍历的过程中，找当前最大或者最小的，然后进行位置交换。

动画来源：<https://visualgo.net/en/sorting?slide=1>
![选择排序][1]

## 示例

### 实现

```rust
fn selection_sort(data: &mut [i32]) {
    let mut len = data.len();
    while len > 1 {
        let mut pos_max = 0;
        for i in 1..len {
            if data[i] > data[pos_max] {
                pos_max = i;
            }
        }
        len -= 1;
        data.swap(pos_max, len);
    }
}
```

### 空元素

```rust
#[test]
fn test_empty() {
    let mut data = vec![];
    selection_sort(&mut data);
    assert_eq!(data, vec![]);
}
```

### 单元素

```rust
#[test]
fn test_single() {
    let mut data = vec![44];
    selection_sort(&mut data);
    assert_eq!(data, vec![44]);
}
```

### 多元素

```rust
#[test]
fn test_multi() {
    let mut data = vec![44, 55, 33, 22, 0, -44];
    bubble_sort(&mut data);
    assert!(data.windows(2).all(|w| w[0] <= w[1]))
}
```

### 性能

选择排序的时间复杂度是 n 平方，和冒泡相比减少交换次数，和标准库排序的性能比对（1W数据量）：

```text
sort                    time:   [61.144 ms 62.507 ms 63.959 ms]
                        change: [+17423% +17920% +18474%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
```

## 总结

实现了选择排序功能。

## 附录

[1]: images/selection_sort.gif
