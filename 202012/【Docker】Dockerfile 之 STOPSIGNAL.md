# 【Docker】Dockerfile 之 STOPSIGNAL

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## STOPSIGNAL

```Dockerfile
STOPSIGNAL signal
```

The `STOPSIGNAL` instruction sets the system call signal that will be sent to the container to exit. This signal can be a valid unsigned number that matches a position in the kernel’s syscall table, for instance 9, or a signal name in the format SIGNAME, for instance SIGKILL.

`STOPSIGNAL` 指令设置将被发送到容器退出的系统调用信号。该信号可以是与内核 syscall 表中的位置匹配的有效无符号数字（例如9），也可以是 SIGNAME 格式的信号名称（例如 SIGKILL）。

## 总结

介绍了 Dockerfile 中 STOPSIGNAL 指令的说明。