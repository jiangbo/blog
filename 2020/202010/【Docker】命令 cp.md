# 【Docker】命令 cp

参考教程：https://docs.docker.com/engine/reference/commandline/cp/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker cp [OPTIONS] SRC_PATH|- CONTAINER:DEST_PATH`

使用 cp 命令可以将容器内的文件拷贝到宿主机，或者从宿主机拷贝文件到容器中。

## 示例

### 从容器拷贝到主机

```sh
[root@master ~]# docker ps
CONTAINER ID        IMAGE               COMMAND                  CREATED             STATUS              PORTS               NAMES
1d06decda260        nginx               "/docker-entrypoint.…"   46 hours ago        Up 8 minutes        80/tcp              strange_mendel
[root@master ~]# docker cp 1d:/usr/share/nginx/html/index.html ~
[root@master ~]# ls
anaconda-ks.cfg  docker  index.html
[root@master ~]# cat index.html
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

### 从主机拷贝到容器

```sh
[root@master ~]# vim index.html
[root@master ~]# docker cp index 1d:/usr/share/nginx/html
lstat /root/index: no such file or directory
[root@master ~]# docker cp index.html 1d:/usr/share/nginx/html/
[root@master ~]#
root@1d06decda260:/usr/share/nginx/html# curl localhost:80
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
<h1>Welcome to docker!</h1>
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

## 总结

介绍了 cp 命令的使用，可以将文件从容器拷贝出来或者拷贝进去。
