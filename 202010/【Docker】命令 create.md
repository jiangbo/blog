# 【Docker】命令 create

参考教程：https://docs.docker.com/engine/reference/commandline/create/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker create [OPTIONS] IMAGE [COMMAND] [ARG...]`

使用 create 命令可以根据镜像创建一个未启动的容器。

docker create 命令创建一个指定的镜像上创建一层可写的容器层，并为运行指定的命令做好准备。容器的 ID 会打印到标准输出流上，这个命令和 docker run -d 命令很相似，除了不会启动之外。你可以使用 docker start 容器 ID 来启动这个容器。如果您想提前配置容器，以便在需要时可以直接启动它，这将很有用。

## 命令选项

由于命令选项有很多，下面选择几个常用的进行学习。

| 名称 | 默认值 | 描述 |
| --- | --- | --- |
| `--env , -e` |  | 设置环境变量 |
| `--env-file` |  | 从一个文件中读取环境变量 |
| `--expose` |  | 暴露一个随机的端口 |
| `--help` |  | 打印帮助信息 |
| `--hostname , -h` |  | 设置容器的 hostname |
| `--interactive , -i` |  | 打开一个标准输入 |
| `--mac-address` |  | 设置 mac 地址：92:d0:c6:0a:29:33 |
| `--name` |  | 给容器设置一个名称 |
| `--rm` |  | 当容器退出时，自动删除 |
| `--tty , -t` |  | 分配一个伪终端 |
| `--workdir , -w` |  | 设置工作目录 |

## 示例

### 创建容器

```sh
[root@master ~]# docker create busybox
Unable to find image 'busybox:latest' locally
latest: Pulling from library/busybox
61c5ed1cbdf8: Pull complete
Digest: sha256:4f47c01fa91355af2865ac10fef5bf6ec9c7f42ad2321377c21e844427972977
Status: Downloaded newer image for busybox:latest
a0c181fd2db5b5864ab418c222fb87386a133dfed2996b5d5d7d8bafbe1f2f53
```

创建的容器的 id 是：a0c181fd2db5b5864ab418c222fb87386a133dfed2996b5d5d7d8bafbe1f2f53

可以通过 docker ps -a 查看所有的容器，即使不是在运行（Running）状态。

```sh
[root@master ~]# docker ps -a
CONTAINER ID    IMAGE       COMMAND      CREATED              STATUS      PORTS     NAMES
a0c181fd2db5    busybox     "sh"         About a minute ago   Created               sharp_mclaren
```

### 自动删除容器

```sh
[root@master docker]# docker create --rm busybox
ee0643c7fffa4c6b71fd2c691716cd9581efbaed132b69bf8c4dee8cee2e38ab
[root@master docker]# docker start ee
ee
[root@master docker]# docker ps -a
CONTAINER ID        IMAGE               COMMAND             CREATED             STATUS              PORTS               NAMES
```

### 打开交互终端

```sh
[root@master docker]# docker create -it busybox /bin/sh
69575a4213c1ace85b198f3a94b50acc21a3e96ee3429b48ef9d72a39f0f250a
[root@master docker]# docker start -i 69
/ # ls
bin   dev   etc   home  proc  root  sys   tmp   usr   var
/ #
```

### 设置环境变量

```sh
[root@master docker]# docker create -it --env USER_NAME=jiangbo busybox /bin/sh
1b180d917835c348d17156633be0d0fb592218182665f37a3f609e87e63a0926
[root@master docker]# docker start -i 1b1
/ # env
HOSTNAME=1b180d917835
SHLVL=1
HOME=/root
TERM=xterm
PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
PWD=/
USER_NAME=jiangbo
```

### 从文件设置环境变量

```sh
[root@master docker]# cat env.properties
USER_NAME=jiangbo
AGE=44
[root@master docker]# docker create -it --env-file=env.properties busybox /bin/sh
a845a16c71aa3f2ded58fcd998b43d0304341e946e7013a842088700a658622f
[root@master docker]# docker start -i a8
/ # env
HOSTNAME=a845a16c71aa
SHLVL=1
HOME=/root
TERM=xterm
PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
PWD=/
USER_NAME=jiangbo
AGE=44
```

### 设置 mac 地址

```sh
[root@master docker]# docker create -it --mac-address 92:d0:c6:0a:29:33  busybox /bin/sh
055e50604a0c4df3c7a66f7bd8b686ddf7bc35425b359777cb5883ef98e08535
[root@master docker]# docker start -i 05
/ # ip addr
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
    inet 127.0.0.1/8 scope host lo
       valid_lft forever preferred_lft forever
47: eth0@if48: <BROADCAST,MULTICAST,UP,LOWER_UP,M-DOWN> mtu 1500 qdisc noqueue
    link/ether 92:d0:c6:0a:29:33 brd ff:ff:ff:ff:ff:ff
    inet 172.17.0.3/16 brd 172.17.255.255 scope global eth0
       valid_lft forever preferred_lft forever
/ #
```

### 设置 hostname

```sh
[root@master docker]# docker create -it --hostname jiangbo  busybox /bin/sh
71e09755dd856c26d13c7d2649f920dcf289e7590877e479ca6038b2a86ab15d
[root@master docker]# docker start -i 71
/ # hostname
jiangbo
/ #
```

### 指定名称

```sh
[root@master docker]# docker create -it --name myBusybox  busybox /bin/sh
d06c4b236233790a2c3a9f1f63d9a22239cb2c4cc29ba189dc222a92558467e1
[root@master docker]# docker start -i myBusybox
/ # ls
bin   dev   etc   home  proc  root  sys   tmp   usr   var
/ #
```

### 指定工作目录

```sh
[root@master docker]# docker create -it --workdir /home/root  busybox /bin/sh
a108147fd409b840c58226b2cef95aeac8d32402343062d5c7e5bc73206ecdde
[root@master docker]# docker start -i a1
/home/root # ls
/home/root #
```

## 总结

介绍了 create 命令的使用，在创建容器时，可以指定各种参数。
