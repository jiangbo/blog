# 【Docker】命令 search

参考教程：https://docs.docker.com/engine/reference/commandline/search/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker search [OPTIONS] TERM`

使用 search 命令可以在 https://hub.docker.com/ 上搜索可用的镜像。

## 镜像名称结构

```text
Registry/<Your Docker ID>/<Repository Name>:<tag>
```

### Registry

表示使用的 Registry，通常意义上的远程仓库，可以使用私有的 Registry，如果不写，默认为 docker hub 仓库。因为仓库地址在国外，国内有一些大厂提供镜像加速器，可以加速镜像的下载速度。
除了公共的 Registry，也可以搭建私有的 Registry，一般不对外公开，内部进行镜像的共享。私有的 Registry 可以使用 Docker Registry 搭建，也可以使用 Harbor 和 Sonatype Nexus 等。

### Docker ID 

用户在 docker hub 上的用户名，这个相当于命名空间的概念。如果不写，默认是 docker hub 的 library 命名空间下。

### Repository

仓库名称，相当于一个项目的概念，不能省略。

### tag

类似于版本的概念，如果不写，默认为 lastest，即最新。

## 命令选项

| 名称 | 默认值 | 描述 |
| --- | --- | --- |
| `--automated` |  | 已过时，只显示自动构建的镜像，使用 filter 替代 |
| `--filter , -f` |  | 根据提供的条件过滤输出 |
| `--format` |  | 格式化输出 |
| `--limit` | `25` | 搜索结果的最大显示数量 |
| `--no-trunc` |  | 不截断输出 |
| `--stars , -s` |  | 已过时，根据 star 数进行过滤，使用 filter 替代 |

## 示例

### 根据名称搜索

```sh
$ docker search busybox
NAME                      DESCRIPTION                                     STARS      OFFICIAL    AUTOMATED
busybox                   Busybox base image.                             1974       [OK]
progrium/busybox                                                          71                     [OK]
radial/busyboxplus        Full-chain, Internet enabled, busybox made f…   32                     [OK]
yauritux/busybox-curl     Busybox with CURL                               10
arm32v7/busybox           Busybox base image.                             8
armhf/busybox             Busybox base image.                             6
odise/busybox-curl                                                        4                      [OK]
arm64v8/busybox           Busybox base image.                             3
s390x/busybox             Busybox base image.                             2
prom/busybox              Prometheus Busybox Docker base images           2                      [OK]
aarch64/busybox           Busybox base image.                             2
arm32v6/busybox           Busybox base image.                             2
p7ppc64/busybox           Busybox base image for ppc64.                   2
joeshaw/busybox-nonroot   Busybox container with non-root user nobody     2
i386/busybox              Busybox base image.                             2
vukomir/busybox           busybox and curl                                1
spotify/busybox           Spotify fork of https://hub.docker.com/_/bus…   1
ppc64le/busybox           Busybox base image.                             1
sou856099/busybox                                                         0
amd64/busybox             Busybox base image.                             0
concourse/busyboxplus                                                     0
arm32v5/busybox           Busybox base image.                             0
emccorp/busybox           Busybox                                         0
ggtools/busybox-ubuntu    Busybox ubuntu version with extra goodies       0                      [OK]
e2eteam/busybox                                                           0
```

### 限制搜索数量

```sh
$ docker search busybox --limit  4
NAME                    DESCRIPTION                                     STARS    OFFICIAL    AUTOMATED
busybox                 Busybox base image.                             1974     [OK]
radial/busyboxplus      Full-chain, Internet enabled, busybox made f…   32                   [OK]
yauritux/busybox-curl   Busybox with CURL                               10
vukomir/busybox         busybox and curl                                1
```

### 不截断输出

描述信息栏不会被截断，会显示完整的描述信息。

```sh
$ docker search busybox --limit  4 --no-trunc
NAME                    DESCRIPTION                        STARS               OFFICIAL            AUTOMATED
busybox                 Busybox base image.                1974                [OK]
radial/busyboxplus      Full-chain, Internet enabled, 
busybox made from scratch. Comes in git and cURL flavors.   32                                      [OK]
yauritux/busybox-curl   Busybox with CURL                   10
vukomir/busybox         busybox and curl                    1
```

### 根据 stars 数量过滤

```sh
$ docker search busybox  -s 1000
Flag --stars has been deprecated, use --filter=stars=3 instead
NAME                DESCRIPTION           STARS               OFFICIAL            AUTOMATED
busybox             Busybox base image.   1974                [OK]
```

```sh
$ docker search busybox  --stars 1000
Flag --stars has been deprecated, use --filter=stars=3 instead
NAME                DESCRIPTION           STARS               OFFICIAL            AUTOMATED
busybox             Busybox base image.   1974                [OK]
```

```sh
$ docker search busybox  --filter stars=1000
NAME                DESCRIPTION           STARS               OFFICIAL            AUTOMATED
busybox             Busybox base image.   1974                [OK]
```

### 根据自动化过滤

```sh
$ docker search busybox  --automated
Flag --automated has been deprecated, use --filter=is-automated=true instead
NAME                     DESCRIPTION                                     STARS               OFFICIAL            AUTOMATED
progrium/busybox                                                         71                                      [OK]
radial/busyboxplus       Full-chain, Internet enabled, busybox made f…   32                                      [OK]
odise/busybox-curl                                                       4                                       [OK]
prom/busybox             Prometheus Busybox Docker base images           2                                       [OK]
ggtools/busybox-ubuntu   Busybox ubuntu version with extra goodies       0                                       [OK]
```

```sh
$ docker search busybox  --filter is-automated=true
NAME                     DESCRIPTION                                     STARS               OFFICIAL            AUTOMATED
progrium/busybox                                                         71                                      [OK]
radial/busyboxplus       Full-chain, Internet enabled, busybox made f…   32                                      [OK]
odise/busybox-curl                                                       4                                       [OK]
prom/busybox             Prometheus Busybox Docker base images           2                                       [OK]
ggtools/busybox-ubuntu   Busybox ubuntu version with extra goodies       0                                       [OK]
```

### 根据官方镜像过滤

```sh
$ docker search busybox  --filter is-official=true
NAME                DESCRIPTION           STARS               OFFICIAL            AUTOMATED
busybox             Busybox base image.   1974                [OK]
```

### 格式化输出

格式化选项（--format）使用 Go 模板漂亮地打印搜索输出，Go 模板的有效占位符为：
| Placeholder | Description |
| --- | --- |
| `.Name` | 镜像名称 |
| `.Description` | 镜像描述 |
| `.StarCount` | stars 数量 |
| `.IsOfficial` | 如果是官方镜像，显示 “OK” |
| `.IsAutomated` | 如果是自动化构建，显示“OK” |

```sh
$ docker search --format "{{.Name}}: {{.StarCount}}" nginx --limit  3
nginx: 13652
jwilder/nginx-proxy: 1864
bitnami/nginx: 88
```

```sh
$ docker search --format "table {{.Name}}\t{{.IsAutomated}}\t{{.IsOfficial}}" nginx --limit  4
NAME                      AUTOMATED           OFFICIAL
nginx                                         [OK]
jwilder/nginx-proxy       [OK]
richarvey/nginx-php-fpm   [OK]
bitnami/nginx             [OK]
```

## 总结

介绍了 search 命令的使用，除了根据名称搜索，还可以根据一些限制条件进行过滤。介绍了 `--format` 选项的使用方法，对输出的内容进行格式化。

