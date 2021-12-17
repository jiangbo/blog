# 【Rust】可变借用

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/scope/borrow/mut.html>  

## 示例

### main.rs

```rust
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` 是只读内存的引用
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}

// 可变借用
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    let immutabook = Book {
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // copy
    let mut mutabook = immutabook;
    // 不可变借用
    borrow_book(&immutabook);
    // 可变借用
    new_edition(&mut mutabook);

    // 编译错误，不可变变量不能进行可变借用
    // new_edition(&mut immutabook);
}
```

## 总结

了解了 Rust 中的可变借用和不可变借用。

## 附录
