# 【Docker】Docker 数据管理

参考教程：https://yeasy.gitbook.io/docker_practice/introduction
书籍：《Docker技术入门与实践》

在容器中管理数据主要有两种方式：
- 数据卷（Volumes）
- 挂载主机目录 (Bind mounts)

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 数据卷

`数据卷` 是一个可供一个或多个容器使用的特殊目录，它绕过 UFS，可以提供很多有用的特性：
- `数据卷` 可以在容器之间共享和重用 
- 对 `数据卷` 的修改会立马生效  
- 对 `数据卷` 的更新，不会影响镜像
- `数据卷` 默认会一直存在，即使容器被删除

> 注意：`数据卷` 的使用，类似于 Linux 下对目录或文件进行 mount，镜像中的被指定为挂载点的目录中的文件会复制到数据卷中（仅数据卷为空时会复制）。

### 创建数据卷

```sh
$ docker volume create my_volume
my_volume
```

### 查看数据卷

```sh
[node1] (local) root@192.168.0.23 ~
$ docker volume ls
DRIVER              VOLUME NAME
local               my_volume
```

### 查看数据卷详情

```sh
[node1] (local) root@192.168.0.23 ~
$ docker volume inspect my_volume
[
    {
        "CreatedAt": "2020-09-22T11:20:15Z",
        "Driver": "local",
        "Labels": {},
        "Mountpoint": "/var/lib/docker/volumes/my_volume/_data",
        "Name": "my_volume",
        "Options": {},
        "Scope": "local"
    }
]
```

创建一个数据卷

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
