# 【Docker】命令 save 和 load

参考教程：
https://docs.docker.com/engine/reference/commandline/save/
https://docs.docker.com/engine/reference/commandline/load/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker save [OPTIONS] IMAGE [IMAGE...]`
`docker load [OPTIONS]`

使用 save 可以保存镜像到文件，而 load 可以将导出的文件再次导入生成一个镜像。

## 命令选项

### save 命令选项

| 名称 | 默认值 | 描述 |
| `--output , -o` |  | 写到指定的文件中，而不是标准输出流上 |

### load 命令选项

| 名称 | 默认值 | 描述 |
| --- | --- | --- |
| `--input , -i` |  | 从文件加载而不是从标准输入流 |
| `--quiet , -q` |  | 不显示输出信息 |

## 示例

### 保存镜像

```sh
[root@master docker]# docker save busybox > busybox.tar
[root@master docker]# ls -sh busybox.tar
1.4M busybox.tar
[root@master docker]# docker save --output obusybox.tar busybox
[root@master docker]# ls -sh
total 2.7M
1.4M busybox.tar  1.4M obusybox.tar
[root@master docker]#
```

### 使用另外的 tag 保存镜像

```sh
[root@master docker]# docker save -o mytag_busybox.tar busybox busybox:mytag
[root@master docker]# ls -sh
total 4.0M
1.4M busybox.tar  1.4M mytag_busybox.tar  1.4M obusybox.tar
```

### 保存压缩镜像

```sh
[root@master docker]# docker save busybox | gzip > busybox.tar.gz
[root@master docker]# ls -sh
total 4.7M
1.4M busybox.tar  688K busybox.tar.gz  1.4M mytag_busybox.tar  1.4M obusybox.tar
[root@master docker]#
```

### 输入流加载镜像

```sh
[root@master docker]# docker images
REPOSITORY                             TAG      IMAGE ID       CREATED        SIZE
[root@master docker]# docker load < obusybox.tar
Getting image source signatures
Copying blob 988c34d733d9 done
Copying config a34cc20fa7 done
Writing manifest to image destination
Storing signatures
Loaded image(s): docker.io/library/busybox:latest
[root@master docker]# docker images
REPOSITORY                             TAG      IMAGE ID       CREATED        SIZE
docker.io/library/busybox              latest   a34cc20fa773   6 weeks ago    1.37 MB
[root@master docker]#
```

### 加载镜像

```sh
[root@master docker]# docker images
REPOSITORY                             TAG      IMAGE ID       CREATED        SIZE
[root@master docker]# docker load -i obusybox.tar
Getting image source signatures
Copying blob 988c34d733d9 done
Copying config a34cc20fa7 done
Writing manifest to image destination
Storing signatures
Loaded image(s): docker.io/library/busybox:latest
[root@master docker]# docker images
REPOSITORY                             TAG      IMAGE ID       CREATED        SIZE
docker.io/library/busybox              latest   a34cc20fa773   6 weeks ago    1.37 MB
[root@master docker]#
```

### 加载压缩镜像

```sh
[root@master docker]# docker load -i busybox.tar.gz
Getting image source signatures
Copying blob 988c34d733d9 done
Copying config a34cc20fa7 done
Writing manifest to image destination
Storing signatures
Loaded image(s): docker.io/library/busybox:latest
[root@master docker]# docker images
REPOSITORY                             TAG      IMAGE ID       CREATED        SIZE
docker.io/library/busybox              latest   a34cc20fa773   6 weeks ago    1.37 MB
[root@master docker]#
```

### 加载另外 Tag

```sh
[root@master docker]# ls
busybox.tar  busybox.tar.gz  mytag_busybox.tar  obusybox.tar
[root@master docker]# docker load < mytag_busybox.tar
Getting image source signatures
Copying config a34cc20fa7 done
Writing manifest to image destination
Storing signatures
Getting image source signatures
Copying config a34cc20fa7 done
Writing manifest to image destination
Storing signatures
Loaded image(s): docker.io/library/busybox:latest, docker.io/library/busybox:mytag
[root@master docker]#
```

## 总结

介绍了 save/load 命令的使用，可以将镜像导出成文件，并根据文件再次还原为镜像。
