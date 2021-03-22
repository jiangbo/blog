# 【Docker】Dockerfile 最佳实践-EXPOSE

参考教程：https://docs.docker.com/develop/develop-images/dockerfile_best-practices/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## EXPOSE

The `EXPOSE` instruction indicates the ports on which a container listens for connections. Consequently, you should use the common, traditional port for your application. For example, an image containing the Apache web server would use `EXPOSE 80`, while an image containing MongoDB would use `EXPOSE 27017` and so on.

`EXPOSE` 指令指示容器侦听连接的端口。因此，您应该为应用程序使用通用的传统端口。例如，包含 Apache Web 服务器的镜像将使 `EXPOSE 80`，而包含 MongoDB 的镜像将使用 `EXPOSE 27017`，依此类推。

For external access, your users can execute `docker run` with a flag indicating how to map the specified port to the port of their choice. For container linking, Docker provides environment variables for the path from the recipient container back to the source (ie, `MYSQL_PORT_3306_TCP`).

对于外部访问，您的用户可以执行带有标志的 `docker run` 命令，该标志指示如何将指定端口映射到他们选择的端口。对于容器链接，Docker 为从接收者容器到源容器的路径提供了环境变量（即，`MYSQL_PORT_3306_TCP`）。

## 总结

介绍了 Dockerfile 的 EXPOSE 指令的最佳实践。