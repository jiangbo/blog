# 【Docker】命令 kill

参考教程：https://docs.docker.com/engine/reference/commandline/kill/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker kill [OPTIONS] CONTAINER [CONTAINER...]`

使用 kill 命令可以强制停止一个容器。

## 命令选项

| 名称 | 默认值 | 描述 |
| --- | --- | --- |
| `--signal , -s` | `KILL` | 发送的终止信号 |

## 示例

### 停止容器

```sh
[root@master ~]# docker start b82
b82
[root@master ~]# docker ps
CONTAINER ID        IMAGE               COMMAND                  CREATED             STATUS              PORTS                  NAMES
b82ba0c69740        nginx               "/docker-entrypoint.…"   16 minutes ago      Up 1 second         0.0.0.0:8080->80/tcp   pedantic_allen
[root@master ~]# docker kill b82
b82
[root@master ~]# docker ps
CONTAINER ID        IMAGE               COMMAND             CREATED             STATUS              PORTS               NAMES
[root@master ~]#
```

## 总结

介绍了 stop 命令的使用，可以停止一个正在运行的容器，默认等待 10 秒。

[1]: images/docker-run-nginx.png
