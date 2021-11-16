# 【Rust】自定义键类型

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/std/hash/alt_key_types.html>  

## 示例

### main.rs

```rust
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Account<'a> {
    username: &'a str,
    password: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, &'a str>;
fn try_logon(accounts: &Accounts, username: &str, password: &str) {
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Attempting logon...");

    let logon = Account { username, password };

    match accounts.get(&logon) {
        Some(age) => {
            println!("Successful logon!");
            println!("Age: {}", age);
        }
        _ => println!("Login failed!"),
    }
}

fn main() {
    let mut accounts = HashMap::new();

    let account = Account {
        username: "j.everyman",
        password: "password123",
    };

    accounts.insert(account, "44");

    try_logon(&accounts, "j.everyman", "psasword123");

    try_logon(&accounts, "j.everyman", "password123");
}
```

## 总结

了解了 Rust 中，自定义 HashMap 的键。

## 附录
