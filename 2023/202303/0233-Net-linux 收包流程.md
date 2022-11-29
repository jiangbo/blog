# 0233-Net-linux 收包流程

## 环境

- Time 2022-11-27
- WSL-Ubuntu 22.04

## 前言

### 说明

参考：<https://github.com/leandromoreira/linux-network-performance-parameters>

### 目标

了解 linux 系统接收网络数据包的具体流程。

## 数据到达网卡

网卡将光电信号还原成数据包。

## 网卡校验

如果不是在混杂模式下，网卡校验数据包中的 MAC 地址，同时也校验 FCS。
FCS 帧校验序列，在数据包的末尾，有 4 字节的 FCS 校验位，使用 CRC 生成。
如果数据包不满足校验，会被丢弃。

通过命令 `ethtool -S` 或者 `ip -s -s link show` 命令来查看。

## DMA

如果网卡还有接收描述符，通过 DMA，将数据包拷贝到 Ring Buffer。
并将一个描述符指向 Ring Buffer 的内存区域。

通过命令 `ethtool -g` 可以查看发送和接收的描述符。

## 发起硬中断

在等待 rx-usecs 微秒或者 rx-frames 个帧后，网卡发其硬中断。

通过命令 `ethtool -c` 可以查看网卡的 rx-usecs 和 rx-frames 参数。

## 处理硬中断

CPU 处理硬中断函数，触发一个软中断，清除硬中断。

通过命令 `ethtool -x` 可以查看网卡的队列由哪个 CPU 进行处理。
硬件中断信息通过 `cat /proc/interrupts` 命令查看。
/proc/irq 目录下有对应的中断号的信息，smp_affinity 查看和 CPU 的绑定信息。

## 软中断

软中断会使用网卡注册的 NAPI 进行收包。

软中断可以通过 `cat /proc/softirqs` 命令查看。

## 内核收包

NAPI 从 Ring Buffer 进行收包，封装成 sk_buff。
收包的时间由 netdev_budget_usecs 控制，次数由 netdev_budge 和 dev_weight 控制。

通过命令 `sysctl -a | grep netdev` 和 `sysctl net.core.dev_weight` 进行查看。

## 内核网络栈

数据包 sk_buff 进入网络内核栈，交给应用程序。
如果内核处理不过来，数据包需要进行排队，netdev_max_backlog 控制最大排队数量。

可以通过命令 `sysctl net.core.netdev_max_backlog` 查看。
如果队列不够，通过命令 `cat /proc/net/softnet_stat` 查看，第二列增长就有问题。
详细查看：<https://access.redhat.com/solutions/1241943>，附录由更好的工具。

## 总结

介绍了 linux 接收数据包的流程，以及其中可以使用的查看工具。

## 附录

### 工具

```Bash
#!/bin/bash

cmd="${0##*/}"

usage() {
cat >&2 <<EOI
usage: $cmd [ -h ]

Output column definitions:
      cpu  # of the cpu

    total  # of packets (not including netpoll) received by the interrupt handler
             There might be some double counting going on:
                net/core/dev.c:1643: __get_cpu_var(netdev_rx_stat).total++;
                net/core/dev.c:1836: __get_cpu_var(netdev_rx_stat).total++;
             I think the intention was that these were originally on separate
             receive paths ...

  dropped  # of packets that were dropped because netdev_max_backlog was exceeded

 squeezed  # of times ksoftirq ran out of netdev_budget or time slice with work
             remaining

collision  # of times that two cpus collided trying to get the device queue lock.

EOI
 exit 1
}



softnet_stats_header() {
 printf "%3s %10s %10s %10s %10s %10s %10s\n" cpu total dropped squeezed collision rps flow_limit
}

softnet_stats_format() {
 printf "%3u %10lu %10lu %10lu %10lu %10lu %10lu\n" "$1" "0x$2" "0x$3" "0x$4" "0x$5" "0x$6" "0x$7"
}


getopts h flag && usage

cpu=0
softnet_stats_header
while read total dropped squeezed j1 j2 j3 j4 j5 collision rps flow_limit_count
do
 # the last field does not appear on older kernels
 # https://github.com/torvalds/linux/commit/99bbc70741903c063b3ccad90a3e06fc55df9245#diff-5dd540e75b320a50866267e9c52b3289R165
 softnet_stats_format $((cpu++)) "$total" "$dropped" "$squeezed" "$collision" "$rps" "${flow_limit_count:-0}"
done < /proc/net/softnet_stat
```
