# 0278-KVS-将命令转为 JSON

## 环境

- Time 2022-12-11
- WSL-Ubuntu 22.04
- Rust 1.65.0

## 前言

### 说明

参考：<https://github.com/pingcap/talent-plan>

### 目标

在上一节的基础上，将 set 和 remove 操作封装为单独的命令，并转为 JSON。

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

## 命令

```Rust
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}
```

## lib.rs

```Rust
use std::collections::HashMap;

use cmd::Command;

mod cmd;

#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    pub fn get(&mut self, key: &str) -> Option<String> {
        self.map.get(key).map(String::to_string)
    }

    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        let command = Command::Set {
            key: key.to_string(),
            value: value.to_string(),
        };
        let json = serde_json::to_string(&command).unwrap();
        dbg!(json);
        self.map.insert(key, value)
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        let command = Command::Remove {
            key: key.to_string(),
        };
        let json = serde_json::to_string(&command).unwrap();
        dbg!(json);
        self.map.remove(key)
    }
}
```

## 运行

使用 `cargo build --release` 构建后运行。

```text
root@jiangbo12490:~/git/game/target/release# ./kvs set name JiangBo
[src/lib.rs:23] json = "{\"Set\":{\"key\":\"name\",\"value\":\"JiangBo\"}}"
SET KEY: name, VALUE: JiangBo
root@jiangbo12490:~/git/game/target/release# ./kvs set age 44
[src/lib.rs:23] json = "{\"Set\":{\"key\":\"age\",\"value\":\"44\"}}"
SET KEY: age, VALUE: 44
root@jiangbo12490:~/git/game/target/release# ./kvs get name
GET KEY: name, VALUE: None
root@jiangbo12490:~/git/game/target/release# ./kvs rm name
[src/lib.rs:32] json = "{\"Remove\":{\"key\":\"name\"}}"
REM KEY: name, VALUE: None
```

## 总结

将对键值操作的命令，转为 JSON。

## 附录
