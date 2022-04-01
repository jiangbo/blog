# 【Rust】基数排序

## 环境

- Time 2022-04-01
- Rust 1.59.0

## 演示

思想：将列表中每个数按照基数拆分成不同的位数，然后分别对不同的位数分别进行排序。
>限制：不能有负数，最大值不能太大。

动画来源：<https://visualgo.net/en/sorting?slide=1>
![基数排序][1]

## 示例

### 实现

```rust
fn radix_sort(data: &mut [i32]) {
    if let Some(&max) = data.iter().max() {
        let radix = data.len().next_power_of_two();
        let mut place = 1;
        while place <= max {
            let digit_of = |x| (x / place) as usize % radix;
            let mut temp = vec![0; radix];
            for &x in data.iter() {
                temp[digit_of(x)] += 1;
            }
            for i in 1..radix {
                temp[i] += temp[i - 1];
            }
            for &x in data.to_owned().iter().rev() {
                temp[digit_of(x)] -= 1;
                data[temp[digit_of(x)]] = x;
            }
            place *= radix as i32;
        }
    }
}
```

### 空元素

```rust
#[test]
fn test_empty() {
    let mut data = vec![];
    radix_sort(&mut data);
    assert_eq!(data, vec![]);
}
```

### 单元素

```rust
#[test]
fn test_single() {
    let mut data = vec![44];
    radix_sort(&mut data);
    assert_eq!(data, vec![44]);
}
```

### 多元素

```rust
#[test]
fn test_multi() {
    let mut data = vec![44, 55, 33, 22, 0];
    radix_sort(&mut data);
    assert!(data.windows(2).all(|w| w[0] <= w[1]))
}
```

### 性能

基数排序的时间复杂度是 n * k，和标准库排序的性能比对（1W数据量）：

```text
sort                    time:   [177.60 us 179.72 us 182.09 us]
                        change: [-47.019% -45.842% -44.558%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
```

## 总结

实现了基数排序功能。

## 附录

[1]: images/radix_sort.gif
