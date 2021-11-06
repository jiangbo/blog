# 【Rust】闭包-捕获变量

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/fn/closures/capture.html>  

## 示例

### 捕获引用

```rust
fn main() {
    let color = String::from("green");

    // 闭包借用 color 变量
    let print = || println!("`color`: {}", color);
    print();

    // 可以再次不可变借用，因为闭包只捕获了不可用引用。
    let _reborrow = &color;
    print();

    // 闭包使用完成后，变量可以进行移动。
    let _color_moved = color;
}
```

### 捕获可变引用

```rust
fn main() {
    let mut count = 0;
    // 闭包需要使用 mut，因为里面有 mut count。
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    inc();

    // 因为上面的可变借用完了，所以这里可以再次使用可变借用
    let _count_reborrowed = &mut count;
}
```

### 捕获值

```rust
fn main() {
    let haystack = vec![1, 2, 3];
    // haystack 移动到了闭包里
    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // 上面已经发生了移动，所以这里不能再次使用了
    // println!("There're {} elements in vec", haystack.len());
}
```

### 只能调用一次的移动

```rust
fn main() {
    use std::mem;

    let movable = Box::new(3);
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    consume();
    // 编译错误，只能调用一次
    // consume();
}
```

## 总结

了解了 Rust 中的闭包捕获变量的几种方式。

## 附录
