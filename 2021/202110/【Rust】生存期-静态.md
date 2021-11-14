# 【Rust】生存期-静态

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html>  

## 示例

静态生存期使用 `‘static` 表示，

### main.rs

```rust
// 静态生存期
static NUM: i32 = 18;

fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // 字面量也是静态生存期
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // 虽然变量不能访问，但是数据还在二进制数据中
    }

    {
        let lifetime_num = 9;
        let coerced_static = coerce_static(&lifetime_num);
        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}
```

## 总结

了解了 Rust 中的静态的生存期参数的标注。

## 附录
