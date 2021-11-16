# 【Rust】标准库-HashSet

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/std/hash/hashset.html>  

## 示例

### main.rs

```rust
use std::collections::HashSet;

fn main() {
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();
    assert!(a.insert(4));
    println!("A: {:?}", a);
    println!("B: {:?}", b);

    // 并集
    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());
    // 差集
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

    println!(
        "Intersection: {:?}",
        // 交集
        a.intersection(&b).collect::<Vec<&i32>>()
    );

    println!(
        "Symmetric Difference: {:?}",
        // 对称差
        a.symmetric_difference(&b).collect::<Vec<&i32>>()
    );
}
```

## 总结

了解了 Rust 中标准库中的 HashSet 的使用。

## 附录
