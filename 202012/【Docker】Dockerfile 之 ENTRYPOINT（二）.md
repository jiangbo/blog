# 【Docker】Dockerfile 之 ENTRYPOINT（二）

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## ENTRYPOINT 的 exec 格式示例

You can use the _exec_ form of `ENTRYPOINT` to set fairly stable default commands and arguments and then use either form of `CMD` to set additional defaults that are more likely to be changed.
您可以使用 `ENTRYPOINT` 的 _exec_ 形式来设置相当稳定的默认命令和参数，然后使用 `CMD` 的任一种形式来设置更可能被更改的其他默认值。

```Dockerfile
FROM ubuntu
ENTRYPOINT ["top", "-b"]
CMD ["-c"]
```

When you run the container, you can see that `top` is the only process:
运行容器时，可以看到 `top` 是唯一的进程：

```sh
$ docker run -it --rm --name test  top -H
top - 03:06:16 up 2 days, 12:28,  0 users,  load average: 3.28, 4.85, 4.80
Threads:   1 total,   1 running,   0 sleeping,   0 stopped,   0 zombie
%Cpu(s):  0.0 us, 33.3 sy,  0.0 ni, 66.7 id,  0.0 wa,  0.0 hi,  0.0 si,  0.0 st
MiB Mem :  32174.9 total,  10607.0 free,   9970.9 used,  11597.0 buff/cache
MiB Swap:      0.0 total,      0.0 free,      0.0 used.  21364.0 avail Mem 

  PID USER      PR  NI    VIRT    RES    SHR S  %CPU  %MEM     TIME+ COMMAND
    1 root      20   0    5968   3336   2908 R   0.0   0.0   0:00.03 top
```

To examine the result further, you can use `docker exec`:
要进一步检查结果，可以使用 `docker exec`：

```sh
$ docker exec -it test ps aux
USER       PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND
root         1  0.1  0.0   5968  3336 ?        Ss   03:08   0:00 top -b -H
root         8  0.0  0.0   5896  2928 pts/0    Rs+  03:08   0:00 ps aux
```

And you can gracefully request `top` to shut down using `docker stop test`.
您可以使用 `docker stop test` 优雅地请求 `top` 关闭。

The following `Dockerfile` shows using the `ENTRYPOINT` to run Apache in the foreground (i.e., as `PID 1`):
以下 `Dockerfile` 显示了使用 `ENTRYPOINT` 在前台运行 Apache（即，作为 PID 1）：

```Dockerfile
FROM debian:stable
RUN apt-get update && apt-get install -y --force-yes apache2
EXPOSE 80 443
VOLUME ["/var/www", "/var/log/apache2", "/etc/apache2"]
ENTRYPOINT ["/usr/sbin/apache2ctl", "-D", "FOREGROUND"]
```

If you need to write a starter script for a single executable, you can ensure that the final executable receives the Unix signals by using `exec` and `gosu` commands:
如果需要为单个可执行文件编写启动脚本，则可以使用 `exec` 和 `gosu` 命令确保最终的可执行文件接收 Unix 信号：

```sh
#!/usr/bin/env bash
set -e

if [ "$1" = 'postgres' ]; then
    chown -R postgres "$PGDATA"

    if [ -z "$(ls -A "$PGDATA")" ]; then
        gosu postgres initdb
    fi

    exec gosu postgres "$@"
fi

exec "$@"
```

Lastly, if you need to do some extra cleanup (or communicate with other containers) on shutdown, or are co-ordinating more than one executable, you may need to ensure that the `ENTRYPOINT` script receives the Unix signals, passes them on, and then does some more work:
最后，如果您需要在关机时进行一些额外的清理工作（或与其他容器通信），或者要协调多个可执行文件，则可能需要确保 `ENTRYPOINT` 脚本接收 Unix 信号，然后将其传递，然后做更多的工作：

```sh
#!/bin/sh
# Note: I've written this using sh so it works in the busybox container too

# USE the trap if you need to also do manual cleanup after the service is stopped,
#     or need to start multiple services in the one container
trap "echo TRAPed signal" HUP INT QUIT TERM

# start service in background here
/usr/sbin/apachectl start

echo "[hit enter key to exit] or run 'docker stop <container>'"
read

# stop service and clean up here
echo "stopping apache"
/usr/sbin/apachectl stop

echo "exited $0"
```

If you run this image with `docker run -it --rm -p 80:80 --name test apache`, you can then examine the container’s processes with `docker exec`, or `docker top`, and then ask the script to stop Apache:
如果使用 `docker run -it --rm -p 80:80 --name test apache` 运行该映像，则可以使用 `docker exec` 或 `docker top` 检查容器的进程，然后询问脚本停止 Apache：

```sh
$ docker exec -it test ps aux

USER       PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND
root         1  0.1  0.0   4448   692 ?        Ss+  00:42   0:00 /bin/sh /run.sh 123 cmd cmd2
root        19  0.0  0.2  71304  4440 ?        Ss   00:42   0:00 /usr/sbin/apache2 -k start
www-data    20  0.2  0.2 360468  6004 ?        Sl   00:42   0:00 /usr/sbin/apache2 -k start
www-data    21  0.2  0.2 360468  6000 ?        Sl   00:42   0:00 /usr/sbin/apache2 -k start
root        81  0.0  0.1  15572  2140 ?        R+   00:44   0:00 ps aux

$ docker top test

PID                 USER                COMMAND
10035               root                {run.sh} /bin/sh /run.sh 123 cmd cmd2
10054               root                /usr/sbin/apache2 -k start
10055               33                  /usr/sbin/apache2 -k start
10056               33                  /usr/sbin/apache2 -k start

$ /usr/bin/time docker stop test

test
real	0m 0.27s
user	0m 0.03s
sys	0m 0.03s
```

> **Note**
> 
> You can override the `ENTRYPOINT` setting using `--entrypoint`, but this can only set the binary to _exec_ (no `sh -c` will be used).

> **注意**
>
>您可以使用 `--entrypoint` 覆盖 `ENTRYPOINT` 设置，但这只能将二进制文件设置为 _exec_（将不使用 `sh -c`）。

> **Note**
> 
> The _exec_ form is parsed as a JSON array, which means that you must use double-quotes (") around words not single-quotes (').

> **注意**
>
> _exec_ 格式被解析为 JSON 数组，这意味着您必须在单词引用时使用双引号而不是单引号。

Unlike the _shell_ form, the _exec_ form does not invoke a command shell. This means that normal shell processing does not happen. For example, `ENTRYPOINT [ "echo", "$HOME" ]` will not do variable substitution on `$HOME`. If you want shell processing then either use the _shell_ form or execute a shell directly, for example: `ENTRYPOINT [ "sh", "-c", "echo $HOME" ]`. When using the exec form and executing a shell directly, as in the case for the shell form, it is the shell that is doing the environment variable expansion, not docker.


与 _shell_ 格式不同，_exec_ 格式不调用命令外壳程序。这意味着不会进行常规的外壳处理。例如，`ENTRYPOINT [ "echo", "$HOME" ]` 不会在 `$HOME` 上进行变量替换。如果要进行 shell 处理，则可以使用 _shell_ 形式或直接执行 shell，例如： `ENTRYPOINT [ "sh", "-c", "echo $HOME" ]`。当使用 exec 格式并直接执行 shell 时，是由 shell 进行环境变量扩展，而不是 docker。

## 总结

介绍了 Dockerfile 中 ENTRYPOINT 指令的 exec 格式的使用。