# 0212-使用 dummy 收发数据

## 环境

- Time 2022-11-20
- WSL-Ubuntu 22.04
- Rust 1.65.0
- pnet 0.31.0

## 前言

### 说明

参考：<https://docs.rs/pnet_datalink/0.31.0/pnet_datalink/linux/>

### 目标

前面使用了 pnet 发送了一个数据包，这里发送一个数据包，然后接收并显示出来。

## 接收数据

```Rust
use pnet_datalink::{linux, Channel};

fn main() {
    let interfaces = linux::interfaces();

    let dummy: Vec<_> = interfaces
        .into_iter()
        .filter(|e| e.name == "mydummy")
        .collect();

    let config = linux::Config::default();
    let channel = linux::channel(&dummy[0], config);
    let (mut _sender, mut reader) = match channel {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        _ => panic!("Not a valid channel returned"),
    };

    let bytes = reader.next().unwrap();
    dbg!(bytes);

    let string = String::from_utf8_lossy(bytes);
    println!("reader: {}", string)
}
```

## 发送数据

```Rust
use pnet_datalink::{linux, Channel};

fn main() {
    let interfaces = linux::interfaces();

    let dummy: Vec<_> = interfaces
        .into_iter()
        .filter(|e| e.name == "mydummy")
        .collect();

    let config = linux::Config::default();
    let channel = linux::channel(&dummy[0], config);
    let (mut sender, mut _reader) = match channel {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        _ => panic!("Not a valid channel returned"),
    };

    let buffer = String::from("hello, jiangbo");
    sender.send_to(buffer.as_bytes(), None).unwrap().unwrap();
}
```

## tcpdump 的显示

```text
21:09:17.441710 20:6a:69:61:6e:67 (oui Unknown) > 68:65:6c:6c:6f:2c (oui Unknown), ethertype Unknown (0x626f), length 14:
```

可以看到确实发送了一个数据包

## reader 显示

```text
reader: hello, jiangbo
```

## 总结

通过 dummy 和 pnet，实现了两个程序之间的数据交换。

## 附录
