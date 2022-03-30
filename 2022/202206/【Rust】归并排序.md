# 【Rust】归并排序

## 环境

- Time 2022-03-30
- Rust 1.59.0

## 演示

思想：将大列表分割成小列表，在将小列表排序后组合成大列表再次排序。

动画来源：<https://visualgo.net/en/sorting?slide=1>
![归并排序][1]

## 示例

### 递归实现

```rust
fn merge_sort(data: &mut [i32]) {
    if data.len() > 1 {
        let mid = data.len() / 2;
        sort(&mut data[..mid]);
        sort(&mut data[mid..]);
        merge(data, mid)
    }
}

fn merge(data: &mut [i32], mid: usize) {
    let left = data[..mid].to_vec();
    let right = data[mid..].to_vec();

    let mut l = 0;
    let mut r = 0;

    for v in data {
        if r == right.len() || (l < left.len() && left[l] < right[r]) {
            *v = left[l];
            l += 1;
        } else {
            *v = right[r];
            r += 1;
        }
    }
}
```

### 迭代实现

```rust
fn merge_sort(data: &mut [i32]) {
    let mut step = 2;
    while step < data.len() {
        data.chunks_mut(step).for_each(|sub| merge(sub, step / 2));
        step *= 2;
    }
    data.chunks_mut(step).for_each(|sub| merge(sub, step / 2));
}

fn merge(data: &mut [i32], mid: usize) {
    let mid = mid.min(data.len());
    let left = data[..mid].to_vec();
    let right = data[mid..].to_vec();

    let mut l = 0;
    let mut r = 0;

    for v in data.iter_mut() {
        if r == right.len() || (l < left.len() && left[l] < right[r]) {
            *v = left[l];
            l += 1;
        } else {
            *v = right[r];
            r += 1;
        }
    }
}
```

### 空元素

```rust
#[test]
fn test_empty() {
    let mut data = vec![];
    merge_sort(&mut data);
    assert_eq!(data, vec![]);
}
```

### 单元素

```rust
#[test]
fn test_single() {
    let mut data = vec![44];
    merge_sort(&mut data);
    assert_eq!(data, vec![44]);
}
```

### 多元素

```rust
#[test]
fn test_multi() {
    let mut data = vec![44, 55, 33, 22, 0, -44];
    merge_sort(&mut data);
    assert!(data.windows(2).all(|w| w[0] <= w[1]))
}
```

### 性能

归并排序的时间复杂度是 nlogn，和标准库排序的性能比对（1W数据量）：

```text
sort                    time:   [1.7945 ms 1.8228 ms 1.8532 ms]
                        change: [+453.55% +468.25% +482.91%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe
```

## 总结

实现了归并排序功能。

## 附录

[1]: images/merge_sort.gif
