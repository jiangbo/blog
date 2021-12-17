# 【Rust】Option-可选

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html>  

## 示例

除了使用恐慌，还可以使用可选（Option）来处理一些意外情况。

### main.rs

```rust
fn give_adult(drink: Option<&str>) {
    // 推荐使用这种
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary."),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No drink? Oh well."),
    }
}

fn drink(drink: Option<&str>) {
    // unwrap 展开，如果没有，则会恐慌
    let inside = drink.unwrap();
    if inside == "lemonade" {
        panic!("AAAaaaaa!!!!");
    }
    println!("I love {}s!!!!!", inside);
}

fn main() {
    let water = Some("water");
    let lemonade = Some("lemonade");
    let void = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink(coffee);
    // 恐慌
    // drink(nothing);
}
```

## 总结

了解了 Rust 中 Option 和 panic，如果直接展开空的 Option，则会引起恐慌。

## 附录
