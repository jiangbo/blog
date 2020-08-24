# 【Docker】命令 pull

参考教程：https://docs.docker.com/engine/reference/commandline/pull/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker pull [OPTIONS] NAME[:TAG|@DIGEST]`

使用 pull 命令可以从 https://hub.docker.com/ 上或者私服上下载可用的镜像。

### 代理配置

如果你使用代理上网，比如在公司内网中，则在连接到 Registry 之前，需要使用环境变量 HTTP_PROXY，HTTPS_PROXY 和 NO_PROXY 配置 Docker 守护进程的代理。

### 并发下载

默认情况下，Docker 守护进程同一时间拉取三层镜像（镜像的分层之后学习）。如果你使用的是低宽带，则可能会导致超时问题，你可以通过通过守护进程的 `--max-concurrent-downloads` 参数来降低它。

## 命令选项

| 名称 | 默认值 | 描述 |
| --- | --- | --- |
| `--all-tags , -a` |  | 下载该仓库的所有 tag，相当于下载项目的所有版本 |
| `--disable-content-trust` | `true` | 跳过镜像验证 |
| `--platform` |  | 实验性特性，不用于生产 |
| `--quiet , -q` |  | 不显示详细输出 |

## 示例

### 拉取镜像

* 没有指定 Registry，所以默认到 docker hub 中下载
* 没有指定 docker user id，所以默认到 library 中下载
* 没有指定 tag 的名称，所以下载 latest

```sh
$ docker pull debian
Using default tag: latest
latest: Pulling from library/debian
d6ff36c9ec48: Pull complete
Digest: sha256:1e74c92df240634a39d050a5e23fb18f45df30846bb222f543414da180b47a5d
Status: Downloaded newer image for debian:latest
docker.io/library/debian:latest                                                         0
```

### 通过 digest 拉取镜像

```sh
$ docker pull ubuntu:14.04
14.04: Pulling from library/ubuntu
2e6e20c8e2e6: Pull complete
30bb187ac3fc: Pull complete
b7a5bcc4a58a: Pull complete
Digest: sha256:ffc76f71dd8be8c9e222d420dc96901a07b61616689a44c7b3ef6a10b7213de4
Status: Downloaded newer image for ubuntu:14.04
docker.io/library/ubuntu:14.04

$ docker pull ubuntu@sha256:ffc76f71dd8be8c9e222d420dc96901a07b61616689a44c7b3ef6a10b7213de4
sha256:ffc76f71dd8be8c9e222d420dc96901a07b61616689a44c7b3ef6a10b7213de4: Pulling from library/ubuntu
Digest: sha256:ffc76f71dd8be8c9e222d420dc96901a07b61616689a44c7b3ef6a10b7213de4
Status: Image is up to date for ubuntu@sha256:ffc76f71dd8be8c9e222d420dc96901a07b61616689a44c7b3ef6a10b7213de4
docker.io/library/ubuntu@sha256:ffc76f71dd8be8c9e222d420dc96901a07b61616689a44c7b3ef6a10b7213de4
```

### 从其它仓库拉取

命令格式为：`docker pull myregistry.local:5000/testing/test-image`，搭建私服后学习。

### 拉取多个镜像

--all-tags 参数会下载所有的 tag 信息的镜像。

```sh
$ docker pull --all-tags fedora

Pulling repository fedora
ad57ef8d78d7: Download complete
105182bb5e8b: Download complete
511136ea3c5a: Download complete
73bd853d2ea5: Download complete
....

Status: Downloaded newer image for fedora
```

### 取消拉取

可以使用 `CTRL + C` 取消拉取镜像。

## 总结

介绍了 pull 命令的使用，可以从不同的仓库中拉取镜像到本地。

