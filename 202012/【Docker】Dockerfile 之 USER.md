# 【Docker】Dockerfile 之 USER

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## USER

```Dockerfile
USER <user>[:<group>]
```

or

```Dockerfile
USER <UID>[:<GID>]
```

The `USER` instruction sets the user name (or UID) and optionally the user group (or GID) to use when running the image and for any `RUN`, `CMD` and `ENTRYPOINT` instructions that follow it in the `Dockerfile`.

`USER` 指令设置运行镜像时要使用的用户名（或 UID）以及可选的用户组（或 GID），以及 `Dockerfile` 中跟随该镜像的所有 `RUN`，`CMD` 和 `ENTRYPOINT` 指令。

> Note that when specifying a group for the user, the user will have _only_ the specified group membership. Any other configured group memberships will be ignored.

>请注意，在为用户指定组时，用户将仅具有指定的组成员身份。任何其它已配置的组成员身份将被忽略。

> **Warning**
> 
> When the user doesn’t have a primary group then the image (or the next instructions) will be run with the `root` group.
> 
> On Windows, the user must be created first if it’s not a built-in account. This can be done with the `net user` command called as part of a Dockerfile.

> 警告
>
> 当用户没有主要组时，该镜像（或后续指令）将与 `root` 组一起运行。
>
> 在 Windows 上，如果不是内置帐户，则必须首先创建该用户。这可以通过作为 Dockerfile 的一部分调用的e `net user` 命令来完成。

```Dockerfile
FROM microsoft/windowsservercore
# Create Windows user in the container
RUN net user /add patrick
# Set it for subsequent commands
USER patrick
```

## 总结

介绍了 Dockerfile 中 USER 指令的用法和注意事项。