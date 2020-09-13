# 【Docker】命令 rename

参考教程：https://docs.docker.com/engine/reference/commandline/rename/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker rename CONTAINER NEW_NAME`

使用 rename 命令可以重命名 docker 容器的名称。

## 示例

### 重命名容器

```sh
[root@master ~]# docker ps
CONTAINER ID        IMAGE               COMMAND                  CREATED             STATUS              PORTS                  NAMES
b82ba0c69740        nginx               "/docker-entrypoint.…"   4 days ago          Up 10 minutes       0.0.0.0:8080->80/tcp   pedantic_allen
[root@master ~]# docker b82ba0c69740 myNginx
docker: 'b82ba0c69740' is not a docker command.
See 'docker --help'
[root@master ~]# docker rename b82ba0c69740 myNginx
[root@master ~]# docker ps
CONTAINER ID        IMAGE               COMMAND                  CREATED             STATUS              PORTS                  NAMES
b82ba0c69740        nginx               "/docker-entrypoint.…"   4 days ago          Up 10 minutes       0.0.0.0:8080->80/tcp   myNginx
[root@master ~]#
```

## 总结

介绍了 rename 命令的使用，可以重命名容器。

[1]: images/docker-run-nginx.png
