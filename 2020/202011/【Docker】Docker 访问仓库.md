# 【Docker】Docker 访问仓库

参考教程：https://yeasy.gitbook.io/docker_practice/introduction
书籍：《Docker技术入门与实践》

仓库（`Repository`）是集中存放镜像的地方。

一个容易混淆的概念是注册服务器（`Registry`）。实际上注册服务器是管理仓库的具体服务器，每个服务器上可以有多个仓库，而每个仓库下面有多个镜像。从这方面来说，仓库可以被认为是一个具体的项目或目录。例如对于仓库地址 `docker.io/ubuntu` 来说，`docker.io` 是注册服务器地址，`ubuntu` 是仓库名。

大部分时候，并不需要严格区分这两者的概念。

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## Docker Hub

推送镜像到 Docker Hub。

```sh
[node1] (local) root@192.168.0.18 ~
$ docker tag busybox jiangbo920827/my_busybox:v1
[node1] (local) root@192.168.0.18 ~
$ docker images
REPOSITORY                 TAG                 IMAGE ID            CREATED             SIZE
jiangbo920827/my_busybox   v1                  6858809bf669        10 days ago         1.23MB
busybox                    latest              6858809bf669        10 days ago         1.23MB
[node1] (local) root@192.168.0.18 ~
$ docker login --username jiangbo920827
Password: 
WARNING! Your password will be stored unencrypted in /root/.docker/config.json.
Configure a credential helper to remove this warning. See
https://docs.docker.com/engine/reference/commandline/login/#credentials-store

Login Succeeded
[node1] (local) root@192.168.0.18 ~
$ docker push jiangbo920827/my_busybox:v1
The push refers to repository [docker.io/jiangbo920827/my_busybox]
be8b8b42328a: Mounted from library/busybox 
v1: digest: sha256:2ca5e69e244d2da7368f7088ea3ad0653c3ce7aaccd0b8823d11b0d5de956002 size: 527
[node1] (local) root@192.168.0.18 ~
```

## 私有仓库

有时候使用 Docker Hub 这样的公共仓库可能不方便，用户可以创建一个本地仓库供私人使用。

​[`docker-registry`](https://docs.docker.com/registry/) 是官方提供的工具，可以用于构建私有的镜像仓库。本文内容基于 [`docker-registry`](https://github.com/docker/distribution) v2.x 版本。

```sh
[node1] (local) root@192.168.0.18 ~
$ docker run -d -p 5000:5000 --restart=always --name registry registry
Unable to find image 'registry:latest' locally
latest: Pulling from library/registry
cbdbe7a5bc2a: Pull complete 
47112e65547d: Pull complete 
46bcb632e506: Pull complete 
c1cc712bcecd: Pull complete 
3db6272dcbfa: Pull complete 
Digest: sha256:8be26f81ffea54106bae012c6f349df70f4d5e7e2ec01b143c46e2c03b9e551d
Status: Downloaded newer image for registry:latest
4256b6cfdd6daaa74ec055f030eb92432c753e905239dad428f16acbdf589a45
[node1] (local) root@192.168.0.18 ~
$ docker tag busybox 127.0.0.1:5000/my_busybox
[node1] (local) root@192.168.0.18 ~
$ docker push 127.0.0.1:5000/my_busybox
The push refers to repository [127.0.0.1:5000/my_busybox]
be8b8b42328a: Pushed 
latest: digest: sha256:2ca5e69e244d2da7368f7088ea3ad0653c3ce7aaccd0b8823d11b0d5de956002 size: 527
[node1] (local) root@192.168.0.18 ~
$ curl 127.0.0.1:5000/v2/_catalog
{"repositories":["my_busybox"]}
[node1] (local) root@192.168.0.18 ~
```

## Nexus 3

使用 Docker 官方的 Registry 创建的仓库面临一些维护问题。比如某些镜像删除以后空间默认是不会回收的，需要一些命令去回收空间然后重启 Registry 程序。在企业中把内部的一些工具包放入 Nexus 中是比较常见的做法，最新版本 `Nexus3.x` 全面支持 Docker 的私有镜像。所以使用 [`Nexus3.x`](https://www.sonatype.com/nexus/repository-oss/download) 一个软件来管理 `Docker` , `Maven` , `Yum` , `PyPI` 等是一个明智的选择。

```sh
$ docker run -d --name nexus3 --restart=always \
    -p 8081:8081 \
    --mount src=nexus-data,target=/nexus-data \
    sonatype/nexus3
```

看到如下的界面，则启动成功：
![Nexus3][1]

## 总结

介绍了公共仓库和私有仓库，都可以用来存储镜像。

[1]: images/nexus3.png
