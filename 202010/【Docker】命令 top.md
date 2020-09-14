# 【Docker】命令 top

参考教程：https://docs.docker.com/engine/reference/commandline/top/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker top CONTAINER [ps OPTIONS]`

使用 top 命令可以显示容器中正在运行的进程。

## 示例

### 显示容器内进程

```sh
[root@master docker]# docker top my_nginx
USER    PID   PPID   %CPU    ELAPSED            TTY   TIME   COMMAND
root    1     0      0.000   4h1m8.900693184s   ?     0s     nginx: master process nginx -g daemon off;
nginx   28    1      0.000   4h1m7.901509452s   ?     0s     nginx: worker process
```

## 总结

介绍了 top 命令的使用，可以显示容器中正在运行的进程。
