# 0267-KVS-实现 KVS 的命令行

## 环境

- Time 2022-12-03
- WSL-Ubuntu 22.04
- CLAP 4.0.29

## 前言

### 说明

参考：<https://github.com/pingcap/talent-plan>

### 目标

KVS 的意思是 KEY VALUE STORE，创建一个可以使用 `set` `get` 和 `rm` 的命令行程序。

## Cargo.toml

```toml
[package]
edition = "2021"
name = "kvs"
version = "1.0.0"

[dependencies]
clap = {version = "4", features = ["derive"]}

[dev-dependencies]
assert_cmd = "2.0.6"
predicates = "2.1.3"
```

## lib.rs

```Rust
#![deny(missing_docs)]
//! 一个简单的键值存储库

pub use kv::KvStore;

mod kv;
```

## kv.rs

```Rust
use std::collections::HashMap;

/// 键值存储库
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```
#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// 创建一个 `KvStore`.
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    /// 设置 key value
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// 通过 key 获取值，如果没有返回 `None`
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    /// 删除 key 和对应的 value
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
```

## kvs.rs

```Rust
use clap::{Parser, Subcommand};
use std::process::exit;

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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Set { key, value } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Commands::Get { key } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Commands::Rm { key } => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
```

## 通过测试

```text
     Running unittests src/lib.rs (target/debug/deps/kvs-e4cdc5207c38343b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/kvs.rs (target/debug/deps/kvs-1cea8926ad7a73ad)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/tests.rs (target/debug/deps/tests-d6f6900368ceb861)

running 13 tests
test get_non_existent_value ... ok
test get_stored_value ... ok
test overwrite_value ... ok
test remove_key ... ok
test cli_invalid_subcommand ... ok
test cli_rm ... ok
test cli_get ... ok
test cli_set ... ok
test cli_no_args ... ok
test cli_version ... ok
test cli_invalid_rm ... ok
test cli_invalid_get ... ok
test cli_invalid_set ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests kvs

running 1 test
test src/kv.rs - kv::KvStore (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s
```

## 总结

实现了 KVS 的命令行程序，使用内存存储数据。

## 附录
