# 【Docker】命令 diff

参考教程：https://docs.docker.com/engine/reference/commandline/diff/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker diff CONTAINER`

检查容器中哪些文件发生了变更。

## 命令选项

| 标记 | 描述 |
| --- | --- |
| `A` | 增加文件或者文件夹 |
| `D` | 删除的文件或者文件夹 |
| `C` | 修改的文件或者文件夹 |

## 示例

### 查看容器文件变换

```sh
[root@master ~]# docker diff 1d
C /usr
C /usr/share
C /usr/share/nginx
C /usr/share/nginx/html
C /usr/share/nginx/html/index.html
C /run
A /run/nginx.pid
C /etc
C /etc/nginx
C /etc/nginx/conf.d
C /etc/nginx/conf.d/default.conf
C /root
A /root/.bash_history
C /var
C /var/cache
C /var/cache/nginx
A /var/cache/nginx/uwsgi_temp
A /var/cache/nginx/client_temp
A /var/cache/nginx/fastcgi_temp
A /var/cache/nginx/proxy_temp
A /var/cache/nginx/scgi_temp
```

## 总结

介绍了 diff 命令的使用，可以显示容器内文件的变换。
