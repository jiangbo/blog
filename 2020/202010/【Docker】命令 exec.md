# 【Docker】命令 exec

参考教程：https://docs.docker.com/engine/reference/commandline/exec/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker exec [OPTIONS] CONTAINER COMMAND [ARG...]`

exec 命令可以连接到一个容器上，即使退出了连接，容器也不会停止。

## 命令选项

| 名称 | 默认值 | 描述 |
| --- | --- | --- |
| `--detach , -d` |  | 后台运行容器 |
| `--detach-keys` |  | 后台运行的按键 |
| `--env , -e` |  | 设置环境变量 |
| `--interactive , -i` |  | 连接到标准输入 |
| `--privileged` |  | 授予特别的权限 |
| `--tty , -t` |  | 分配一个伪终端 |
| `--user , -u` |  | 用户空间 |
| `--workdir , -w` |  | 设置在容器中的工作目录 |

## 示例

### 连接到容器

```sh
[root@master ~]# docker ps
CONTAINER ID        IMAGE               COMMAND                  CREATED             STATUS              PORTS               NAMES
1d06decda260        nginx               "/docker-entrypoint.…"   23 minutes ago      Up 2 minutes        80/tcp              strange_mendel
[root@master ~]# docker exec -it 1d /bin/bash
root@1d06decda260:/# ip a
bash: ip: command not found
root@1d06decda260:/# exit
exit
[root@master ~]# docker ps
CONTAINER ID        IMAGE               COMMAND                  CREATED             STATUS              PORTS               NAMES
1d06decda260        nginx               "/docker-entrypoint.…"   24 minutes ago      Up 3 minutes        80/tcp              strange_mendel
[root@master ~]#
```

## 总结

介绍了 exec 命令的作用，可以连接到一个正在运行的容器，并且在退出后，容器并不会停止。
