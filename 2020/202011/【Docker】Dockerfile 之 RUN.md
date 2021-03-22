# 【Docker】Dockerfile 之 RUN

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## RUN

RUN has 2 forms:

RUN 有 2 种格式：

- `RUN <command>` (_shell_ form, the command is run in a shell, which by default is `/bin/sh -c` on Linux or `cmd /S /C` on Windows)
- `RUN ["executable", "param1", "param2"]` (_exec_ form)

- `RUN <command>`（_shell_ 形式，该命令在 shell 中运行，在 Linux 上默认为`/ bin/sh -c`，在 Windows 上默认为 `cmd / S / C`）
- `RUN ["executable", "param1", "param2"]`（_exec_ 形式）

The `RUN` instruction will execute any commands in a new layer on top of the current image and commit the results. The resulting committed image will be used for the next step in the `Dockerfile`.

RUN 指令将在当前镜像顶部的新层中执行所有命令，并提交结果。生成的提交镜像将用于 Dockerfile 中的下一步。

Layering `RUN` instructions and generating commits conforms to the core concepts of Docker where commits are cheap and containers can be created from any point in an image’s history, much like source control.

分层运行 RUN 指令并生成提交符合 Docker 的核心概念，在 Docker 上，提交很廉价，并且可以从镜像历史记录的任何位置创建容器，就像源代码控制一样。

The _exec_ form makes it possible to avoid shell string munging, and to `RUN` commands using a base image that does not contain the specified shell executable.

_exec_ 形式可以避免破坏 shell 字符串，并可以使用不包含指定 shell 可执行文件的基础镜像执行 `RUN` 命令。

The default shell for the _shell_ form can be changed using the `SHELL` command.

可以使用 `SHELL` 命令更改 _sh​​ell_ 格式的默认 shell。

In the _shell_ form you can use a `\` (backslash) to continue a single RUN instruction onto the next line. For example, consider these two lines:

在 _shell_ 格式中，您可以使用 `\`（反斜杠）将一条 RUN 指令继续到下一行。例如，考虑以下两行：


```Dockerfile
RUN /bin/bash -c 'source $HOME/.bashrc; \
echo $HOME'
```

Together they are equivalent to this single line:

它们在一起等效于以下这一行：

```Dockerfile
RUN /bin/bash -c 'source $HOME/.bashrc; echo $HOME'
```

To use a different shell, other than ‘/bin/sh’, use the _exec_ form passing in the desired shell. For example:

要使用 `/bin/sh` 以外的其他 shell，请使用 _exec_ 形式传入所需的 shell。例如：

```Dockerfile
RUN ["/bin/bash", "-c", "echo hello"]
```

> **Note**
> 
> The _exec_ form is parsed as a JSON array, which means that you must use double-quotes (“) around words not single-quotes (‘).

> **注意**
>
> _exec_ 形式被解析为 JSON 数组，这意味着您必须使用双引号（"）而不是单引号（'）来包围单词。

Unlike the _shell_ form, the _exec_ form does not invoke a command shell. This means that normal shell processing does not happen. For example, `RUN [ "echo", "$HOME" ]` will not do variable substitution on `$HOME`. If you want shell processing then either use the _shell_ form or execute a shell directly, for example: `RUN [ "sh", "-c", "echo $HOME" ]`. When using the exec form and executing a shell directly, as in the case for the shell form, it is the shell that is doing the environment variable expansion, not docker.

与 _shell_ 格式不同，_exec_ 格式不调用命令外壳程序。这意味着不会进行常规的外壳处理。例如，`RUN [ "echo", "$HOME" ]` 不会在 `$HOME` 上进行变量替换。如果要进行 shell 处理，则可以使用 _shell_ 形式或直接执行 shell，例如： `RUN [ "sh", "-c", "echo $HOME" ]`。当使用 exec 格式并直接执行 shell 时，是由 shell 进行环境变量扩展，而不是 docker。

> **Note**
> 
> In the _JSON_ form, it is necessary to escape backslashes. This is particularly relevant on Windows where the backslash is the path separator. The following line would otherwise be treated as _shell_ form due to not being valid JSON, and fail in an unexpected way:
> 
> ```Dockerfile
> RUN ["c:\windows\system32\tasklist.exe"]
> ```
> 
> The correct syntax for this example is:
> 
> ```Dockerfile
> RUN ["c:\\windows\\system32\\tasklist.exe"]
> ```

> **注意**
>
>在 _JSON_ 格式中，必须转义反斜杠。这在 Windows 中特别有用，在 Windows 中反斜杠是路径分隔符。由于无效的 JSON，以下行否则将被视为 _shell_ 形式，并以意外的方式失败：
>
>```Dockerfile
> RUN ["c:\windows\system32\tasklist.exe"]
>```
>
>此示例的正确语法为：
>
>```Dockerfile
> RUN ["c:\\windows\\system32\\tasklist.exe"]
>```

The cache for `RUN` instructions isn’t invalidated automatically during the next build. The cache for an instruction like `RUN apt-get dist-upgrade -y` will be reused during the next build. The cache for `RUN` instructions can be invalidated by using the `--no-cache` flag, for example `docker build --no-cache`.

在下次构建期间，`RUN` 指令的缓存不会自动失效。诸如 `RUN apt-get dist-upgrade -y` 之类的指令的缓存将在下一次构建中重用。可以通过使用 `--no-cache` 标志来使 RUN 指令的缓存无效，例如 `docker build --no-cache`。

See the [`Dockerfile` Best Practices guide](https://docs.docker.com/engine/userguide/eng-image/dockerfile_best-practices/) for more information.

有关更多信息，请参阅[ `Dockerfile` 最佳实践指南](https://docs.docker.com/engine/userguide/eng-image/dockerfile_best-practices/)。

The cache for `RUN` instructions can be invalidated by [`ADD`](https://docs.docker.com/engine/reference/builder/#add) and [`COPY`](https://docs.docker.com/engine/reference/builder/#copy) instructions.

`RUN` 指令的缓存可以通过 [`ADD`](https://docs.docker.com/engine/reference/builder/#add) 和 [`COPY`](https://docs.docker.com/engine/reference/builder/#copy) 失效。

## 示例

### Dockerfile 文件

```Dockerfile
FROM busybox
ENV name=jiangbo
RUN echo $name
RUN echo "---------------"
RUN ["/bin/sh","-c","echo $name"]
```

### 构建结果

```sh
[root@master env]# docker build . --no-cache
Sending build context to Docker daemon  3.584kB
Step 1/5 : FROM busybox
 ---> dc3bacd8b5ea
Step 2/5 : ENV name=jiangbo
 ---> Running in eee44fef26f6
Removing intermediate container eee44fef26f6
 ---> 947038c832dc
Step 3/5 : RUN echo $name
 ---> Running in 26e9d42ced2a
jiangbo
Removing intermediate container 26e9d42ced2a
 ---> a55b1d062caa
Step 4/5 : RUN echo "---------------"
 ---> Running in 2b3455b3b565
---------------
Removing intermediate container 2b3455b3b565
 ---> 314567362777
Step 5/5 : RUN ["/bin/sh","-c","echo $name"]
 ---> Running in 7ebe6c03686d
jiangbo
Removing intermediate container 7ebe6c03686d
 ---> 2c6b18b8be36
Successfully built 2c6b18b8be36
[root@master env]#
```

## 总结

介绍了 Dockerfile 中 RUN 指令的使用。