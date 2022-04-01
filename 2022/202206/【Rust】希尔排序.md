# 【Rust】希尔排序

## 环境

- Time 2022-04-01
- Rust 1.59.0

## 演示

思想：将列表间隔的元素分别排序，然后缩小间隔，直接所有元素有序。

参考：<https://github.com/TheAlgorithms/Rust/blob/master/src/sorting/shell_sort.rs>

## 示例

### 实现

```rust
fn shell_sort(data: &mut [i32]) {
    let mut gap = data.len() / 2;
    while gap > 0 {
        for start in 0..gap {
            insertion(data, start, gap);
        }
        gap /= 2;
    }
}

fn insertion(data: &mut [i32], start: usize, gap: usize) {
    for i in ((start + gap)..data.len()).step_by(gap) {
        let (val_current, mut pos) = (data[i], i);
        while pos >= gap && data[pos - gap] > val_current {
            data[pos] = data[pos - gap];
            pos -= gap;
        }
        data[pos] = val_current;
    }
}
```

### 空元素

```rust
#[test]
fn test_empty() {
    let mut data = vec![];
    shell_sort(&mut data);
    assert_eq!(data, vec![]);
}
```

### 单元素

```rust
#[test]
fn test_single() {
    let mut data = vec![44];
    shell_sort(&mut data);
    assert_eq!(data, vec![44]);
}
```

### 多元素

```rust
#[test]
fn test_multi() {
    let mut data = vec![44, 55, 33, 22, 0, -44];
    shell_sort(&mut data);
    assert!(data.windows(2).all(|w| w[0] <= w[1]))
}
```

### 性能

希尔排序的时间复杂度是 nlogn，和标准库排序的性能比对（1W数据量）：

```text
sort                    time:   [1.0590 ms 1.0686 ms 1.0791 ms]
                        change: [+219.88% +225.35% +230.50%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 14 outliers among 100 measurements (14.00%)
  6 (6.00%) high mild
  8 (8.00%) high severe
```

## 总结

实现了希尔排序功能。

## 附录
