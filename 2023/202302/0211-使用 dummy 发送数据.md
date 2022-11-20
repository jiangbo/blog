# 0211-使用 dummy 发送数据

## 环境

- Time 2022-11-20
- WSL-Ubuntu 22.04
- Rust 1.65.0
- pnet 0.31.0

## 前言

### 说明

参考：<https://docs.rs/pnet_datalink/0.31.0/pnet_datalink/linux/>

### 目标

前面使用了 pnet 自己模拟的一个数据链路层的发送和接收过程。
现在使用 linux 的 dummy 来模拟数据的发送和接收。

## 新建网络接口

新建一个网络接口：

```text
root@jiangbo12490:~# ip link add mydummy type dummy
root@jiangbo12490:~# ip -s link show mydummy
8: mydummy: <BROADCAST,NOARP> mtu 1500 qdisc noop state DOWN mode DEFAULT group default qlen 1000
    link/ether 36:e0:f1:2a:e8:45 brd ff:ff:ff:ff:ff:ff
    RX:  bytes packets errors dropped  missed   mcast
             0       0      0       0       0       0
    TX:  bytes packets errors dropped carrier collsns
             0       0      0       0       0       0
```

其中 `ip link add` 命令可以新增一个网络接口，type 指定了类型。
使用 `ip -s link show mydummy` 查看了网络接口的当前状态。

## 启用网络接口

```text
root@jiangbo12490:~# ip  link set mydummy up
root@jiangbo12490:~# ip -s link show mydummy
8: mydummy: <BROADCAST,NOARP,UP,LOWER_UP> mtu 1500 qdisc noqueue state UNKNOWN mode DEFAULT group default qlen 1000
    link/ether 36:e0:f1:2a:e8:45 brd ff:ff:ff:ff:ff:ff
    RX:  bytes packets errors dropped  missed   mcast
             0       0      0       0       0       0
    TX:  bytes packets errors dropped carrier collsns
           140       2      0       0       0       0
```

`ip  link set mydummy up` 是启用网络接口。
启动后查看状态，可以看到已经传送了两个包了，长度 140 字节。

## 监听网络接口

可以使用 tcpdump 命令监听网络接口上的流量，又叫抓包。

```text
root@jiangbo12490:~/git/game# tcpdump -e -vv -i mydummy
tcpdump: listening on mydummy, link-type EN10MB (Ethernet), snapshot length 262144 bytes


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

    sender.build_and_send(1, 14, &mut |_| {});
}
```

## tcpdump 的显示

```text
root@jiangbo12490:~/git/game# tcpdump -e -vv -i mydummy
tcpdump: listening on mydummy, link-type EN10MB (Ethernet), snapshot length 262144 bytes

20:55:41.681667 00:00:00:00:00:00 (oui Ethernet) > 00:00:00:00:00:00 (oui Ethernet), 802.3, length 0:  [|llc]
```

tcpdump 显示了信息，证明数据包确实发送到了网络接口上。
也可以使用 `ip -s link show mydummy` 查看，数据包增加了一个。

## 总结

使用了 linux dummy 网络接口来进行数据的发送。

## 附录
