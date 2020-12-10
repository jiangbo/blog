# 【Docker】Dockerfile 之 ENTRYPOINT（三）

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## ENTRYPOINT 的 shell 格式示例

You can specify a plain string for the `ENTRYPOINT` and it will execute in `/bin/sh -c`. This form will use shell processing to substitute shell environment variables, and will ignore any `CMD` or `docker run` command line arguments. To ensure that `docker stop` will signal any long running `ENTRYPOINT` executable correctly, you need to remember to start it with `exec`:
您可以为 `ENTRYPOINT` 指定一个纯字符串，它将在 `/bin/sh -c` 中执行。这种形式将使用外壳处理环境变量，并且将忽略任何 `CMD` 或 `docker run` 命令行参数。为了确保 `docker stop` 能够正确通知任何长期运行的 `ENTRYPOINT` 可执行文件，您需要记住要使用 `exec` 启动它：

```Dockerfile
FROM ubuntu
ENTRYPOINT exec top -b
```

When you run this image, you’ll see the single `PID 1` process:
当运行这个镜像时，您会看到一个 `PID 1` 进程：

```sh
$ docker run -it --rm --name test top

Mem: 1704520K used, 352148K free, 0K shrd, 0K buff, 140368121167873K cached
CPU:   5% usr   0% sys   0% nic  94% idle   0% io   0% irq   0% sirq
Load average: 0.08 0.03 0.05 2/98 6
  PID  PPID USER     STAT   VSZ %VSZ %CPU COMMAND
    1     0 root     R     3164   0%   0% top -b
```

Which exits cleanly on `docker stop`:
使用 `docker stop` 可以完全退出：

```sh
$ /usr/bin/time docker stop test

test
real	0m 0.20s
user	0m 0.02s
sys	0m 0.04s
```

If you forget to add `exec` to the beginning of your `ENTRYPOINT`:
如果您忘记在 `ENTRYPOINT` 的开头添加 `exec`：

```Dockerfile
FROM ubuntu
ENTRYPOINT top -b
CMD --ignored-param1
```

You can then run it (giving it a name for the next step):
然后可以运行它（为下一步命名）：

```sh
$ docker run -it --name test top --ignored-param2

Mem: 1704184K used, 352484K free, 0K shrd, 0K buff, 140621524238337K cached
CPU:   9% usr   2% sys   0% nic  88% idle   0% io   0% irq   0% sirq
Load average: 0.01 0.02 0.05 2/101 7
  PID  PPID USER     STAT   VSZ %VSZ %CPU COMMAND
    1     0 root     S     3168   0%   0% /bin/sh -c top -b cmd cmd2
    7     1 root     R     3164   0%   0% top -b
```

You can see from the output of `top` that the specified `ENTRYPOINT` is not `PID 1`.
从 `top` 的输出中可以看到，指定的 `ENTRYPOINT` 不是 `PID 1`。

If you then run `docker stop test`, the container will not exit cleanly - the `stop` command will be forced to send a `SIGKILL` after the timeout:
如果随后运行 `docker stop test`，则容器将不会干净退出。超时后，将强制执行 `stop` 命令发送 `SIGKILL`：

```sh
$ docker exec -it test ps aux

PID   USER     COMMAND
    1 root     /bin/sh -c top -b cmd cmd2
    7 root     top -b
    8 root     ps aux

$ /usr/bin/time docker stop test

test
real	0m 10.19s
user	0m 0.04s
sys	0m 0.03s
```
## 总结

介绍了 Dockerfile 中 ENTRYPOINT 指令的 shell 格式的使用。