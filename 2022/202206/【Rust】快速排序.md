# 【Rust】快速排序

## 环境

- Time 2022-03-30
- Rust 1.59.0

## 演示

思想：选择一个中间值，不停交换左右两侧（和中间值比较大小），直到有序。

动画来源：<https://visualgo.net/en/sorting?slide=1>
![快速排序][1]

## 示例

### 实现

```rust
fn quick_sort(data: &mut [i32]) {
    quick_sort0(data, 1, data.len());
}

fn quick_sort0(data: &mut [i32], start: usize, end: usize) {
    if start < end {
        let left = partition(data, start, end);
        if data[left] < data[start - 1] {
            data.swap(left, start - 1);
        }

        quick_sort0(data, start, left);
        quick_sort0(data, left + 1, end);
    }
}

fn partition(data: &mut [i32], start: usize, end: usize) -> usize {
    let mut left = start;
    let mut right = end - 1;
    while left < right {
        while data[left] <= data[start - 1] && left < right {
            left += 1;
        }

        while data[right] >= data[start - 1] && left < right {
            right -= 1;
        }
        data.swap(left, right);
    }
    left
}
```

### 空元素

```rust
#[test]
fn test_empty() {
    let mut data = vec![];
    quick_sort(&mut data);
    assert_eq!(data, vec![]);
}
```

### 单元素

```rust
#[test]
fn test_single() {
    let mut data = vec![44];
    quick_sort(&mut data);
    assert_eq!(data, vec![44]);
}
```

### 多元素

```rust
#[test]
fn test_multi() {
    let mut data = vec![44, 55, 33, 22, 0, -44];
    quick_sort(&mut data);
    assert!(data.windows(2).all(|w| w[0] <= w[1]))
}
```

### 性能

快速排序的时间复杂度是 n 平方，和标准库排序的性能比对（1W数据量）：

```text
sort                    time:   [815.31 us 824.30 us 835.06 us]
                        change: [+145.10% +150.50% +155.47%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
```

## 总结

实现了快速排序功能。

## 附录

[1]: images/quick_sort.gif
