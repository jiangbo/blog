# 【Docker】命令 logs

参考教程：https://docs.docker.com/engine/reference/commandline/logs/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker logs [OPTIONS] CONTAINER`

使用 logs 命令可以查看一个容器的日志信息。

## 命令选项

| 名称 | 默认值 | 描述 |
| --- | --- | --- |
| `--details` |  | 显示额外的日志信息 |
| `--follow , -f` |  | 跟踪日志的输出 |
| `--since` |  | 显示从某个时间点开始的日志信息，例如：2013-01-02T13:23:37 或者相对时间：42m/42 minutes |
| `--tail` | `all` | 从末尾计算需要显示的日志行数 |
| `--timestamps , -t` |  | 显示时间戳 |
| `--until` |  | 和 since 类似，不过显示这个时间点之前的信息 |

## 示例

### 显示日志

```sh
[root@master ~]# docker run --name test -d busybox sh -c "while true; do $(echo date); sleep 1; done"
9641d41e63ad6e6363125c56bcf6f536c0ddd3572abc6e36913c0321889839cc
[root@master ~]# docker logs --details 964
Mon Sep 14 03:31:14 UTC 2020
Mon Sep 14 03:31:15 UTC 2020
Mon Sep 14 03:31:16 UTC 2020
Mon Sep 14 03:31:17 UTC 2020
Mon Sep 14 03:31:18 UTC 2020
Mon Sep 14 03:31:19 UTC 2020
Mon Sep 14 03:31:20 UTC 2020
```

### 跟踪日志显示

```sh
[root@master ~]# docker logs -f 964
Mon Sep 14 03:31:14 UTC 2020
Mon Sep 14 03:31:15 UTC 2020
Mon Sep 14 03:31:16 UTC 2020
Mon Sep 14 03:31:17 UTC 2020
Mon Sep 14 03:31:18 UTC 2020
```

### 显示最后十条日志

```sh
[root@master ~]# docker logs --tail 10  964
Mon Sep 14 03:36:00 UTC 2020
Mon Sep 14 03:35:59 UTC 2020
Mon Sep 14 03:35:58 UTC 2020
Mon Sep 14 03:35:57 UTC 2020
Mon Sep 14 03:35:56 UTC 2020
Mon Sep 14 03:35:55 UTC 2020
Mon Sep 14 03:35:54 UTC 2020
Mon Sep 14 03:35:53 UTC 2020
Mon Sep 14 03:35:52 UTC 2020
Mon Sep 14 03:35:51 UTC 2020
[root@master ~]#
```

### 显示时间

```sh
[root@master ~]# docker logs --tail 10 -t 964
2020-09-14T11:37:10.150281744+08:00 Mon Sep 14 03:37:10 UTC 2020
2020-09-14T11:37:09.113900574+08:00 Mon Sep 14 03:37:09 UTC 2020
2020-09-14T11:37:08.095807701+08:00 Mon Sep 14 03:37:08 UTC 2020
2020-09-14T11:37:07.076534240+08:00 Mon Sep 14 03:37:07 UTC 2020
2020-09-14T11:37:06.056548302+08:00 Mon Sep 14 03:37:06 UTC 2020
2020-09-14T11:37:05.035730336+08:00 Mon Sep 14 03:37:05 UTC 2020
2020-09-14T11:37:04.020247099+08:00 Mon Sep 14 03:37:04 UTC 2020
2020-09-14T11:37:03.001625882+08:00 Mon Sep 14 03:37:02 UTC 2020
2020-09-14T11:37:01.980953509+08:00 Mon Sep 14 03:37:01 UTC 2020
2020-09-14T11:37:00.962954752+08:00 Mon Sep 14 03:37:00 UTC 2020
[root@master ~]#
```

### 根据时间过滤

```sh
[root@master ~]# docker logs -t --since 4s 964
2020-09-14T11:38:23.693648677+08:00 Mon Sep 14 03:38:23 UTC 2020
2020-09-14T11:38:24.712847929+08:00 Mon Sep 14 03:38:24 UTC 2020
2020-09-14T11:38:25.732625152+08:00 Mon Sep 14 03:38:25 UTC 2020
2020-09-14T11:38:26.760329397+08:00 Mon Sep 14 03:38:26 UTC 2020
[root@master ~]#
```

### 获取时间间隔内日志

```sh
$ docker logs -t --since 10s --until 4s 318
2020-09-14T03:40:57.571416787Z Mon Sep 14 03:40:57 UTC 2020
2020-09-14T03:40:58.572266428Z Mon Sep 14 03:40:58 UTC 2020
2020-09-14T03:40:59.573579316Z Mon Sep 14 03:40:59 UTC 2020
2020-09-14T03:41:00.575278553Z Mon Sep 14 03:41:00 UTC 2020
2020-09-14T03:41:01.575871760Z Mon Sep 14 03:41:01 UTC 2020
2020-09-14T03:41:02.576968052Z Mon Sep 14 03:41:02 UTC 2020
$
```

## 总结

介绍了 logs 命令的使用，可以查看容器的日志。
