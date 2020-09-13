# 【Docker】命令 images

参考教程：https://docs.docker.com/engine/reference/commandline/images/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker images [OPTIONS] [REPOSITORY[:TAG]]`

使用 images 命令可以查看本地已经下载的所有镜像。

## 命令选项

由于命令选项有很多，下面选择几个常用的进行学习。

| 名称 | 默认值 | 描述 |
| --- | --- | --- |
| `--all , -a` |  | 显示所有的镜像（默认隐藏中间镜像） |
| `--digests` |  | 显示数字签名 |
| `--filter , -f` |  | 根据条件过滤显示 |
| `--format` |  | 格式化输出 |
| `--no-trunc` |  | 不截断显示 |
| `--quiet , -q` |  | 只显示 id |

## 示例

### 显示镜像

```sh
[root@master ~]# docker images
REPOSITORY          TAG                 IMAGE ID            CREATED             SIZE
busybox             latest              edabd795951a        3 days ago          1.22MB
nginx               latest              4bb46517cac3        3 weeks ago         133MB
redis               latest              1319b1eaa0b7        4 weeks ago         104MB
[root@master ~]# docker images redis
REPOSITORY          TAG                 IMAGE ID            CREATED             SIZE
redis               latest              1319b1eaa0b7        4 weeks ago         104MB
[root@master ~]# docker images redis:latest
REPOSITORY          TAG                 IMAGE ID            CREATED             SIZE
redis               latest              1319b1eaa0b7        4 weeks ago         104MB
[root@master ~]#
```

## 总结

介绍了 images 命令的使用，可以显示本地下载的镜像。
