# 0283-KVS-实现 set 和 remove

## 环境

- Time 2022-12-11
- WSL-Ubuntu 22.04
- Rust 1.65.0

## 前言

### 说明

参考：<https://github.com/pingcap/talent-plan>

### 目标

在上一节的基础上，实现 set 和 remove 的返回。

## Cargo.toml

```toml
[package]
edition = "2021"
name = "kvs"
version = "1.0.0"

[dependencies]
clap = {version = "4", features = ["derive"]}
serde = {version = "1", features = ["derive"]}
serde_json = "1"
```

## cmd.rs

```Rust
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Command<'a> {
    Set { key: String, value: String },
    Remove { key: &'a str },
}
```

## set 方法

```Rust
pub (crate) fn set(&mut self, key: String, value: String) -> Result<Option<String>> {
    let command = Command::Set {
        key: key.to_string(),
        value: value.to_string(),
    };
    let json = serde_json::to_string(&command)?;
    self.writer.write_all(json.as_bytes())?;
    Ok(self.map.insert(key, value))
}
```

## remove 方法

```Rust
pub(crate) fn remove(&mut self, key: String) -> Result<Option<String>> {
    let command = Command::Remove { key: &key };
    let json = serde_json::to_string(&command)?;
    self.writer.write_all(json.as_bytes())?;
    Ok(self.map.remove(&key))
}
```

## 运行

使用 `cargo build --release` 构建后运行。

```text
root@jiangbo12490:~/git/game/target/release# ./kvs set name JiangBo
SET KEY: name, VALUE: None
root@jiangbo12490:~/git/game/target/release# ./kvs set name ZhangSan
SET KEY: name, VALUE: JiangBo
root@jiangbo12490:~/git/game/target/release# ./kvs rm name
REM KEY: name, VALUE: ZhangSan
```

## 查看文件

```text
root@jiangbo12490:~/log# cat /root/log/1.log
{"Set":{"key":"name","value":"JiangBo"}}{"Set":{"key":"name","value":"ZhangSan"}}{"Remove":{"key":"name"}}
```

## 总结

实现了 set 和 remove 时返回之前已经存在的值。

## 附录
