# 【Rust】格式化输出

## 环境

- Rust 1.54.0
- VSCode 1.59.1

## 参考文档

<https://doc.rust-lang.org/stable/rust-by-example/hello/print.html>

## 示例

其中 `println!` 和 `print!` 类似，只是多 ln 的会多一个换行，会输出到标准输出流。  
`eprint!` 和 `eprintln!` 会将内容输出到标准错误流。

### 换行

```rust
fn main() {
    println!();
}
```

### 普通文本

```rust
fn main() {
    println!("Hello World!"); // Hello World!
}
```

### 参数文本

其中 `{}` 会被后面的参数 31 替换，如果多个 `{}` 则依次替换后面的参数。

```rust
fn main() {
    println!("{} days", 31); // 31 days
    println!("{} {}", 31, "days"); // 31 days
}
```

### 位置参数

```rust
fn main() {
    println!("{0}-{1}-{1}-{0}", "A", "B"); // A-B-B-A
}
```

### 命名参数

```rust
fn main() {
    println!("name:{name}, age:{age}", age = 44, name = "jiangbo");
}
```

### 进制转换

```rust
fn main() {
    println!("十进制：{}", 63); // 十进制：63
    println!("二进制：{:b}", 63); // 二进制：111111
    println!("八进制：{:o}", 63); // 八进制：77
    println!("大写十六进制：{:X}", 63); // 大写十六进制：3F
    println!("小写十六进制：{:x}", 63); // 小写十六进制：3f
}
```

### 输出宽度

```rust
fn main() {
    // All of these print "Hello x    !"
    println!("Hello {:5}!", "x");
    println!("Hello {:1$}!", "x", 5);
    println!("Hello {1:0$}!", 5, "x");
    println!("Hello {:width$}!", "x", width = 5);
}
```

### 对齐和填充

- `<` 左对齐
- `^` 居中对齐
- `>` 右对齐

```rust
fn main() {
    println!("Hello {:<5}!", "x"); // Hello x    !
    println!("Hello {:-<5}!", "x"); // Hello x----!
    println!("Hello {:^5}!", "x"); // Hello   x  !
    println!("Hello {:>5}!", "x"); // Hello     x!
}
```

### 精度

```rust
fn main() {
    let pi = 3.141592;
    println!("{:.3}", pi); // 3.142
}
```

### 转义

```rust
fn main() {
    println!("Hello {{}}"); // Hello {}
    println!("{{ Hello"); // { Hello
}
```

## 总结

使用了 `println!` 来展示不同功能的格式化输出。

## 附录
