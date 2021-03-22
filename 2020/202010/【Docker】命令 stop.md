# 【Docker】命令 stop

参考教程：https://docs.docker.com/engine/reference/commandline/stop/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker stop [OPTIONS] CONTAINER [CONTAINER...]`

使用 stop 命令可以停止一个容器。

## 命令选项

| 名称 | 默认值 | 描述 |
| --- | --- | --- |
| `--time , -t` | `10` | 停止时的最大等待时间 |

## 示例

### 停止容器

```sh
[root@master ~]# docker ps
CONTAINER ID        IMAGE               COMMAND                  CREATED             STATUS              PORTS                  NAMES
b82ba0c69740        nginx               "/docker-entrypoint.…"   11 minutes ago      Up 11 minutes       0.0.0.0:8080->80/tcp   pedantic_allen
[root@master ~]# docker stop b82
b82
[root@master ~]# docker ps
CONTAINER ID        IMAGE               COMMAND             CREATED             STATUS              PORTS               NAMES
[root@master ~]#
```

## 总结

介绍了 stop 命令的使用，可以停止一个正在运行的容器，默认等待 10 秒。
