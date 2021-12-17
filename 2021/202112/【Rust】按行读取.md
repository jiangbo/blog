# 【Rust】按行读取

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html>  

## 示例

### main.rs

```rust
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/main.rs") {
        lines.for_each(|line| {
            if let Ok(line) = line {
                println!("{}", line);
            }
        });
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
```

## 总结

了解了 Rust 中读取文件中每一行内容的方法。

## 附录
