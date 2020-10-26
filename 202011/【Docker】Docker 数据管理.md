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

### 启动一个挂载数据卷的容器

```sh
[root@master ~]# docker run -d -P --name web --mount type=volume,source=my-volume,target=/usr/share/nginx/html nginx:alpine
5fa669ef06ecf858a760ab053bcb2fe804e98df8bd66b7e9dc556d24cc73caf4
[root@master ~]# docker ps
CONTAINER ID  IMAGE                           COMMAND               CREATED         STATUS            PORTS                  NAMES
5fa669ef06ec  docker.io/library/nginx:alpine  nginx -g daemon o...  17 seconds ago  Up 5 seconds ago  0.0.0.0:35727->80/tcp  web
[root@master ~]# curl localhost:35727
<!DOCTYPE html>
<html>
<head>
<title>Welcome to nginx!</title>
<style>
    body {
        width: 35em;
        margin: 0 auto;
        font-family: Tahoma, Verdana, Arial, sans-serif;
    }

<!DOCTYPE html>
</style>
</head>
<body>
<h1>Welcome to nginx!</h1>
<p>If you see this page, the nginx web server is successfully installed and
working. Further configuration is required.</p>

<p>For online documentation and support please refer to
<a href="http://nginx.org/">nginx.org</a>.<br/>
Commercial support is available at
<a href="http://nginx.com/">nginx.com</a>.</p>

<p><em>Thank you for using nginx.</em></p>
</body>
</html>
```

### 修改数据卷的内容

```sh
[root@master ~]# docker volume inspect my-volume
[
     {
          "Name": "my-volume",
          "Driver": "local",
          "Mountpoint": "/var/lib/containers/storage/volumes/my-volume/_data",
          "CreatedAt": "2020-10-23T16:56:50.967850159+08:00",
          "Labels": {

          },
          "Scope": "local",
          "Options": {

          }
     }
]
[root@master ~]# cd /var/lib/containers/storage/volumes/my-volume/_data
[root@master _data]# ls
50x.html  index.html
[root@master _data]# vi index.html
[root@master _data]# vim index.html
[root@master _data]# curl localhost:35727
<!DOCTYPE html>
<html>
<head>
<title>Welcome to nginx!</title>
<style>
    body {
        width: 35em;
        margin: 0 auto;
        font-family: Tahoma, Verdana, Arial, sans-serif;
    }
</style>
</head>
<body>
<h1>Welcome to nginx!Hello world!</h1>
<p>If you see this page, the nginx web server is successfully installed and
working. Further configuration is required.</p>

<p>For online documentation and support please refer to
<a href="http://nginx.org/">nginx.org</a>.<br/>
Commercial support is available at
<a href="http://nginx.com/">nginx.com</a>.</p>

<p><em>Thank you for using nginx.</em></p>
</body>
</html>
[root@master _data]#
```

### 删除数据卷

```sh
[root@master _data]# docker ps
CONTAINER ID  IMAGE                           COMMAND               CREATED        STATUS            PORTS                  NAMES
5fa669ef06ec  docker.io/library/nginx:alpine  nginx -g daemon o...  7 minutes ago  Up 6 minutes ago  0.0.0.0:35727->80/tcp  web
[root@master _data]# docker stop 5f
5fa669ef06ecf858a760ab053bcb2fe804e98df8bd66b7e9dc556d24cc73caf4
[root@master ~]# docker rm 5f
5fa669ef06ecf858a760ab053bcb2fe804e98df8bd66b7e9dc556d24cc73caf4
[root@master ~]# docker volume rm my-volume
my-volume
[root@master ~]#
```

### 删除所有未使用的数据卷

```sh
[root@master ~]# docker volume create vol1
^[[Avol1
[root@master ~]# docker volume create vol2
vol2
[root@master ~]# docker volume create vol3
vol3
[root@master ~]# docker volume prune
WARNING! This will remove all volumes not used by at least one container.
Are you sure you want to continue? [y/N] y
ed5e6609ebfce46c374a25067a2b25a1d16a57f833c68126fd5bde5aab9c50f4
vol1
vol2
vol3
[root@master ~]#
```

## 挂载主机目录

### 挂载一个主机目录作为数据卷

使用 --mount 标记可以指定挂载一个本地主机的目录到容器中去。

```sh
$ docker run -d -P \
    --name web \
    # -v /src/webapp:/usr/share/nginx/html \
    --mount type=bind,source=/src/webapp,target=/usr/share/nginx/html \
    nginx:alpine
```

上面的命令加载主机的 /src/webapp 目录到容器的 /usr/share/nginx/html目录。这个功能在进行测试的时候十分方便，比如用户可以放置一些程序到本地目录中，来查看容器是否正常工作。本地目录的路径必须是绝对路径，以前使用 -v 参数时如果本地目录不存在 Docker 会自动为你创建一个文件夹，现在使用 --mount 参数时如果本地目录不存在，Docker 会报错。

Docker 挂载主机目录的默认权限是 读写，用户也可以通过增加 readonly 指定为只读。

```sh
$ docker run -d -P \
    --name web \
    # -v /src/webapp:/usr/share/nginx/html:ro \
    --mount type=bind,source=/src/webapp,target=/usr/share/nginx/html,readonly \
    nginx:alpine
```

加了 readonly 之后，就挂载为 只读 了。如果你在容器内 /usr/share/nginx/html 目录新建文件，会显示如下错误

```sh
/usr/share/nginx/html # touch new.txt
touch: new.txt: Read-only file system
```

### 查看数据卷的具体信息

```json
"Mounts": [
    {
        "Type": "bind",
        "Source": "/src/webapp",
        "Destination": "/usr/share/nginx/html",
        "Mode": "",
        "RW": true,
        "Propagation": "rprivate"
    }
]
```

### 挂载一个本地主机文件作为数据卷

--mount 标记也可以从主机挂载单个文件到容器中

```sh
$ docker run --rm -it \
   # -v $HOME/.bash_history:/root/.bash_history \
   --mount type=bind,source=$HOME/.bash_history,target=/root/.bash_history \
   ubuntu:18.04 \
   bash

root@2affd44b4667:/# history
1  ls
2  diskutil list
```

这样就可以记录在容器输入过的命令了。

## 总结

介绍了数据管理的两种类型，一种是通过数据卷的方式，另外一种是通过挂载主机目录或者文件的方式。

