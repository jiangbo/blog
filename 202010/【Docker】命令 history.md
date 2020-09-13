# 【Docker】命令 history

参考教程：https://docs.docker.com/engine/reference/commandline/history/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker history [OPTIONS] IMAGE`

显示镜像的历史纪录。

## 命令选项

| 名称 | 默认值 | 描述 |
| --- | --- | --- |
| `--format` |  | 格式化输出 |
| `--human , -H` | `true` | 使用人类可读的形式显示日期和大小 |
| `--no-trunc` |  | 不截断输出 |
| `--quiet , -q` |  | 只显示 id |

## 示例

### 查看镜像历史

```sh
[root@master ~]# docker history nginx
IMAGE               CREATED             CREATED BY                                      SIZE                COMMENT
4bb46517cac3        3 weeks ago         /bin/sh -c #(nop)  CMD ["nginx" "-g" "daemon…   0B
<missing>           3 weeks ago         /bin/sh -c #(nop)  STOPSIGNAL SIGTERM           0B
<missing>           3 weeks ago         /bin/sh -c #(nop)  EXPOSE 80                    0B
<missing>           3 weeks ago         /bin/sh -c #(nop)  ENTRYPOINT ["/docker-entr…   0B
<missing>           3 weeks ago         /bin/sh -c #(nop) COPY file:0fd5fca330dcd6a7…   1.04kB
<missing>           3 weeks ago         /bin/sh -c #(nop) COPY file:1d0a4127e78a26c1…   1.96kB
<missing>           3 weeks ago         /bin/sh -c #(nop) COPY file:e7e183879c35719c…   1.2kB
<missing>           3 weeks ago         /bin/sh -c set -x     && addgroup --system -…   63.4MB
<missing>           3 weeks ago         /bin/sh -c #(nop)  ENV PKG_RELEASE=1~buster     0B
<missing>           3 weeks ago         /bin/sh -c #(nop)  ENV NJS_VERSION=0.4.3        0B
<missing>           3 weeks ago         /bin/sh -c #(nop)  ENV NGINX_VERSION=1.19.2     0B
<missing>           4 weeks ago         /bin/sh -c #(nop)  LABEL maintainer=NGINX Do…   0B
<missing>           4 weeks ago         /bin/sh -c #(nop)  CMD ["bash"]                 0B
<missing>           4 weeks ago         /bin/sh -c #(nop) ADD file:3af3091e7d2bb40bc…   69.2MB
```

## 总结

介绍了 history 命令的使用，可以查看镜像历史。
