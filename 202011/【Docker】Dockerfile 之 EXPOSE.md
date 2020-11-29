# 【Docker】Dockerfile 之 EXPOSE

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## EXPOSE

```Dockerfile
EXPOSE <port> [<port>/<protocol>...]
```

The `EXPOSE` instruction informs Docker that the container listens on the specified network ports at runtime. You can specify whether the port listens on TCP or UDP, and the default is TCP if the protocol is not specified.

EXPOSE 指令通知 Docker 容器在运行时侦听指定的网络端口。您可以指定端口是侦听 TCP 还是 UDP，如果未指定协议，则默认值为 TCP。

The `EXPOSE` instruction does not actually publish the port. It functions as a type of documentation between the person who builds the image and the person who runs the container, about which ports are intended to be published. To actually publish the port when running the container, use the `-p` flag on `docker run` to publish and map one or more ports, or the `-P` flag to publish all exposed ports and map them to high-order ports.

`EXPOSE` 指令实际上并未发布端口。它充当构建镜像的人员和运行容器的人员之间的一种文档类型，有关打算发布哪些端口的信息。要在运行容器时实际发布端口，请在 `docker run` 上使用 `-p` 标志来发布和映射一个或多个端口，或使用 `-P` 标志来发布所有公开的端口并将它们映射为高阶端口。

By default, `EXPOSE` assumes TCP. You can also specify UDP:

默认情况下，`EXPOSE` 采用 TCP。您还可以指定 UDP：

```
EXPOSE 80/udp
```

To expose on both TCP and UDP, include two lines:

要同时在 TCP 和 UDP 上公开，请包括以下两行：

```
EXPOSE 80/tcp
EXPOSE 80/udp
```

In this case, if you use `-P` with `docker run`, the port will be exposed once for TCP and once for UDP. Remember that `-P` uses an ephemeral high-ordered host port on the host, so the port will not be the same for TCP and UDP.

在这种情况下，如果将 `-P` 和 `docker run` 一起使用，则该端口将仅对 TCP 公开一次，对于 UDP 公开一次。请记住，`-P` 在主机上使用临时的高阶主机端口，因此该端口对于 TCP 和 UDP 将是不同的。

Regardless of the `EXPOSE` settings, you can override them at runtime by using the `-p` flag. For example

无论使用哪种 `EXPOSE` 设置，都可以在运行时使用 `-p` 标志覆盖它们。例如

```
docker run -p 80:80/tcp -p 80:80/udp ...
```

To set up port redirection on the host system, see [using the -P flag](https://docs.docker.com/engine/reference/run/#expose-incoming-ports). The `docker network` command supports creating networks for communication among containers without the need to expose or publish specific ports, because the containers connected to the network can communicate with each other over any port. For detailed information, see the [overview of this feature](https://docs.docker.com/engine/userguide/networking/).

要在主机系统上设置端口重定向，请参阅[使用 `-P` 标志](https://docs.docker.com/engine/reference/run/#expose-incoming-ports)。`docker network` 命令支持创建网络以在容器之间进行通信，而无需暴露或发布特定端口，因为连接到网络的容器可以通过任何端口相互通信。有关详细信息，请参阅[此功能概述](https://docs.docker.com/engine/userguide/networking/)。

## 示例

### Dockerfile 文件

```Dockerfile
FROM openjdk:8-jdk-alpine
ARG JAR_FILE=*.jar
COPY ${JAR_FILE} app.jar
EXPOSE 8080
ENTRYPOINT ["java","-jar","/app.jar"]
```

### 构建结果

```sh
[root@master demo1]# docker build -t jiangbo:0.0.1 .
Sending build context to Docker daemon    547MB
Step 1/5 : FROM openjdk:8-jdk-alpine
 ---> a3562aa0b991
Step 2/5 : ARG JAR_FILE=*.jar
 ---> Running in fb006ab9edc3
Removing intermediate container fb006ab9edc3
 ---> a270e9ae613b
Step 3/5 : COPY ${JAR_FILE} app.jar
 ---> 9081948fad4a
Step 4/5 : EXPOSE 8080
 ---> Running in f1b492d6fd16
Removing intermediate container f1b492d6fd16
 ---> a169c40b22ef
Step 5/5 : ENTRYPOINT ["java","-jar","/app.jar"]
 ---> Running in 58cbac48f911
Removing intermediate container 58cbac48f911
 ---> 8bb974935c05
Successfully built 8bb974935c05
Successfully tagged jiangbo:0.0.1
```

### 查看结果

```sh
[root@master demo1]# docker run -d -p:8080:8080 jiangbo:0.0.1
368f4661e8ba62d73cf1f07b84830f5496ab6a7a27fff075fbe24b15279a8933
[root@master demo1]# curl localhost:8080
Hello Docker World
[root@master demo1]# docker run -d -P jiangbo:0.0.1
95a7dd17d06e352a890161937d1bda598cd6c4444425c39493b0d6371b40d71b
[root@master demo1]# docker port 95
8080/tcp -> 0.0.0.0:32768
[root@master demo1]# curl localhost:32768
Hello Docker World
```


## 总结

介绍了 Dockerfile 中 EXPOSE 指令的使用。