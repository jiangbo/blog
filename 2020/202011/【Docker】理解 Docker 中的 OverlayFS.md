# 【Docker】理解 Docker 中的 OverlayFS

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## overlay 挂载

先在宿主机上模拟 overlay 挂载。

### 建立需要的目录

```sh
[root@master overlay-fs]# tree .
.
├── lower1
│   └── l1.txt
├── lower2
│   ├── l2.txt
│   └── l3.txt
├── lower3
│   └── l3.txt
├── merged
├── upper
└── work

6 directories, 4 files
[root@master overlay-fs]# cat lower1/l1.txt
lower1
[root@master overlay-fs]# cat lower2/l2.txt
lower2
[root@master overlay-fs]# cat lower2/l3.txt
lower3 in lower 2
[root@master overlay-fs]# cat lower3/l3.txt
lower3
[root@master overlay-fs]#
```

### 执行挂载命令

```sh
[root@master overlay-fs]# mount -t overlay overlay -o lowerdir=lower1:lower2:lower3,upperdir=upper,workdir=work merged/
[root@master overlay-fs]# ls merged/
l1.txt  l2.txt  l3.txt
[root@master overlay-fs]# cat merged/l2.txt
lower2
[root@master overlay-fs]# cat merged/l3.txt
lower3 in lower 2
[root@master overlay-fs]#
```

### 查看挂载信息

```sh
[root@master overlay-fs]#  mount | grep overlay-fs
overlay on /root/overlay-fs/merged type overlay (rw,relatime,seclabel,lowerdir=lower1:lower2:lower3,upperdir=upper,workdir=work)
```

![overlay_constructs][1]

## 镜像层

### 获取镜像

```sh
[root@master ~]# docker pull ubuntu
Using default tag: latest
latest: Pulling from library/ubuntu
6a5697faee43: Pull complete
ba13d3bc422b: Pull complete
a254829d9e55: Pull complete
Digest: sha256:fff16eea1a8ae92867721d90c59a75652ea66d29c05294e6e2f898704bdb8cf1
Status: Downloaded newer image for ubuntu:latest
docker.io/library/ubuntu:latest
[root@master ~]#
```
可以看到，总共从服务器拉取了三层。

### 查看构建纪录

```sh
[root@master ~]# docker history ubuntu:latest
IMAGE               CREATED             CREATED BY                                      SIZE                COMMENT
d70eaf7277ea        4 weeks ago         /bin/sh -c #(nop)  CMD ["/bin/bash"]            0B
<missing>           4 weeks ago         /bin/sh -c mkdir -p /run/systemd && echo 'do…   7B
<missing>           4 weeks ago         /bin/sh -c [ -z "$(apt-get indextargets)" ]     0B
<missing>           4 weeks ago         /bin/sh -c set -xe   && echo '#!/bin/sh' > /…   811B
<missing>           4 weeks ago         /bin/sh -c #(nop) ADD file:435d9776fdd3a1834…   72.9MB
```

没有修改的话，不会生成镜像层。

### 查看镜像层

```sh
[root@master ~]# docker image inspect ubuntu:latest -f '{{json .GraphDriver}}' | jq
{
  "Data": {
    "LowerDir": "/var/lib/docker/overlay2/d4ae655796da84cd446b1d9b3f4af81a2128e0dd4f6bb7f4549e877847ac2191/diff:/var/lib/docker/overlay2/cc17e7fa7371b562c3638dccc2c19439d4a7900e8b9e1207bcaa767a2bfde6f8/diff",
    "MergedDir": "/var/lib/docker/overlay2/904fc870070ff53322419097027f739bc558c2e89443ee7530cd850143e6041b/merged",
    "UpperDir": "/var/lib/docker/overlay2/904fc870070ff53322419097027f739bc558c2e89443ee7530cd850143e6041b/diff",
    "WorkDir": "/var/lib/docker/overlay2/904fc870070ff53322419097027f739bc558c2e89443ee7530cd850143e6041b/work"
  },
  "Name": "overlay2"
}
[root@master ~]# docker image inspect ubuntu:latest -f '{{json .RootFS}}' | jq
{
  "Type": "layers",
  "Layers": [
    "sha256:47dde53750b4a8ed24acebe52cf31ad131e73a9611048fc2f92c9b6274ab4bf3",
    "sha256:0c2689e3f9206b1c4adfb16a1976d25bd270755e734588409b31ef29e3e756d6",
    "sha256:cc9d18e90faad04bc3893cfaa50b7846ee75f48f5b8377a213fa52af2189095c"
  ]
}
```

从 RootFS 和 GraphDriver 可以看到，镜像包括三层，其中 lowerdir 有两层。

### 镜像层详细

