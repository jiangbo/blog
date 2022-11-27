# 0231-ethtool 常用参数

## 环境

- Time 2022-11-25
- WSL-Ubuntu 22.04

## 前言

### 说明

参考：<https://man7.org/linux/man-pages/man8/ethtool.8.html>

### 目标

通过 ethtool 命令的几个常用参数，来获取网卡的一些信息。

## 显示信息

`ethtool dev_name`，命令直接加上设备的名称：

```text
root@jiangbo12490:~# ethtool eth0
Settings for eth0:
        Supported ports: [ ]
        Supported link modes:   Not reported
        Supported pause frame use: No
        Supports auto-negotiation: No
        Supported FEC modes: Not reported
        Advertised link modes:  Not reported
        Advertised pause frame use: No
        Advertised auto-negotiation: No
        Advertised FEC modes: Not reported
        Speed: 10000Mb/s
        Duplex: Full
        Port: Other
        PHYAD: 0
        Transceiver: internal
        Auto-negotiation: off
        Current message level: 0x000000f7 (247)
                               drv probe link ifdown ifup rx_err tx_err
        Link detected: yes
```

其中的 `Speed` 说明这块网卡是万兆网卡，`Duplex` 指明了全双工工作模式。
`Link detected: yes` 提示是否插上了网线。

## 驱动信息

`-i` 或者 `--driver` 参数查看网卡的驱动信息。

```text
root@jiangbo12490:~# ethtool --driver eth0
driver: hv_netvsc
version: 5.10.102.1-microsoft-standard-W
firmware-version: N/A
expansion-rom-version:
bus-info:
supports-statistics: yes
supports-test: no
supports-eeprom-access: no
supports-register-dump: yes
supports-priv-flags: no
```

## 统计信息

参数 `--statistics` 或者 `-S` 可以显示统计信息。
显示了每个 CPU 和 队列的接收数据包和发送数据包的数量。
也包括接收和发送的字节数量。

```text
root@jiangbo12490:~# ethtool -S eth0
NIC statistics:
     tx_scattered: 0
     tx_no_memory: 0
     tx_no_space: 0
     tx_too_big: 0
     tx_busy: 0
     tx_send_full: 0
     rx_comp_busy: 0
     rx_no_memory: 0
     stop_queue: 0
     wake_queue: 0
     vlan_error: 0
     vf_rx_packets: 0
     vf_rx_bytes: 0
     vf_tx_packets: 0
     vf_tx_bytes: 0
     vf_tx_dropped: 0
     tx_queue_0_packets: 12301
     tx_queue_0_bytes: 889622
     rx_queue_0_packets: 207
     rx_queue_0_bytes: 57056
     rx_queue_0_xdp_drop: 0
     ...
     rx_queue_7_packets: 188304
     rx_queue_7_bytes: 280126441
     rx_queue_7_xdp_drop: 0
     cpu0_rx_packets: 0
     cpu0_rx_bytes: 0
     cpu0_tx_packets: 0
     cpu0_tx_bytes: 0
     cpu0_vf_rx_packets: 0
     cpu0_vf_rx_bytes: 0
     cpu0_vf_tx_packets: 0
     cpu0_vf_tx_bytes: 0
     ...
     cpu11_vf_tx_bytes: 0
```

## 支持的特性

参数 `--show-features` 或者 `-k` 可以显示特性信息。

```text
root@jiangbo12490:~# ethtool -k eth0
Features for eth0:
rx-checksumming: on
tx-checksumming: on
        tx-checksum-ipv4: on
        tx-checksum-ip-generic: off [fixed]
        tx-checksum-ipv6: on
        tx-checksum-fcoe-crc: off [fixed]
        tx-checksum-sctp: off [fixed]
scatter-gather: on
        tx-scatter-gather: on
        tx-scatter-gather-fraglist: off [fixed]
tcp-segmentation-offload: on
        tx-tcp-segmentation: on
        tx-tcp-ecn-segmentation: off [fixed]
        tx-tcp-mangleid-segmentation: off
        tx-tcp6-segmentation: on
generic-segmentation-offload: on
generic-receive-offload: on
large-receive-offload: on
rx-vlan-offload: on [fixed]
tx-vlan-offload: on [fixed]
ntuple-filters: off [fixed]
receive-hashing: on
highdma: on [fixed]
rx-vlan-filter: off [fixed]
vlan-challenged: off [fixed]
tx-lockless: off [fixed]
netns-local: off [fixed]
tx-gso-robust: off [fixed]
tx-fcoe-segmentation: off [fixed]
tx-gre-segmentation: off [fixed]
tx-gre-csum-segmentation: off [fixed]
tx-ipxip4-segmentation: off [fixed]
tx-ipxip6-segmentation: off [fixed]
tx-udp_tnl-segmentation: off [fixed]
tx-udp_tnl-csum-segmentation: off [fixed]
tx-gso-partial: off [fixed]
tx-tunnel-remcsum-segmentation: off [fixed]
tx-sctp-segmentation: off [fixed]
tx-esp-segmentation: off [fixed]
tx-udp-segmentation: off [fixed]
tx-gso-list: off [fixed]
fcoe-mtu: off [fixed]
tx-nocache-copy: off
loopback: off [fixed]
rx-fcs: off [fixed]
rx-all: off [fixed]
tx-vlan-stag-hw-insert: off [fixed]
rx-vlan-stag-hw-parse: off [fixed]
rx-vlan-stag-filter: off [fixed]
l2-fwd-offload: off [fixed]
hw-tc-offload: off [fixed]
esp-hw-offload: off [fixed]
esp-tx-csum-hw-offload: off [fixed]
rx-udp_tunnel-port-offload: off [fixed]
tls-hw-tx-offload: off [fixed]
tls-hw-rx-offload: off [fixed]
rx-gro-hw: off [fixed]
tls-hw-record: off [fixed]
rx-gro-list: off
macsec-hw-offload: off [fixed]
```

其中的 `tcp-segmentation-offload` 表示 TCP 的分段的操作可以直接交给网卡。

## 通道

参数 `--show-channels` 或者 `-l` 可以显示通道信息。
有时也叫队列的数量，表示可以几个队列可以同时接收网络包。

```text
root@jiangbo12490:~# ethtool --show-channels eth0
Channel parameters for eth0:
Pre-set maximums:
RX:             0
TX:             0
Other:          0
Combined:       12
Current hardware settings:
RX:             0
TX:             0
Other:          0
Combined:       8
```

可以看到通道数量最大是 12 个，当前是 8 个。

参数 `--set-channels` 或者 `-L` 可以修改设置。

## 队列大小

参数 `--show-ring` 或者 `-g` 可以显示队列的大小。
接收数据包的一个环形缓冲，表示没有被内核取走前可以缓冲几个包。
每个通道/队列的缓冲是分开的，不在一起。
队列和内核中的 `Ring Buffer` 是对应的，接收到数据包，通过 DMA 将数据包拷贝到内核。

```text
root@jiangbo12490:~# ethtool --show-ring eth0
Ring parameters for eth0:
Pre-set maximums:
RX:             18811
RX Mini:        0
RX Jumbo:       0
TX:             2560
Current hardware settings:
RX:             9709
RX Mini:        0
RX Jumbo:       0
TX:             170
```

参数 `--set-ring` 或者 `-G` 可以修改设置。

## 总结

介绍了 ethtool 工具的几个常见的参数，以及查看网卡的信息。

## 附录
