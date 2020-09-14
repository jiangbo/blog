# 【Docker】命令 tag

参考教程：https://docs.docker.com/engine/reference/commandline/tag/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker tag SOURCE_IMAGE[:TAG] TARGET_IMAGE[:TAG]`

使用 tag 命令给镜像新建标签。

## 示例

### 根据 ID 创建标签

```sh
[root@master docker]# docker images
REPOSITORY                             TAG      IMAGE ID       CREATED        SIZE
docker.io/library/busybox              mytag    a34cc20fa773   6 weeks ago    1.37 MB
docker.io/library/busybox              latest   a34cc20fa773   6 weeks ago    1.37 MB
[root@master docker]# docker tag a34cc20fa773 busybox:v1
[root@master docker]# docker images
REPOSITORY                             TAG      IMAGE ID       CREATED        SIZE
busybox                                v1       a34cc20fa773   6 weeks ago    1.37 MB
docker.io/library/busybox              latest   a34cc20fa773   6 weeks ago    1.37 MB
docker.io/library/busybox              mytag    a34cc20fa773   6 weeks ago    1.37 MB
[root@master docker]#
```

### 根据已有标签创建标签

```sh
[root@master docker]# docker tag busybox:v1 busybox:v2
[root@master docker]# docker images
REPOSITORY                             TAG      IMAGE ID       CREATED        SIZE
docker.io/library/busybox              mytag    a34cc20fa773   6 weeks ago    1.37 MB
busybox                                v1       a34cc20fa773   6 weeks ago    1.37 MB
docker.io/library/busybox              latest   a34cc20fa773   6 weeks ago    1.37 MB
busybox                                v2       a34cc20fa773   6 weeks ago    1.37 MB
```

### 创建私有仓库镜像

```sh
[root@master docker]# docker tag busybox myregistryhost:5000/jiangbo/busybox:v1
[root@master docker]# docker images
REPOSITORY                             TAG      IMAGE ID       CREATED        SIZE
docker.io/library/busybox              mytag    a34cc20fa773   6 weeks ago    1.37 MB
myregistryhost:5000/jiangbo/busybox    v1       a34cc20fa773   6 weeks ago    1.37 MB
docker.io/library/busybox              latest   a34cc20fa773   6 weeks ago    1.37 MB
```

## 总结

介绍了 tag 命令的使用，可以新建镜像的标签。