```sh
[root@master ~]# ls /var/lib/docker/overlay2/cc17e7fa7371b562c3638dccc2c19439d4a7900e8b9e1207bcaa767a2bfde6f8/diff
bin  boot  dev  etc  home  lib  lib32  lib64  libx32  media  mnt  opt  proc  root  run  sbin  srv  sys  tmp  usr  var
[root@master ~]# ls /var/lib/docker/overlay2/d4ae655796da84cd446b1d9b3f4af81a2128e0dd4f6bb7f4549e877847ac2191/diff
etc  usr  var
[root@master ~]# ls /var/lib/docker/overlay2/904fc870070ff53322419097027f739bc558c2e89443ee7530cd850143e6041b/diff
run
[root@master ~]#
[root@master ~]# ls /var/lib/docker/overlay2/
904fc870070ff53322419097027f739bc558c2e89443ee7530cd850143e6041b  cc17e7fa7371b562c3638dccc2c19439d4a7900e8b9e1207bcaa767a2bfde6f8  l
backingFsBlockDev                                                 d4ae655796da84cd446b1d9b3f4af81a2128e0dd4f6bb7f4549e877847ac2191
```

## 容器层

### 启动容器

```sh
[root@master ~]# docker run -d ubuntu:latest sleep 3600
ea3e818e7690523e5758bf529bc50bd92d68ad26e02f4953bdb68f9bc5ecd567
[root@master ~]#
```

### 查看镜像层

```sh
[root@master ~]# docker container inspect ea -f '{{json .GraphDriver}}' | jq
{
  "Data": {
    "LowerDir": "/var/lib/docker/overlay2/1e8611585b1c2a4d6066d0829c70a2fb7b0e6da64e9015e81201c0b5c7fa9402-init/diff:/var/lib/docker/overlay2/904fc870070ff53322419097027f739bc558c2e89443ee7530cd850143e6041b/diff:/var/lib/docker/overlay2/d4ae655796da84cd446b1d9b3f4af81a2128e0dd4f6bb7f4549e877847ac2191/diff:/var/lib/docker/overlay2/cc17e7fa7371b562c3638dccc2c19439d4a7900e8b9e1207bcaa767a2bfde6f8/diff",
    "MergedDir": "/var/lib/docker/overlay2/1e8611585b1c2a4d6066d0829c70a2fb7b0e6da64e9015e81201c0b5c7fa9402/merged",
    "UpperDir": "/var/lib/docker/overlay2/1e8611585b1c2a4d6066d0829c70a2fb7b0e6da64e9015e81201c0b5c7fa9402/diff",
    "WorkDir": "/var/lib/docker/overlay2/1e8611585b1c2a4d6066d0829c70a2fb7b0e6da64e9015e81201c0b5c7fa9402/work"
  },
  "Name": "overlay2"
}
```

可以看到，lower的底下三层刚好是镜像的三层，除此之外，还有一个 init 层，在 upperdir 中有一个可写的容器层。

整个容器的层数可以分为三个部分。第一个部分是只读的镜像层，对应是 ubuntu 镜像的三层。第二部分是 init 层。第三层就是容器的可读写层。


### init 层

```sh
[root@master ~]# ls /var/lib/docker/overlay2/
1e8611585b1c2a4d6066d0829c70a2fb7b0e6da64e9015e81201c0b5c7fa9402       backingFsBlockDev                                                 l
1e8611585b1c2a4d6066d0829c70a2fb7b0e6da64e9015e81201c0b5c7fa9402-init  cc17e7fa7371b562c3638dccc2c19439d4a7900e8b9e1207bcaa767a2bfde6f8
904fc870070ff53322419097027f739bc558c2e89443ee7530cd850143e6041b       d4ae655796da84cd446b1d9b3f4af81a2128e0dd4f6bb7f4549e877847ac2191
[root@master ~]#
[root@master ~]# ls /var/lib/docker/overlay2/1e8611585b1c2a4d6066d0829c70a2fb7b0e6da64e9015e81201c0b5c7fa9402-init/diff/etc/
hostname  hosts  mtab  resolv.conf
[root@master ~]#
```

init 层是一个以“-init”结尾的层，夹在只读层和读写层之间。Init 层是 Docker 项目单独生成的一个内部层，专门用来存放 /etc/hosts、/etc/resolv.conf 等信息。需要这样一层的原因是，这些文件本来属于只读的 Ubuntu 镜像的一部分，但是用户往往需要在启动容器时写入一些指定的值比如 hostname，所以就需要在可读写层对它们进行修改。可是，这些修改往往只对当前的容器有效，我们并不希望执行 docker commit 时，把这些信息连同可读写层一起提交掉。所以，Docker 做法是，在修改了这些文件之后，以一个单独的层挂载了出来。而用户执行 docker commit 只会提交可读写层，所以是不包含这些内容的。

