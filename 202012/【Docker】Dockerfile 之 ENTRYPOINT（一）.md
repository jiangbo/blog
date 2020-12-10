# 【Docker】Dockerfile 之 ENTRYPOINT（一）

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## ENTRYPOINT

ENTRYPOINT has two forms:
ENTRYPOINT 有两种形式：

The _exec_ form, which is the preferred form:
_exec_ 形式，这是首选形式：

```Dockerfile
ENTRYPOINT ["executable", "param1", "param2"]
```

The _shell_ form:
_shell_ 形式：

```Dockerfile
ENTRYPOINT command param1 param2
```

An `ENTRYPOINT` allows you to configure a container that will run as an executable.
`ENTRYPOINT` 允许您配置为容器配置一个可执行的命令。

For example, the following starts nginx with its default content, listening on port 80:
例如，以下代码以其默认内容启动 nginx，监听端口 80：

```sh
$ docker run -i -t --rm -p 80:80 nginx
```

Command line arguments to `docker run <image>` will be appended after all elements in an _exec_ form `ENTRYPOINT`, and will override all elements specified using `CMD`. This allows arguments to be passed to the entry point, i.e., `docker run <image> -d` will pass the `-d` argument to the entry point. You can override the `ENTRYPOINT` instruction using the `docker run --entrypoint` flag.
`docker run <image>` 的命令行参数将以 _exec_ 形式的所有元素添加到 `ENTRYPOINT` 后面，并将覆盖所有使用 `CMD` 指定的元素。这允许将参数传递给入口点，即 `docker run <image> -d` 将参数 `-d` 传递给入口点。您可以使用 `docker run --entrypoint` 标志覆盖 `ENTRYPOINT` 指令。

The _shell_ form prevents any `CMD` or `run` command line arguments from being used, but has the disadvantage that your `ENTRYPOINT` will be started as a subcommand of `/bin/sh -c`, which does not pass signals. This means that the executable will not be the container’s `PID 1` - and will _not_ receive Unix signals - so your executable will not receive a `SIGTERM` from `docker stop <container>`.
_shell_ 格式阻止使用任何 `CMD` 或 `run` 命令行参数，但具有以下缺点：`ENTRYPOINT`将作为 `/bin/sh -c` 的子命令启动，该子命令不会传递信号。这意味着该可执行文件将不是容器的 `PID 1`，并且不会接收 Unix 信号，因此您的可执行文件将不会从 `docker stop <container>` 接收到 `SIGTERM`。

Only the last `ENTRYPOINT` instruction in the `Dockerfile` will have an effect.
只有 `Dockerfile` 中的最后一条 `ENTRYPOINT` 指令才会生效。

## 总结

介绍了 Dockerfile 中 ENTRYPOINT 指令。