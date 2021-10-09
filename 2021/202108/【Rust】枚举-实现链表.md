# 【Rust】枚举-实现链表

## 环境

- Rust 1.55.0
- VSCode 1.59.1

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/custom_types/enum/testcase_linked_list.html>

使用枚举类型，实现一个单向的链表。Box::new 表示在堆上分配一块内存存储数据。

## 示例

```rust
use crate::List::{Cons, Nil};

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match self {
            Cons(_, tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match self {
            Cons(head, tail) => format!("{}, {}", head, tail.stringify()),
            Nil => format!("Nil"),
        }
    }
}

fn main() {
    let mut list = List::new();
    list = list.prepend(1).prepend(2).prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
```

## 总结

使用枚举类型，实现一个单向的链表。

## 附录
