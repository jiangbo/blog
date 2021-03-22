# 【Docker】命令 restart

参考教程：https://docs.docker.com/engine/reference/commandline/restart/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker restart [OPTIONS] CONTAINER [CONTAINER...]`

restart 命令可以将一个正在运行的容器进行重启。

## 命令选项

由于命令选项有很多，下面选择几个常用的进行学习。

| 名称 | 默认值 | 描述 |
| --- | --- | --- |
| `--time , -t` | `10` | 在停止容器前的等待时间 |

## 示例

### 重启容器

```sh
[root@master ~]# docker ps
CONTAINER ID        IMAGE               COMMAND                  CREATED             STATUS              PORTS                  NAMES
b82ba0c69740        nginx               "/docker-entrypoint.…"   4 days ago          Up About a minute   0.0.0.0:8080->80/tcp   pedantic_allen
[root@master ~]# docker restart b82
b82
[root@master ~]#
```

## 总结

介绍了 restart 命令的使用，可以重启容器。