### 容器可读写层

```sh
[root@master 1e8611585b1c2a4d6066d0829c70a2fb7b0e6da64e9015e81201c0b5c7fa9402]# cd diff/
[root@master diff]# ls
[root@master diff]# docker exec -it ea bash
root@ea3e818e7690:/# ls
bin  boot  dev  etc  home  lib  lib32  lib64  libx32  media  mnt  opt  proc  root  run  sbin  srv  sys  tmp  usr  var
root@ea3e818e7690:/# touch jiangbo.txt
root@ea3e818e7690:/# exit
exit
[root@master diff]# ls
jiangbo.txt  root
[root@master diff]#
```

![container-layers][2]


## RootFS 和 GraphDriver 的关联

```sh
[root@master sha256]# cd /var/lib/docker/image/overlay2/layerdb/sha256
[root@master sha256]# ll
total 0
drwx------. 2 root root 71 Nov 25 00:03 47dde53750b4a8ed24acebe52cf31ad131e73a9611048fc2f92c9b6274ab4bf3
drwx------. 2 root root 85 Nov 25 00:03 7011438f48b79cbf5fce3bfba74aed2e53fe5fe6a3b7fd6fe03018d28caee7a3
drwx------. 2 root root 85 Nov 25 00:03 778d52487737cf5362fd95086fa55793001dbc7b331344a540a594824e2994fd
[root@master sha256]# cat 47dde53750b4a8ed24acebe52cf31ad131e73a9611048fc2f92c9b6274ab4bf3/cache-id
cc17e7fa7371b562c3638dccc2c19439d4a7900e8b9e1207bcaa767a2bfde6f8
[root@master sha256]# cat 47dde53750b4a8ed24acebe52cf31ad131e73a9611048fc2f92c9b6274ab4bf3/diff
sha256:47dde53750b4a8ed24acebe52cf31ad131e73a9611048fc2f92c9b6274ab4bf3
[root@master sha256]# cat 7011438f48b79cbf5fce3bfba74aed2e53fe5fe6a3b7fd6fe03018d28caee7a3/cache-id
d4ae655796da84cd446b1d9b3f4af81a2128e0dd4f6bb7f4549e877847ac2191
[root@master sha256]# cat 7011438f48b79cbf5fce3bfba74aed2e53fe5fe6a3b7fd6fe03018d28caee7a3/diff
sha256:0c2689e3f9206b1c4adfb16a1976d25bd270755e734588409b31ef29e3e756d6
[root@master sha256]# cat 778d52487737cf5362fd95086fa55793001dbc7b331344a540a594824e2994fd/cache-id
904fc870070ff53322419097027f739bc558c2e89443ee7530cd850143e6041b
[root@master sha256]# cat 778d52487737cf5362fd95086fa55793001dbc7b331344a540a594824e2994fd/diff
sha256:cc9d18e90faad04bc3893cfaa50b7846ee75f48f5b8377a213fa52af2189095c
[root@master sha256]#
{
  "Data": {
    "LowerDir": "/var/lib/docker/overlay2/d4ae655796da84cd446b1d9b3f4af81a2128e0dd4f6bb7f4549e877847ac2191/diff:/var/lib/docker/overlay2/cc17e7fa7371b562c3638dccc2c19439d4a7900e8b9e1207bcaa767a2bfde6f8/diff",
    "MergedDir": "/var/lib/docker/overlay2/904fc870070ff53322419097027f739bc558c2e89443ee7530cd850143e6041b/merged",
    "UpperDir": "/var/lib/docker/overlay2/904fc870070ff53322419097027f739bc558c2e89443ee7530cd850143e6041b/diff",
    "WorkDir": "/var/lib/docker/overlay2/904fc870070ff53322419097027f739bc558c2e89443ee7530cd850143e6041b/work"
  },
  "Name": "overlay2"
}
[root@master ~]# docker image inspect ubuntu:latest -f '{{json .RootFS}}' | jq
{
  "Type": "layers",
  "Layers": [
    "sha256:47dde53750b4a8ed24acebe52cf31ad131e73a9611048fc2f92c9b6274ab4bf3",
    "sha256:0c2689e3f9206b1c4adfb16a1976d25bd270755e734588409b31ef29e3e756d6",
    "sha256:cc9d18e90faad04bc3893cfaa50b7846ee75f48f5b8377a213fa52af2189095c"
  ]
}
```

## 总结

介绍了 overlay 的实现方式，以及在容器中的如何处理文件的增删改查等。

[1]:images/overlay_constructs.jpg
[2]:images/container-layers.png 