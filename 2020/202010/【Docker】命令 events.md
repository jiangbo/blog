# 【Docker】命令 events

参考教程：https://docs.docker.com/engine/reference/commandline/events/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker events [OPTIONS]`

实时获取 docker 上发生的事件。

## 命令选项

| 名称 | 默认值 | 描述 |
| --- | --- | --- |
| `--filter , -f` |  | 设置过滤条件 |
| `--format` |  | 设置格式化 |
| `--since` |  | 开始时间 |
| `--until` |  | 结束时间 |

## 示例

### 监听事件

```sh
[root@master ~]# docker events
2020-09-04T20:45:07.028334249+08:00 container exec_die 1d06decda2604b0f6eb52624b498f59df404ab2b7087e61c1616c42248c8bad5 (execID=74b3ff62e729c9758176d9f7d738a3dcc7f328d72150b1258fb926124c89d8ad, exitCode=0, image=nginx, maintainer=NGINX Docker Maintainers <docker-maint@nginx.com>, name=strange_mendel)
2020-09-04T20:45:38.511582967+08:00 container kill 1d06decda2604b0f6eb52624b498f59df404ab2b7087e61c1616c42248c8bad5 (image=nginx, maintainer=NGINX Docker Maintainers <docker-maint@nginx.com>, name=strange_mendel, signal=15)
2020-09-04T20:45:38.607036454+08:00 container die 1d06decda2604b0f6eb52624b498f59df404ab2b7087e61c1616c42248c8bad5 (exitCode=0, image=nginx, maintainer=NGINX Docker Maintainers <docker-maint@nginx.com>, name=strange_mendel)
2020-09-04T20:45:38.693629934+08:00 network disconnect 275c403d734ed47d71a8d24396507010c6337e00861c9f041b080c4661a14e1e (container=1d06decda2604b0f6eb52624b498f59df404ab2b7087e61c1616c42248c8bad5, name=bridge, type=bridge)
2020-09-04T20:45:38.732609313+08:00 container stop 1d06decda2604b0f6eb52624b498f59df404ab2b7087e61c1616c42248c8bad5 (image=nginx, maintainer=NGINX Docker Maintainers <docker-maint@nginx.com>, name=strange_mendel)

```

## 总结

介绍了 events 命令的使用，可以监听 docker 上的事件。
