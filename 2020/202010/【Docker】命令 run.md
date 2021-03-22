# 【Docker】命令 run

参考教程：https://docs.docker.com/engine/reference/commandline/run/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker run [OPTIONS] IMAGE [COMMAND] [ARG...]`

使用 run 命令可以直接从镜像启动一个容器，如果镜像在本地不存在，会自动到 registry 中去获取。

## 命令选项

由于命令选项有很多，下面选择几个常用的进行学习。

| 名称 | 默认值 | 描述 |
| --- | --- | --- |

| `--interactive , -i` |  | 标准输入 |
| `--tty , -t` |  | 分配一个伪终端 |
| `--detach , -d` |  | 后台运行容器 |
| `--publish , -p` |  | 映射容器的端口到宿主机 |
| `--publish-all , -P` |  | 映射容器的所有端口到宿主机的随机端口 |

## 示例

### 前台运行容器

```sh
[root@master ~]# docker run -it busybox /bin/sh
/ # ls
bin   dev   etc   home  proc  root  sys   tmp   usr   var
/ # ps -ef
PID   USER     TIME  COMMAND
    1 root      0:00 /bin/sh
    7 root      0:00 ps -ef
/ #
```

### 启动后台容器

```sh
[root@master ~]# docker run -d -p8080:80 nginx
b82ba0c69740581c68cb602f66edb38d7327f63f9f47b0d6002e3763af70d481
[root@master ~]# curl 192.168.56.102:8080
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

![运行 nginx][1]

## 总结

介绍了 run 命令的使用，在运行容器时，可以指定各种参数。

[1]: images/docker-run-nginx.png
