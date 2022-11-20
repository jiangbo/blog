# 0215-引入 pnet_packet 包

## 环境

- Time 2022-11-20
- WSL-Ubuntu 22.04
- Rust 1.65.0
- pnet 0.31.0

## 前言

### 说明

参考：<https://docs.rs/pnet_packet/latest/pnet_packet/index.html>

### 目标

使用 pnet_packet 包优化代码。

## Cargo.toml

```toml
[package]
edition = "2021"
name = "network"
version = "1.0.0"

[dependencies]
pnet_datalink = "0.31.0"
pnet_packet = "0.31.0"
```

## reader.rs

```Rust
use pnet_datalink::{channel, Channel};
fn main() {
    let interface = pnet_datalink::interfaces()
        .into_iter()
        .find(|iface| iface.name == "mydummy")
        .unwrap();

    let channel = channel(&interface, Default::default());
    let (mut _sender, mut reader) = match channel {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        _ => panic!("Not a valid channel returned"),
    };

    loop {
        let buf = reader.next().unwrap();
        println!("{:?}", buf);
    }
}
```

## sender.rs

```Rust
use pnet_datalink::{channel, Channel};
use pnet_packet::ethernet::{EtherTypes, MutableEthernetPacket};

fn main() {
    let interface = pnet_datalink::interfaces()
        .into_iter()
        .find(|iface| iface.name == "mydummy")
        .unwrap();

    let channel = channel(&interface, Default::default());
    let (mut sender, mut _reader) = match channel {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        _ => panic!("Not a valid channel returned"),
    };

    let mut buffer = [0; 14];
    let mut packet = MutableEthernetPacket::new(&mut buffer).unwrap();

    packet.set_destination(interface.mac.unwrap());
    packet.set_source(interface.mac.unwrap());
    packet.set_ethertype(EtherTypes::Ipv4);

    sender.send_to(&buffer, None).unwrap().unwrap();
}
```

## 总结

使用 pnet_packet 包优化代码，将之前的单个赋值直接修改成方法封装。

## 附录
