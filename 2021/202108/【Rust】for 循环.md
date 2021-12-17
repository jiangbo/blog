# 【Rust】for 循环

## 环境

- Rust 1.56.1
- VSCode 1.60.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/flow_control/for.html>  

for 循环可以遍历一个迭代器。遍历迭代器有三种方式：into_iter，iter 和 iter_mut。

## 示例

### 区间

生成迭代器的简单方法是使用区间，默认情况下是左闭右开的，可以使用 `=` 来使右闭。

```rust
fn main() {
    // .. 生成一个区间
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // .. 生成一个区间，= 号可以取到右边的值
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
```

### into_iter 迭代

如果没有指明，默认使用 `into_iter` 迭代。使用这种迭代方式会消耗集合，循环完后，集合不能再被使用。

```rust
fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // println!("names: {:?}", names);
    // 集合已经被消耗了，不能使用了。
}
```

### iter 迭代

使用这种迭代只会借用元素，循环完成后，集合还可以正常使用。

```rust
fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match *name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);
}
```

### iter_mut 迭代

可变借用，可以修改集合中的元素。

```rust
fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match *name {
            "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);
}
```

## 总结

了解了 Rust 中的 for 循环，可以用来遍历迭代器，有三种不同的迭代方法。

## 附录
