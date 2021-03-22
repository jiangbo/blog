# 【Docker】命令 export 和 import

参考教程：
https://docs.docker.com/engine/reference/commandline/export/
https://docs.docker.com/engine/reference/commandline/import/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker export [OPTIONS] CONTAINER`
`docker import [OPTIONS] file|URL|- [REPOSITORY[:TAG]]`

使用 export 可以导出容器到文件，相当于一个系统的快照。import 可以将导出的文件再次导入生成一个镜像。

## 命令选项

### export 命令选项

| 名称 | 默认值 | 描述 |
| `--output , -o` |  | 写到指定的文件中，而不是标准输出流上 |

### import 命令选项

| 名称 | 默认值 | 描述 |
| --- | --- | --- |
| `--change , -c` |  | 使用 docker 指令创建镜像 |
| `--message , -m` |  | 在导入镜像时设置提交信息 |

## 示例

### 导入容器到文件

```sh
[root@master kafka_2.12-2.5.1]# docker run -p 80:80 --name my_nginx -d nginx
2ccd9d92501d31eeee282c5a3d29f756cb5856f1bf3f51088187e7b1bb9994c8
[root@master kafka_2.12-2.5.1]# docker ps
CONTAINER ID  IMAGE                           COMMAND               CREATED        STATUS            PORTS               NAMES
2ccd9d92501d  docker.io/library/nginx:latest  nginx -g daemon o...  8 seconds ago  Up 4 seconds ago  0.0.0.0:80->80/tcp  my_nginx
[root@master kafka_2.12-2.5.1]# cd
[root@master ~]# docker export -o "my_nginx.tar" 2ccd9d92501d
[root@master ~]# ls
anaconda-ks.cfg  my_nginx.tar
```

### 导入文件成镜像

```sh
[root@master ~]# docker images
REPOSITORY                             TAG      IMAGE ID       CREATED        SIZE
docker.io/library/redis                latest   14e621ff43d4   12 days ago    103 MB
docker.io/library/nginx                latest   15bd5deea75c   4 weeks ago    139 MB
docker.io/library/busybox              latest   a34cc20fa773   6 weeks ago    1.38 MB
docker.io/library/alpine               latest   63081882c6ff   3 months ago   5.68 MB
docker.io/library/hello-world          latest   565456b31eec   8 months ago   19.5 kB
[root@master ~]# docker import --message "commit my nginx" my_nginx.tar my_nginx:1.0.0
Getting image source signatures
Copying blob 9683ce706f99 done
Copying config ac5d6ecaf4 done
Writing manifest to image destination
Storing signatures
ac5d6ecaf4c167193829a5012f3dd625d2521978833802e404d77001b572080d
[root@master ~]# docker images
REPOSITORY                             TAG      IMAGE ID       CREATED         SIZE
docker.io/library/my_nginx             1.0.0    ac5d6ecaf4c1   5 minutes ago   137 MB
docker.io/library/redis                latest   14e621ff43d4   12 days ago     103 MB
docker.io/library/nginx                latest   15bd5deea75c   4 weeks ago     139 MB
docker.io/library/busybox              latest   a34cc20fa773   6 weeks ago     1.38 MB
docker.io/library/alpine               latest   63081882c6ff   3 months ago    5.68 MB
docker.io/library/hello-world          latest   565456b31eec   8 months ago    19.5 kB
```

### 查看提交历史

```sh
[root@master ~]# docker history my_nginx:1.0.0
ID             CREATED          CREATED BY                                      SIZE      COMMENT
ac5d6ecaf4c1   11 minutes ago   /bin/sh -c #(nop) ADD file:9683ce706f9947e...   137.2MB   commit my nginx
[root@master ~]#
```

## 总结

介绍了 export/import 命令的使用，可以将容器导出成文件，并根据文件再次还原为镜像。
