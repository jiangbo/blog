# 【Rust】插入排序

## 环境

- Time 2022-03-29
- Rust 1.59.0

## 演示

思想：依次遍历元素并将其插入到已排序的部分中，直到所有元素有序。

动画来源：<https://visualgo.net/en/sorting?slide=1>
![插入排序][1]

## 示例

### 实现

```rust
fn insertion_sort(data: &mut [i32]) {
    for i in 1..data.len() {
        let current = data[i];
        let mut pos = i;
        while pos > 0 && current < data[pos - 1] {
            data[pos] = data[pos - 1];
            pos -= 1;
        }
        data[pos] = current;
    }
}
```

### 优化实现

```rust
fn insertion_sort(data: &mut [i32]) {
    for i in 1..data.len() {
        let current = data[i];
        let pos = data[..i].binary_search(&current).unwrap_or_else(|pos| pos);
        let mut j = i;
        while j > pos {
            data[j] = data[j - 1];
            j -= 1;
        }
        data[pos] = current;
    }
}
```

### 空元素

```rust
#[test]
fn test_empty() {
    let mut data = vec![];
    insertion_sort(&mut data);
    assert_eq!(data, vec![]);
}
```

### 单元素

```rust
#[test]
fn test_single() {
    let mut data = vec![44];
    insertion_sort(&mut data);
    assert_eq!(data, vec![44]);
}
```

### 多元素

```rust
#[test]
fn test_multi() {
    let mut data = vec![44, 55, 33, 22, 0, -44];
    insertion_sort(&mut data);
    assert!(data.windows(2).all(|w| w[0] <= w[1]))
}
```

### 性能

插入排序的时间复杂度是 n 平方，和标准库排序的性能比对（1W数据量）：

```text
sort                    time:   [20.383 ms 20.884 ms 21.432 ms]
                        change: [+6065.3% +6225.8% +6422.1%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  8 (8.00%) high mild
  1 (1.00%) high severe
```

## 总结

实现了插入排序功能。

## 附录

[1]: images/insertion_sort.gif
