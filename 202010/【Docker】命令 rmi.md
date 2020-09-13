# 【Docker】命令 rmi

参考教程：https://docs.docker.com/engine/reference/commandline/rmi/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker rmi [OPTIONS] IMAGE [IMAGE...]`

使用 rm 命令可以删除本地镜像。如果镜像有多个标签，则只会去掉标签，不会真正的删除。如果只有一个标签，那么标签和镜像都会被删除。

## 命令选项

| 名称 | 默认值 | 描述 |
| --- | --- | --- |
| `--force , -f` |  | 强制删除镜像 |
| `--no-prune` |  | 不删除没有标签的镜像 |

## 示例

### 删除多个标签的镜像

```sh
[root@master ~]# docker images
REPOSITORY          TAG                 IMAGE ID            CREATED             SIZE
nginx               latest              4bb46517cac3        2 weeks ago         133MB
redis               latest              1319b1eaa0b7        4 weeks ago         104MB
busybox             1.0.0               018c9d7b792b        5 weeks ago         1.22MB
busybox             1.0.1               018c9d7b792b        5 weeks ago         1.22MB
busybox             1.0.2               018c9d7b792b        5 weeks ago         1.22MB
busybox             1.0.3               018c9d7b792b        5 weeks ago         1.22MB
busybox             latest              018c9d7b792b        5 weeks ago         1.22MB
[root@master ~]# docker 018c9d7b792b
docker: '018c9d7b792b' is not a docker command.
See 'docker --help'
[root@master ~]# docker rmi 018c9d7b792b
Error response from daemon: conflict: unable to delete 018c9d7b792b (must be forced) - image is referenced in multiple repositories
[root@master ~]# docker rmi busybox:1.0.3
Untagged: busybox:1.0.3
[root@master ~]#
```

### 删除多个镜像

```sh
[root@master ~]# docker images
REPOSITORY          TAG                 IMAGE ID            CREATED             SIZE
nginx               latest              4bb46517cac3        2 weeks ago         133MB
redis               latest              1319b1eaa0b7        4 weeks ago         104MB
busybox             1.0.0               018c9d7b792b        5 weeks ago         1.22MB
busybox             1.0.1               018c9d7b792b        5 weeks ago         1.22MB
busybox             1.0.2               018c9d7b792b        5 weeks ago         1.22MB
busybox             latest              018c9d7b792b        5 weeks ago         1.22MB
[root@master ~]# docker rmi 018c9d7b792b -f
Untagged: busybox:1.0.0
Untagged: busybox:1.0.1
Untagged: busybox:1.0.2
Untagged: busybox:latest
Untagged: busybox@sha256:4f47c01fa91355af2865ac10fef5bf6ec9c7f42ad2321377c21e844427972977
Deleted: sha256:018c9d7b792b4be80095d957533667279843acf9a46c973067c8d1dff31ea8b4
Deleted: sha256:514c3a3e64d4ebf15f482c9e8909d130bcd53bcc452f0225b0a04744de7b8c43
[root@master ~]#
```

## 总结

介绍了 rmi 命令的使用，可以本地的镜像。
