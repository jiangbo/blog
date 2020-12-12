# 【Docker】Dockerfile 最佳实践-VOLUME

参考教程：https://docs.docker.com/develop/develop-images/dockerfile_best-practices/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## VOLUME

The `VOLUME` instruction should be used to expose any database storage area, configuration storage, or files/folders created by your docker container. You are strongly encouraged to use `VOLUME` for any mutable and/or user-serviceable parts of your image.

`VOLUME` 指令应用于公开由 Docker 容器创建的任何数据库存储区，配置存储或文件/文件夹。强烈建议您将 `VOLUME` 用于镜像的任何可变和/或用户可维修的部分。

## 总结

介绍了 Dockerfile 的 VOLUME 指令的最佳实践。