# 【Rust】借用

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/scope/borrow.html>  

## 示例

通过引用来传递对象，在 rust 中称为借用。

### main.rs

```rust
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() {
    // 堆上分配
    let boxed_i32 = Box::new(5_i32);
    // 自动解引用
    borrow_i32(&boxed_i32);
    {
        // 自动解引用
        let _ref_to_i32: &i32 = &boxed_i32;

        // 编译错误，后面还有借用，不能取得所有权
        // eat_box_i32(boxed_i32);

        // 如果注释掉这句，那么前面的移动就是合法的
        borrow_i32(_ref_to_i32);
    }

    // 没有任何引用和借用了，可以进行移动。
    eat_box_i32(boxed_i32);
}
```

## 总结

了解了 Rust 中的移动和借用，移动会取得所有权，而借用不会。

## 附录
