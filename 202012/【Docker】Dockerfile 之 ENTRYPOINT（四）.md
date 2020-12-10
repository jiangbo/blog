# 【Docker】Dockerfile 之 ENTRYPOINT（四）

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## ENTRYPOINT 和 CMD 的交互

Both `CMD` and `ENTRYPOINT` instructions define what command gets executed when running a container. There are few rules that describe their co-operation.
`CMD` 和 `ENTRYPOINT` 指令均定义了运行容器时执行的命令。有少量的规则描述他们的合作。

1. Dockerfile should specify at least one of `CMD` or `ENTRYPOINT` commands.
1. Dockerfile 应该至少指定 `CMD` 或 `ENTRYPOINT` 命令之一。
    
2. `ENTRYPOINT` should be defined when using the container as an executable.
2. 使用容器作为可执行文件时，应定义 `ENTRYPOINT`。
    
3. `CMD` should be used as a way of defining default arguments for an `ENTRYPOINT` command or for executing an ad-hoc command in a container.
3. 应该使用 `CMD` 作为定义 `ENTRYPOINT` 命令或在容器中执行命令的默认参数的方式。
    
4. `CMD` will be overridden when running the container with alternative arguments.
4. 当使用其他参数运行容器时，`CMD` 将被覆盖。

The table below shows what command is executed for different `ENTRYPOINT` / `CMD` combinations:
下表显示了针对不同的 `ENTRYPOINT`/`CMD` 组合执行的命令：

|   | No ENTRYPOINT | ENTRYPOINT exec\_entry p1\_entry | ENTRYPOINT \[“exec\_entry”, “p1\_entry”\] |
| --- | --- | --- | --- |
| **No CMD** | _error, not allowed_ | /bin/sh -c exec\_entry p1\_entry | exec\_entry p1\_entry |
| **CMD \[“exec\_cmd”, “p1\_cmd”\]** | exec\_cmd p1\_cmd | /bin/sh -c exec\_entry p1\_entry | exec\_entry p1\_entry exec\_cmd p1\_cmd |
| **CMD \[“p1\_cmd”, “p2\_cmd”\]** | p1\_cmd p2\_cmd | /bin/sh -c exec\_entry p1\_entry | exec\_entry p1\_entry p1\_cmd p2\_cmd |
| **CMD exec\_cmd p1\_cmd** | /bin/sh -c exec\_cmd p1\_cmd | /bin/sh -c exec\_entry p1\_entry | exec\_entry p1\_entry /bin/sh -c exec\_cmd p1\_cmd |

> **Note**
> 
> If `CMD` is defined from the base image, setting `ENTRYPOINT` will reset `CMD` to an empty value. In this scenario, `CMD` must be defined in the current image to have a value.

> **注意**
>
>如果从基础镜像定义了 `CMD`，则设置 `ENTRYPOINT` 会将 `CMD` 重置为空值。在这种情况下，必须在当前镜像中定义`CMD` 以具有值。

## 总结

介绍了 Dockerfile 中 ENTRYPOINT 指令和 CMD 指令的交互。