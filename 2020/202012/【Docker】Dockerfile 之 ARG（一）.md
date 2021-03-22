# 【Docker】Dockerfile 之 ARG（一）

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## ARG

```Dockerfile
ARG <name>[=<default value>]
```

The `ARG` instruction defines a variable that users can pass at build-time to the builder with the `docker build` command using the `--build-arg <varname>=<value>` flag. If a user specifies a build argument that was not defined in the Dockerfile, the build outputs a warning.

`ARG` 指令定义了一个变量，用户可以在构建时使用 `docker build` 命令使用`--build-arg <varname>=<value>` 标志将其传递给构建器。如果用户指定了未在 Dockerfile 中定义的构建参数，则构建会输出警告。

```sh
[Warning] One or more build-args [foo] were not consumed.
```

A Dockerfile may include one or more `ARG` instructions. For example, the following is a valid Dockerfile:

Dockerfile 可能包含一个或多个 `ARG` 指令。例如，以下是有效的 Dockerfile：

```Dockerfile
FROM busybox
ARG user1
ARG buildno
# ...
```

> **Warning:**
> 
> It is not recommended to use build-time variables for passing secrets like github keys, user credentials etc. Build-time variable values are visible to any user of the image with the `docker history` command.

> **警告：**
>
> 不建议使用构建时变量来传递诸如 github 密钥，用户凭据等机密。构建时变量值对于使用 `docker history` 命令的镜像的任何用户都是可见的。

### 默认值

An `ARG` instruction can optionally include a default value:

`ARG` 指令可以选择包含默认值：

```Dockerfile
FROM busybox
ARG user1=someuser
ARG buildno=1
# ...
```

If an `ARG` instruction has a default value and if there is no value passed at build-time, the builder uses the default.

如果 `ARG` 指令具有默认值，并且在构建时未传递任何值，则构建器将使用默认值。

### 范围

An `ARG` variable definition comes into effect from the line on which it is defined in the `Dockerfile` not from the argument’s use on the command-line or elsewhere. For example, consider this Dockerfile:

`ARG` 变量从 Dockerfile 中定义的行开始生效，而不是从命令行或其他地方的自变量使用开始。例如，考虑以下 Dockerfile：

```Dockerfile
FROM busybox
USER ${user:-some_user}
ARG user
USER $user
# ...
```

A user builds this file by calling:

```sh
$ docker build --build-arg user=what_user .
```

The `USER` at line 2 evaluates to `some_user` as the `user` variable is defined on the subsequent line 3. The `USER` at line 4 evaluates to `what_user` as `user` is defined and the `what_user` value was passed on the command line. Prior to its definition by an `ARG` instruction, any use of a variable results in an empty string.

第 2 行的 `USER` 评估为 `some_user`，因为在随后的第 3 行中定义了 `USER` 变量。第 4 行的 `USER` 评估为 `what_user`，因为定义了 `user`，并且为 `what_user` 在命令行中传递。在通过 `ARG` 指令对其进行定义之前，对变量的任何使用都会导致一个空字符串。

An `ARG` instruction goes out of scope at the end of the build stage where it was defined. To use an arg in multiple stages, each stage must include the `ARG` instruction.

`ARG` 指令在定义它的构建阶段结束时超出范围。要在多个阶段使用变量，每个阶段都必须包含 `ARG` 指令。

```Dockerfile
FROM busybox
ARG SETTINGS
RUN ./run/setup $SETTINGS

FROM busybox
ARG SETTINGS
RUN ./run/other $SETTINGS
```

## 总结

介绍了 Dockerfile 中 ARG 指令的说明，默认值和范围。