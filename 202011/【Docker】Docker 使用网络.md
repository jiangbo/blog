# 【Docker】Docker 使用网络

参考教程：https://yeasy.gitbook.io/docker_practice/introduction
书籍：《Docker技术入门与实践》

Docker 允许通过外部访问容器或容器互联的方式来提供网络服务。

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 外部访问容器

容器中可以运行一些网络应用，要让外部也可以访问这些应用，可以通过 -P 或 -p 参数来指定端口映射。

### 映射随机端口

当使用 -P 标记时，Docker 会随机映射一个端口到内部容器开放的网络端口。

```sh
[root@master bin]# docker run -d -P nginx:alpine
20545e03e5fa1a864be3015382d2e2b9b88c7ab48ac8dde3c0cc75ade7a6560c
[root@master bin]# docker ps
CONTAINER ID  IMAGE          COMMAND               CREATED      STATUS              PORTS                  NAMES
20545e03e5fa  nginx:alpine  nginx -g daemon o...  12 seconds ago  Up 5 seconds ago  0.0.0.0:46749->80/tcp  vigilant_black
```

### 指定端口映射

```sh
$ docker run -d -p 80:80 nginx:alpine
```

### 映射到指定地址的指定端口

```sh
$ docker run -d -p 127.0.0.1:80:80 nginx:alpine
```

### 查看映射端口配置

```sh
[root@master bin]# docker port  205
80/tcp -> 0.0.0.0:46749
```

### 映射多个端口

```sh
$ docker run -d \
    -p 80:80 \
    -p 443:443 \
    nginx:alpine
```

## 容器互联

如果你之前有 Docker 使用经验，你可能已经习惯了使用 --link 参数来使容器互联。
随着 Docker 网络的完善，强烈建议大家将容器加入自定义的 Docker 网络来连接多个容器，而不是使用 --link 参数

### 新建网络

```sh
docker network create -d bridge my-net
```

-d 参数指定 Docker 网络类型，有 bridge overlay。其中 overlay 网络类型用于 Swarm mode，在本小节中你可以忽略它。

### 容器1连接网络

```sh
$ docker run -d --rm --name busybox1 --network my-net busybox sleep 3600
134b610b0c1693edc3cfde3327257e03f78d78db799aacaee75263cb0de58ed7
```

### 容器2连接网络

```sh
$ docker run -d --rm --name busybox2 --network my-net busybox sleep 3600
b2390512f2afc58110a0d2c799483ed72d354de0add211d6d33024aeab5366f4
```

### 查看容器

```sh
$ docker ps
CONTAINER ID        IMAGE               COMMAND             CREATED             STATUS              PORTS               NAMES
b2390512f2af        busybox             "sleep 3600"        52 seconds ago      Up 52 seconds                           busybox2
134b610b0c16        busybox             "sleep 3600"        2 minutes ago       Up 2 minutes                            busybox1
```

### 测试连接

```sh
[node1] (local) root@192.168.0.28 ~
$ docker exec -it b239 sh
/ # ping busybox1
PING busybox1 (172.19.0.2): 56 data bytes
64 bytes from 172.19.0.2: seq=0 ttl=64 time=0.133 ms
64 bytes from 172.19.0.2: seq=1 ttl=64 time=0.081 ms
64 bytes from 172.19.0.2: seq=2 ttl=64 time=0.131 ms
64 bytes from 172.19.0.2: seq=3 ttl=64 time=0.097 ms
64 bytes from 172.19.0.2: seq=4 ttl=64 time=0.100 ms
^C
--- busybox1 ping statistics ---
5 packets transmitted, 5 packets received, 0% packet loss
round-trip min/avg/max = 0.081/0.108/0.133 ms
```

## 配置 DNS

如何自定义配置容器的主机名和 DNS 呢？秘诀就是 Docker 利用虚拟文件来挂载容器的 3 个相关配置文件。

### 查看网络文件

在容器中使用 `mount` 命令可以看到挂载信息：

```sh
$ mount
/dev/disk/by-uuid/1fec...ebdf on /etc/hostname type ext4 ...
/dev/disk/by-uuid/1fec...ebdf on /etc/hosts type ext4 ...
tmpfs on /etc/resolv.conf type tmpfs ...
```

这种机制可以让宿主主机 DNS 信息发生更新后，所有 Docker 容器的 DNS 配置通过 `/etc/resolv.conf` 文件立刻得到更新。

### 指定 DNS

配置全部容器的 DNS ，也可以在 `/etc/docker/daemon.json` 文件中增加以下内容来设置。

```json
{
  "dns" : [
    "114.114.114.114",
    "8.8.8.8"
  ]
}
```

这样每次启动的容器 DNS 自动配置为 `114.114.114.114` 和 `8.8.8.8`。使用以下命令来证明其已经生效。

```sh
$ docker run -it --rm ubuntu:18.04 cat etc/resolv.conf
nameserver 114.114.114.114
nameserver 8.8.8.8
```

如果用户想要手动指定容器的配置，可以在使用 `docker run` 命令启动容器时加入如下参数：

`-h HOSTNAME` 或者 `--hostname=HOSTNAME` 设定容器的主机名，它会被写到容器内的 `/etc/hostname` 和 `/etc/hosts`。但它在容器外部看不到，既不会在 `docker container ls` 中显示，也不会在其他的容器的 `/etc/hosts` 看到。

`--dns=IP_ADDRESS` 添加 DNS 服务器到容器的 `/etc/resolv.conf` 中，让容器用这个服务器来解析所有不在 `/etc/hosts` 中的主机名。

`--dns-search=DOMAIN` 设定容器的搜索域，当设定搜索域为 `.example.com` 时，在搜索一个名为 host 的主机时，DNS 不仅搜索 host，还会搜索 `host.example.com`。

> 注意：如果在容器启动时没有指定最后两个参数，Docker 会默认用主机上的 `/etc/resolv.conf` 来配置容器。

## 总结

介绍了网络的基本使用。

