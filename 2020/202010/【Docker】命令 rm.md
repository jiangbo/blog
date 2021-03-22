# 【Docker】命令 rm

参考教程：https://docs.docker.com/engine/reference/commandline/rm/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker rm [OPTIONS] CONTAINER [CONTAINER...]`

使用 rm 命令可以删除一个容器。

## 命令选项

| 名称 | 默认值 | 描述 |
| --- | --- | --- |
| `--force , -f` |  | 强制删除一个运行中的容器 |
| `--link , -l` |  | 删除指定的链接 |
| `--volumes , -v` |  | 删除与容器相关连的匿名数据卷 |

## 示例

### 强制删除运行中的容器

```sh
[root@master ~]# docker ps
CONTAINER ID        IMAGE               COMMAND             CREATED             STATUS              PORTS               NAMES
6d1097584439        busybox             "sleep 3600"        24 minutes ago      Up 24 minutes                           loving_elgamal
02ad866ecaf4        busybox             "sleep 3600"        30 minutes ago      Up 30 minutes                           beautiful_hofstadter
b43564e9170e        busybox             "/bin/sh"           5 days ago          Up 32 minutes                           romantic_ellis
[root@master ~]# docker rm 02 -f
02
[root@master ~]#
```

### 删除停止容器

```sh
[root@master ~]# docker ps -a
CONTAINER ID        IMAGE               COMMAND             CREATED             STATUS                            PORTS               NAMES
6d1097584439        busybox             "sleep 3600"        26 minutes ago      Exited (137) About a minute ago                       loving_elgamal
b43564e9170e        busybox             "/bin/sh"           5 days ago          Up 34 minutes                                         romantic_ellis
[root@master ~]# docker rm 6d
6d
[root@master ~]#
```

## 总结

介绍了 rm 命令的使用，可以删除运行中或者已经停止的容器。
