# 0282-KVS-使用 anyhow 处理错误

## 环境

- Time 2022-12-11
- WSL-Ubuntu 22.04
- Rust 1.65.0

## 前言

### 说明

参考：<https://github.com/pingcap/talent-plan>

### 目标

在上一节的基础上，新增 anyhow 依赖，处理代码中的错误。

## Cargo.toml

```toml
[package]
edition = "2021"
name = "kvs"
version = "1.0.0"

[dependencies]
anyhow = "1"
clap = {version = "4", features = ["derive"]}
serde = {version = "1", features = ["derive"]}
serde_json = "1"
```

## main.rs

```Rust
use clap::{Parser, Subcommand};
use kvs::KvStore;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 设置值
    Set {
        /// 键
        key: String,
        /// 值
        value: String,
    },
    /// 获取值
    Get {
        /// 键
        key: String,
    },
    /// 删除值
    Rm {
        /// 键
        key: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut store = KvStore::new()?;
    match cli.command {
        Commands::Set { key, value } => {
            if let Some(value) = store.set(key.to_string(), value)? {
                println!("SET KEY: {key}, VALUE: {value}");
            } else {
                println!("SET KEY: {key}, VALUE: None");
            }
        }
        Commands::Get { key } => {
            if let Some(value) = store.get(&key)? {
                println!("GET KEY: {key}, VALUE: {value}");
            } else {
                println!("GET KEY: {key}, VALUE: None");
            }
        }
        Commands::Rm { key } => {
            if let Some(value) = store.remove(key.to_string())? {
                println!("REM KEY: {key}, VALUE: {value}");
            } else {
                println!("REM KEY: {key}, VALUE: None");
            }
        }
    }
    Ok(())
}
```

## lib.rs

```Rust
use anyhow::Result;
use log::CommandLog;

mod cmd;
mod log;

pub struct KvStore {
    log: CommandLog,
}

impl KvStore {
    pub fn new() -> Result<KvStore> {
        Ok(Self {
            log: CommandLog::new()?,
        })
    }

    pub fn get(&mut self, key: &str) -> Result<Option<String>> {
        self.log.get(key)
    }

    pub fn set(&mut self, key: String, value: String) -> Result<Option<String>> {
        self.log.set(key, value)
    }

    pub fn remove(&mut self, key: String) -> Result<Option<String>> {
        self.log.remove(key)
    }
}
```

## log.rs

```Rust
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};

use anyhow::{Ok, Result};
use serde_json::Deserializer;

use crate::cmd::Command;

pub(crate) struct CommandLog {
    writer: BufWriter<File>,
    map: HashMap<String, String>,
}

impl CommandLog {
    pub(crate) fn new() -> Result<Self> {
        let path = "/root/log/1.log";
        let writer = new_writer(path)?;
        let mut map = HashMap::default();
        load(path, &mut map)?;
        Ok(Self { writer, map })
    }

    pub(crate) fn get(&mut self, key: &str) -> Result<Option<String>> {
        Ok(self.map.get(key).map(String::from))
    }

    pub(crate) fn set(&mut self, key: String, value: String) -> Result<Option<String>> {
        let command = Command::Set { key, value };
        let json = serde_json::to_string(&command)?;
        self.writer.write_all(json.as_bytes())?;
        Ok(None)
    }

    pub(crate) fn remove(&mut self, key: String) -> Result<Option<String>> {
        let command = Command::Remove { key };
        let json = serde_json::to_string(&command)?;
        self.writer.write_all(json.as_bytes())?;
        Ok(None)
    }
}

fn new_writer(path: &str) -> Result<BufWriter<File>> {
    let file = OpenOptions::new().append(true).create(true).open(path)?;
    Ok(BufWriter::new(file))
}

fn load(path: &str, map: &mut HashMap<String, String>) -> Result<()> {
    let stream = Deserializer::from_reader(new_reader(path)?).into_iter();
    for cmd in stream {
        match cmd? {
            Command::Set { key, value } => map.insert(key, value),
            Command::Remove { key } => map.remove(&key),
        };
    }
    Ok(())
}

fn new_reader(path: &str) -> Result<BufReader<File>> {
    let file = OpenOptions::new().read(true).open(path)?;
    Ok(BufReader::new(file))
}
```

## 总结

使用 anyhow 来处理程序中遇到的错误。

## 附录
