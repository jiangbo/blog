# 【Docker】命令 attach

参考教程：https://docs.docker.com/engine/reference/commandline/attach/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker attach [OPTIONS] CONTAINER`

attach 命令可以连接到一个正在运行的容器，并且在退出后，也会结束掉连接上的容器。

## 命令选项

| 名称 | 默认值 | 描述 |
| --- | --- | --- |
| `--detach-keys` |  | 覆盖后台运行容器的快捷键 |
| `--no-stdin` |  | 不连接到标准输入 |
| `--sig-proxy` | `true` | 代理进程收到的所有信号 |

## 示例

### 连接到容器

```sh
[root@master ~]# docker attach 1d06decda260
172.17.0.1 - - [02/Sep/2020:14:44:29 +0000] "GET / HTTP/1.1" 200 612 "-" "curl/7.29.0" "-"
^C[root@master ~]# docker ps
CONTAINER ID        IMAGE               COMMAND             CREATED             STATUS              PORTS               NAMES
[root@master ~]#
```

## 总结

介绍了 attach 命令的作用，可以连接到一个正在运行的容器。
