# 【Docker】Dockerfile 之 WORKDIR

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## WORKDIR

```Dockerfile
WORKDIR /path/to/workdir
```

The `WORKDIR` instruction sets the working directory for any `RUN`, `CMD`, `ENTRYPOINT`, `COPY` and `ADD` instructions that follow it in the `Dockerfile`. If the `WORKDIR` doesn’t exist, it will be created even if it’s not used in any subsequent `Dockerfile` instruction.

`WORKDIR` 指令为 `Dockerfile` 中跟在其后的所有 `RUN`，`CMD`，`ENTRYPOINT`，`COPY` 和 `ADD` 指令设置工作目录。如果 `WORKDIR` 不存在，即使之后的 Dockerfile 指令中未使用它也将被创建。

The `WORKDIR` instruction can be used multiple times in a `Dockerfile`. If a relative path is provided, it will be relative to the path of the previous `WORKDIR` instruction. For example:

`WORKDIR` 指令可在 `Dockerfile` 中多次使用。如果提供了相对路径，则它将相对于上一个 `WORKDIR` 指令的路径。 例如：

```Dockerfile
WORKDIR /a
WORKDIR b
WORKDIR c
RUN pwd
```

The output of the final `pwd` command in this `Dockerfile` would be `/a/b/c`.
该 `Dockerfile` 中最后一个 `pwd` 命令的输出为 `/a/b/c`。

The `WORKDIR` instruction can resolve environment variables previously set using `ENV`. You can only use environment variables explicitly set in the `Dockerfile`. For example:

`WORKDIR` 指令可以解析以前使用 `ENV` 设置的环境变量。您只能使用在 `Dockerfile` 中显式设置的环境变量。 例如：

```Dockerfile
ENV DIRPATH=/path
WORKDIR $DIRPATH/$DIRNAME
RUN pwd
```

The output of the final `pwd` command in this `Dockerfile` would be `/path/$DIRNAME`.

该 `Dockerfile` 中最后一个 `pwd` 命令的输出为 `/path/$DIRNAME`。

## 总结

介绍了 Dockerfile 中 WORKDIR 指令的用法和注意事项。