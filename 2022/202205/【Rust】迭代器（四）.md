# 【Rust】迭代器（四）

## 环境

- Time 2022-03-23
- Rust 1.59.0

## 示例

### collect

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    let doubled: Vec<_> = vec.iter().map(|e| e * 2).collect();
    for ele in doubled {
        println!("{ele}");
    }
}
```

### partition

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    let (even, odd): (Vec<i32>, Vec<_>) = vec.iter().partition(|&e| e % 2 == 0);
    println!("even: {even:?}, odd: {odd:?}");
}
```

### try_fold

```rust
fn main() {
    let vec = vec![1, 11, 111, 111];
    let triangular = vec.iter().try_fold(0_i8, |prev, &e| {
        if let Some(next) = prev.checked_add(e) {
            ControlFlow::Continue(next)
        } else {
            ControlFlow::Break(prev)
        }
    });

    println!("{triangular:?}");
}
```

### fold

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];

    let result = vec
        .iter()
        .fold("-1".to_string(), |acc, x| format!("{}{}", acc, x));
    println!("{result}");
}
```

### reduce

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    let result = vec.into_iter().reduce(|acc, x| acc + x);
    println!("{result:?}");
}
```

### all

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    let result = vec.iter().all(|e| e < &10);
    println!("{result:?}");
}
```

### any

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    let result = vec.iter().any(|e| e < &0);
    println!("{result:?}");
}
```

### find

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    let result = vec.iter().find(|&&e| e > 2);
    println!("{result:?}");
}
```

## 总结

了解了迭代器中相关的一些方法。

## 附录
