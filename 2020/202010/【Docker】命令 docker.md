# 【Docker】命令 docker

参考教程：https://docs.docker.com/engine/reference/commandline/cli/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker [OPTIONS] COMMAND [ARG...]`

使用不带参数的 `docker` 命令或者使用 `docker help` 可以查看所有可以使用的命令和参数。

## 命令选项

### config

客户端配置文件的位置，默认在 "/root/.docker" 目录下。Docker 配置生效的顺序如下：

1. 直接的命令行选项
2. docker 预定义的环境变量
3. config 配置文件

### context

可以指定需要连接 docker 守护程序，短形式是 `-c`。

### debug

短形式 `-D`，可以开启 debug 模式。

### help

`docker --help` 可以查看命令的使用方式。

### host

短形式 `-H`，指定连接的 docker 守护程序。

### log-level

短形式 `-l`，可以指定日志的等级，有五种等级：debug，info，warn，error 和 fatal，默认是 info。

### tls

其中和证书相关选项有

*  --tls
* tlscacert
* tlscert
* tlskey
* tlsverify

这里不展开，用到了在学。

### version

短形式 `-v`，可以查看版本信息。

## 示例

### 获取使用帮助

```sh
[root@node1 ~]# docker --help

Usage:  docker [OPTIONS] COMMAND

A self-sufficient runtime for containers

Options:
      --config string      Location of client config files (default "/root/.docker")
  -c, --context string     Name of the context to use to connect to the daemon
                           (overrides DOCKER_HOST env var and default context set
                           with "docker context use")
  -D, --debug              Enable debug mode
  -H, --host list          Daemon socket(s) to connect to
  -l, --log-level string   Set the logging level
                           ("debug"|"info"|"warn"|"error"|"fatal") (default "info")
      --tls                Use TLS; implied by --tlsverify
      --tlscacert string   Trust certs signed only by this CA (default
                           "/root/.docker/ca.pem")
      --tlscert string     Path to TLS certificate file (default
                           "/root/.docker/cert.pem")
      --tlskey string      Path to TLS key file (default "/root/.docker/key.pem")
      --tlsverify          Use TLS and verify the remote
  -v, --version            Print version information and quit

Management Commands:
  builder     Manage builds
...
```

### 客户端 debug 模式

```sh
[root@node1 ~]# docker -D info
Client:
 Debug Mode: true

Server:
 Containers: 226
  Running: 112
  Paused: 0
  Stopped: 114
 Images: 188
 Server Version: 19.03.12
 Storage Driver: overlay2
...
```

## 总结

介绍了 docker 命令的基本格式，和一些参数选项。对于不了解的指令，可以使用 `--help` 查看使用帮助。

