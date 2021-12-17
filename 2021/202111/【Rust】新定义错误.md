# 【Rust】新定义错误

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html>  

## 示例

### main.rs

```rust
type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug, Clone)]
struct DoubleError;

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        // 转化成自己的错误类型
        .ok_or(DoubleError)
        .and_then(|s| {
            s.parse::<i32>()
                // 转换成自己的错误类型
                .map_err(|_| DoubleError)
                .map(|i| 2 * i)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
```

## 总结

了解了 Rust 中，可以定义一种新的错误类型，然后将所有不一致的类型都转为自己定义的类型。

## 附录
