# 【Rust】冒泡排序

## 环境

- Time 2022-03-28
- Rust 1.59.0

## 演示

思想：如果相邻的两个元素不符合顺序要求，则交换两者的位置。

动画来源：<https://visualgo.net/en/sorting?slide=1>
![冒泡排序][1]

## 示例

### 实现

```rust
fn bubble_sort(data: &mut [i32]) {
    let mut swapped = true;
    let mut len = data.len();
    while swapped && len > 1 {
        swapped = false;
        len -= 1;
        for i in 0..len {
            if data[i] > data[i + 1] {
                data.swap(i, i + 1);
                swapped = true;
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
    bubble_sort(&mut data);
    assert_eq!(data, vec![]);
}
```

### 单元素

```rust
#[test]
fn test_single() {
    let mut data = vec![44];
    bubble_sort(&mut data);
    assert_eq!(data, vec![44]);
}
```

### 多元素

```rust
#[test]
fn test_multi() {
    let mut data = vec![44, 55, 33, 22, 0, -44];
    bubble_sort(&mut data);
    assert_eq!(data, vec![-44, 0, 22, 33, 44, 55]);
}
```

### 性能

冒泡排序的时间复杂度是 n 平方，和标准库排序的性能比对（10W数据量）：

```text
sort                    time:   [73.043 us 73.879 us 74.926 us]
                        change: [+18.504% +22.872% +27.043%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 21 outliers among 100 measurements (21.00%)
  21 (21.00%) high severe
```

## 倒序

```rust
fn bubble_sort(data: &mut [i32]) {
    let mut swapped = true;
    let mut len = data.len();
    while swapped && len > 1 {
        swapped = false;
        len -= 1;
        for i in 0..len {
            // 只需要将比较运算符取反都就可以反向
            if data[i] < data[i + 1] {
                data.swap(i, i + 1);
                swapped = true;
            }
        }
    }
}
```

## 总结

实现了冒泡排序功能。

## 附录

[1]: images/bubble_sort.gif
