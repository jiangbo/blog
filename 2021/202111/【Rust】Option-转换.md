# 【Rust】Option-转换

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/error/option_unwrap/map.html>  

## 示例

Option 有个 map 方法，可以进行值的映射，或者说转换。

### main.rs

```rust
#![allow(dead_code)]
#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Potato,
}

#[derive(Debug)]
struct Peeled(Food);
#[derive(Debug)]
struct Chopped(Food);
#[derive(Debug)]
struct Cooked(Food);

fn process(food: Option<Food>) -> Option<Cooked> {
    // 通过 map 进行转换
    food.map(Peeled)
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

// Check whether there's food or not before trying to eat it!
fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm. I love {:?}", food),
        None => println!("Oh no! It wasn't edible."),
    }
}

fn main() {
    let potato = None;
    // Let's try the simpler looking `process()` now.
    let cooked_potato = process(potato);
    eat(cooked_potato);
}
```

## 总结

了解了 Rust 中 Option 的 map 方法，可以将一个值映射（转换）成另一个值。

## 附录
