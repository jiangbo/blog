# 【Rust】Option-展开

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/error/option_unwrap/question_mark.html>  

## 示例

使用问号进行展开，具有更好的可读性。

### main.rs

```rust
struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
}

impl Person {
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}

fn main() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
}
```

## 总结

了解了 Rust 中使用问号对 Option 进行展开，如果是空的，则直接返回空，如果不为空才获得值。

## 附录
