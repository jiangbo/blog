# 【Docker】命令 ps

参考教程：https://docs.docker.com/engine/reference/commandline/ps/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker ps [OPTIONS]`

使用 ps 命令可以查看一个容器列表的信息。

## 命令选项

| 名称 | 默认值 | 描述 |
| --- | --- | --- |
| `--all , -a` |  | 显示所有的容器，默认只显示运行中的容器 |
| `--filter , -f` |  | 根据过滤条件输出信息 |
| `--format` |  | 格式化输出 |
| `--last , -n` | `-1` | 显示最新创建的 n 个容器 |
| `--latest , -l` |  | 显示最新创建的容器 |
| `--no-trunc` |  | 不截断输出|
| `--quiet , -q` |  | 只显示容器的 ID |
| `--size , -s` |  | 显示文件的大小 |

## 示例

### 显示运行中的容器

```sh
[root@master docker]# docker ps
CONTAINER ID  IMAGE                           COMMAND               CREATED      STATUS          PORTS               NAMES
2ccd9d92501d  docker.io/library/nginx:latest  nginx -g daemon o...  4 hours ago  Up 4 hours ago  0.0.0.0:80->80/tcp  my_nginx
[root@master docker]#
```

### 显示所有容器

```sh
[root@master docker]# docker ps -a
CONTAINER ID  IMAGE                           COMMAND               CREATED      STATUS                  PORTS               NAMES
2ccd9d92501d  docker.io/library/nginx:latest  nginx -g daemon o...  4 hours ago  Up 4 hours ago          0.0.0.0:80->80/tcp  my_nginx
9aad98fa024f  docker.io/library/nginx:latest  nginx -g daemon o...  4 hours ago  Exited (0) 4 hours ago                      condescending_mendeleev
[root@master docker]#
```

### 显示容器文件大小

```sh
[root@master docker]# docker ps -a -s
CONTAINER ID  IMAGE                           COMMAND               CREATED      STATUS                  PORTS               NAMES                    SIZE
2ccd9d92501d  docker.io/library/nginx:latest  nginx -g daemon o...  4 hours ago  Up 4 hours ago          0.0.0.0:80->80/tcp  my_nginx                 1.12kB (virtual 135MB)
9aad98fa024f  docker.io/library/nginx:latest  nginx -g daemon o...  4 hours ago  Exited (0) 4 hours ago                      condescending_mendeleev  1.11kB (virtual 135MB)
[root@master docker]#
```

### 过滤容器输出

```sh
[root@master docker]# docker ps -a --filter 'exited=0'
CONTAINER ID  IMAGE                           COMMAND               CREATED      STATUS                  PORTS  NAMES
9aad98fa024f  docker.io/library/nginx:latest  nginx -g daemon o...  4 hours ago  Exited (0) 4 hours ago         condescending_mendeleev
[root@master docker]#
```

### 格式化输出

```sh
[root@master docker]# docker ps -a --format "{{.ID}}: {{.Command}}"
2ccd9d92501d: nginx -g daemon o...
9aad98fa024f: nginx -g daemon o...
[root@master docker]#
```

### table 格式化输出

```sh
[root@master docker]# docker ps -a --format "table {{.ID}}\t{{.Labels}}"
ID\tLabels
2ccd9d92501d\tmap[maintainer:NGINX Docker Maintainers &lt;docker-maint@nginx.com&gt;]
9aad98fa024f\tmap[maintainer:NGINX Docker Maintainers &lt;docker-maint@nginx.com&gt;]
[root@master docker]#
```

## 总结

介绍了 ps 命令的使用，可以查看容器列表信息。
