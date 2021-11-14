# 【Rust】trait-消除歧义

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/trait/disambiguating.html>  

## 示例

如果有两个同名的方法，这时调用的时候，需要使用指定调用的类型。

### 静态返回

```rust
trait UsernameWidget {
    fn get(&self) -> String;
}

trait AgeWidget {
    fn get(&self) -> u8;
}

struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn main() {
    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };
    // 指定类型调用
    let username = UsernameWidget::get(&form);
    assert_eq!("rustacean".to_owned(), username);
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);
}
```

## 总结

了解了 Rust 中出现同名方法时，怎么调用特定的某一个方法。

## 附录
