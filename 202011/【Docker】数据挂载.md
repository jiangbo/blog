# 【Docker】绑定挂载

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 绑定挂载的操作

挂载的类型为 bind，需要提供原目录和目标目录进行挂载，也可以是文件，路径必须是绝对路径。

```sh
[root@master ~]# docker run --name web -P -d --mount type=bind,src=/root/html,dst=/usr/share/nginx/html nginx:alpine
Unable to find image 'nginx:alpine' locally
alpine: Pulling from library/nginx
188c0c94c7c5: Pull complete 
61c2c0635c35: Pull complete 
378d0a9d4d5f: Pull complete 
2fe865f77305: Pull complete 
b92535839843: Pull complete 
Digest: sha256:5aa44b407756b274a600c7399418bdfb1d02c33317ae27fd5e8a333afb115db1
Status: Downloaded newer image for nginx:alpine
497d0139b30750b8ce9c87df8802f48ead5dc5f1e18f29e8e52a18af8d8c8cf2
[root@master ~]# docker ps
CONTAINER ID        IMAGE               COMMAND                  CREATED             STATUS              PORTS                   NAMES
497d0139b307        nginx:alpine        "/docker-entrypoint.…"   7 seconds ago       Up 5 seconds        0.0.0.0:32769->80/tcp   web
```

## 搭建开发环境

使用 bind 可以使用容器来搭建开发环境，特别适用于不想或者不能在本地安装各种软件的情况，下面是具体的操作。

### 启动 nginx 服务器

```sh
[root@master ~]# docker run --name web -P -d --mount type=bind,src=/root/html,dst=/usr/share/nginx/html nginx:alpine
Unable to find image 'nginx:alpine' locally
alpine: Pulling from library/nginx
188c0c94c7c5: Pull complete 
61c2c0635c35: Pull complete 
378d0a9d4d5f: Pull complete 
2fe865f77305: Pull complete 
b92535839843: Pull complete 
Digest: sha256:5aa44b407756b274a600c7399418bdfb1d02c33317ae27fd5e8a333afb115db1
Status: Downloaded newer image for nginx:alpine
497d0139b30750b8ce9c87df8802f48ead5dc5f1e18f29e8e52a18af8d8c8cf2
[root@master ~]# docker ps
CONTAINER ID        IMAGE               COMMAND                  CREATED             STATUS              PORTS                   NAMES
497d0139b307        nginx:alpine        "/docker-entrypoint.…"   7 seconds ago       Up 5 seconds        0.0.0.0:32769->80/tcp   web
```

### 使用 vscode 远程连接

使用 vscode 远程连接到服务器，并打开刚刚挂载的目录，在目录新增 html 文件。

![vscode-bind.png][1]

### 访问网页

访问网页查看效果如下：

![nginx-hello.png][2]

## tmpfs

> --tmpfs 也可以实现挂载 tmpfs，推荐使用 mount。

--tmpfs 不支持配置选项，也不支持 swarm。

### 创建 tmpfs 挂载

```sh
[root@master ~]# docker run -d \
>   -it \
>   --name tmptest \
>   --mount type=tmpfs,destination=/app \
>   nginx:latest
b1929d6811d5d42e2a22d55a8aa2ffda7c083253487800e883b24bdd30884c5c
```

### 验证挂载类型

```sh
[root@master ~]# docker container inspect tmptest --format '{{json .Mounts}}'|jq
[
  {
    "Type": "tmpfs",
    "Source": "",
    "Destination": "/app",
    "Mode": "",
    "RW": true,
    "Propagation": ""
  }
]
```

### 指定参数

--tmpfs-size：tmpfs 占用的字节数，默认无限。
--tmpfs-mode：八进制的文件权限模式，默认 1777。

```sh
[root@master ~]# docker run -d \
>   -it \
>   --name tmptest \
>   --mount type=tmpfs,destination=/app,tmpfs-mode=1770 \
>   nginx:latest
0a33d1388325e5dbcd0f051dbb25a82c6433b8d0c9844067e8a89021bc8c9d46
[root@master ~]# docker container inspect tmptest --format '{{json .Mounts}}'|jq
[
  {
    "Type": "tmpfs",
    "Source": "",
    "Destination": "/app",
    "Mode": "",
    "RW": true,
    "Propagation": ""
  }
]
```

### 验证权限

```sh
[root@master ~]# docker exec -it tmptest bash
root@0a33d1388325:/# cd /
root@0a33d1388325:/# ls -l
total 12
drwxrwx--T.   2 root root   40 Nov  2 15:44 app
drwxr-xr-x.   2 root root 4096 Aug  3 07:00 bin
drwxr-xr-x.   2 root root    6 Jul 10 21:04 boot
drwxr-xr-x.   5 root root  360 Nov  2 15:44 dev
drwxr-xr-x.   1 root root   41 Aug 14 00:36 docker-entrypoint.d
-rwxrwxr-x.   1 root root 1202 Aug 14 00:36 docker-entrypoint.sh
```
> T 权限，针对目录，可以对自己的文件进行增删改查，不能删除别人的。

## 总结

介绍了数据挂载的使用，包括使用 bind 简化开发环境的搭建，tmpfs 的使用。

[1]: images/vscode-bind.png
[2]: images/nginx-hello.png