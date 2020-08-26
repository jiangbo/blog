# 【Docker】命令 start

参考教程：https://docs.docker.com/engine/reference/commandline/start/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker start [OPTIONS] CONTAINER [CONTAINER...]`

使用 start 命令可以启动一个容器。

## 命令选项

由于命令选项有很多，下面选择几个常用的进行学习。

| 名称 | 默认值 | 描述 |
| --- | --- | --- |
| `--attach , -a` |  | 连接到标准输出流 |
| `--detach-keys` |  | 定义后台运行容器的按键 |
| `--interactive , -i` |  | 连接到标准输出流 |

## 示例

### 启动后台容器

```sh
# 拉取镜像
[root@master docker]# docker pull nginx
Using default tag: latest
latest: Pulling from library/nginx
Digest: sha256:b0ad43f7ee5edbc0effbc14645ae7055e21bc1973aee5150745632a24a752661
Status: Image is up to date for nginx:latest
docker.io/library/nginx:latest
# 创建容器
[root@master docker]# docker create nginx
bf0e4bfd796c43f53b052611bac5c260d457606dd9150c512ef48137962c2d04
# 启动容器
[root@master docker]# docker start bf
bf
# 查看容器 IP
[root@master docker]# docker inspect bf -f '{{json .NetworkSettings.Networks.bridge.IPAddress}}'
"172.17.0.3"
# 访问 nginx
[root@master docker]# curl 172.17.0.3:80
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

### 启动交互式容器

```sh
[root@master docker]# docker create -it busybox /bin/sh
69575a4213c1ace85b198f3a94b50acc21a3e96ee3429b48ef9d72a39f0f250a
[root@master docker]# docker start -i 69
/ # ls
bin   dev   etc   home  proc  root  sys   tmp   usr   var
/ #
```

## 总结

介绍了 start 命令的使用，可以运行一个创建好的容器，同时也可以运行已经停止的容器。
