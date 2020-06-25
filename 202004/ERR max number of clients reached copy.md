# Redis：ERR max number of clients reached

## 环境

1. redis 2.7

## 原因

在项目启动的过程中，由于 redis 连接数量达到了最大，报错提示：ERR max number of clients reached。

## 分析

根据错误提示，可知是 redis 的连接客户端达到了最大数量。

### 客户端连接数

首先，登录 redis 服务器，使用 redis-cli 连接上 redis，下面的  xxx.xxx.xxx.xxx 表示服务器的 IP，如果没有默认为 127.0.0.1。

``` shell
./redis-cli -h xxx.xxx.xxx.xxx

# 如果不需要密码，可以不用认证
AUTH password
```

连接上 redis 后，使用 ` INFO clients` 查看客户端连接数量：

```text
# Clients
connected_clients:300
client_longest_output_list:0
client_biggest_input_buf:0
blocked_clients:0
```

然后使用命令 `CONFIG GET maxclients` 查看配置的最大客户端连接数量：

```text
1) "maxclients"
2) "3984"
```

如果连接的数量达到了最大，可以将 redis 中的最大客户端数量加大。以下是配置文件中的说明：

```text
# Set the max number of connected clients at the same time. By default
# this limit is set to 10000 clients, however if the Redis server is not
# able to configure the process file limit to allow for the specified limit
# the max number of allowed clients is set to the current file limit
# minus 32 (as Redis reserves a few file descriptors for internal uses).
#
# Once the limit is reached Redis will close all the new connections sending
# an error 'max number of clients reached'.
#
 maxclients 10000
```

## 连接数量

### 总连接数

有时候，需要排查到底是哪些 IP 占用的连接数过多，可以通过命令 `netstat -an|grep 26379|wc -l` 来统计连接的数量。  

### 根据 IP 统计

通过命令

```shell
netstat -n |grep 26379 |awk '/^tcp/ {print $5}'| awk -F: '{print $1}'|sort | uniq -c | sort -rn
```

来统计每个 IP 的具体连接数量，就可以找出来占用连接数最多的 IP（grep 过滤根据实际情况修改）。

## linux 系统限制

### ulimit -a

```text
[app_rdu@vm-kvm11286-app ~]$ ulimit -a
core file size          (blocks, -c) unlimited
data seg size           (kbytes, -d) unlimited
scheduling priority             (-e) 0
file size               (blocks, -f) unlimited
pending signals                 (-i) 62776
max locked memory       (kbytes, -l) 64
max memory size         (kbytes, -m) unlimited
open files                      (-n) 1024
pipe size            (512 bytes, -p) 8
POSIX message queues     (bytes, -q) 819200
real-time priority              (-r) 0
stack size              (kbytes, -s) 10240
cpu time               (seconds, -t) unlimited
max user processes              (-u) 4096
virtual memory          (kbytes, -v) unlimited
file locks                      (-x) unlimited
```

### open file 限制

客户端的连接数量，除了配置文件中配置的数量限制外，也会受到 linux 系统打开文件数量的限制。
在 redis 启动时，有时候会看到如下的提示：

```text
You requested maxclients of 10000 requiring at least 10032 max file descriptors.
Redis can’t set maximum open files to 10032 because of OS error: Operation not permitted.
Current maximum open files is 4096. Maxclients has been reduced to 4064 to compensate for low ulimit. If you need higher maxclients increase ‘ulimit –n’.
```

上面的提示说明最大连接数被 linux 系统限制了。可以通过命令 `ulimit –Sn 10000` 来修改。
