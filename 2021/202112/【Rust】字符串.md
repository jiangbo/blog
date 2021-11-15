# 【Rust】字符串

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/std/str.html>  

## 示例

rust 中有两种字符串，一种是分配在堆上的 String，另一种是字符串切片（&str）。

### main.rs

```rust
fn main() {
    let pangram = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort_unstable();
    chars.dedup();

    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(", ");
    }

    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    let alice = String::from("I like dogs");
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
}
```

## 总结

了解了 Rust 中字符串，以及一些常用的方法。

## 附录
