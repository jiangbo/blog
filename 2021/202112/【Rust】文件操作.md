# 【Rust】文件操作

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/std_misc/fs.html>  

## 示例

### 创建文件夹

```rust
use std::fs;

fn main() {
    println!("`mkdir a`");
    if let Err(why) = fs::create_dir("a") {
        println!("! {:?}", why.kind())
    }
}
```

### 遍历文件夹

```rust
use std::fs;

fn main() {
    match fs::read_dir(".") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                println!("> {:?}", path.unwrap().path());
            }
        }
    }
}
```

### 删除文件

```rust
use std::fs;

fn main() {
    fs::remove_file("hello.txt").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}
```

### 删除文件夹

```rust
use std::fs;

fn main() {
    fs::remove_dir_all("test").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}
```

## 总结

了解了 Rust 中文件的常用操作。

## 附录
