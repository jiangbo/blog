# 【Rust】计数排序

## 环境

- Time 2022-03-31
- Rust 1.59.0

## 演示

思想：定义个长度为列表中最大值的数组，然后将列表中每个值对应到数组的索引上。
>限制：不能有负数，最大值不能太大。

动画来源：<https://visualgo.net/en/sorting?slide=1>
![计数排序][1]

## 示例

### 实现

```rust
fn count_sort(data: &mut [i32]) {
    if let Some(max) = data.iter().max().map(|e| (e + 1) as usize) {
        let mut temp = vec![0; max];
        data.iter().for_each(|&e| temp[e as usize] += 1);
        let mut i = 0;
        for (val, &num) in temp.iter().enumerate() {
            for _ in 0..num {
                data[i] = val as i32;
                i += 1;
            }
        }
    }
}
```

### 空元素

```rust
#[test]
fn test_empty() {
    let mut data = vec![];
    count_sort(&mut data);
    assert_eq!(data, vec![]);
}
```

### 单元素

```rust
#[test]
fn test_single() {
    let mut data = vec![44];
    count_sort(&mut data);
    assert_eq!(data, vec![44]);
}
```

### 多元素

```rust
#[test]
fn test_multi() {
    let mut data = vec![44, 55, 33, 22, 0, -44];
    count_sort(&mut data);
    assert!(data.windows(2).all(|w| w[0] <= w[1]))
}
```

### 性能

计数排序的时间复杂度是 n + k，和标准库排序的性能比对（1W数据量）：

```text
sort                    time:   [87.390 us 88.532 us 89.848 us]
                        change: [-72.999% -72.325% -71.626%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  7 (7.00%) high mild
  6 (6.00%) high severe
```

## 总结

实现了计数排序功能。

## 附录

[1]: images/count_sort.gif
