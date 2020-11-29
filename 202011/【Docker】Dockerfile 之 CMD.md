# 【Docker】Dockerfile 之 CMD

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## CMD

The `CMD` instruction has three forms:

`CMD` 指令具有三种形式：

- `CMD ["executable","param1","param2"]` (_exec_ form, this is the preferred form)
- `CMD ["param1","param2"]` (as _default parameters to ENTRYPOINT_)
- `CMD command param1 param2` (_shell_ form)

- `CMD ["executable","param1","param2"]`（_exec_ 形式，这是首选形式）
- `CMD [“ param1”，“ param2”]`（作为 ENTRYPOINT 的默认参数）
- `CMD command param1 param2`（_shell_ 形式）

There can only be one `CMD` instruction in a `Dockerfile`. If you list more than one `CMD` then only the last `CMD` will take effect.

在 `Dockerfile` 中只能有一个 CMD 指令。如果您列出多个 `CMD`，那么只有最后一个 `CMD` 才会生效。

**The main purpose of a `CMD` is to provide defaults for an executing container.** These defaults can include an executable, or they can omit the executable, in which case you must specify an `ENTRYPOINT` instruction as well.

**`CMD` 的主要目的是为执行中的容器提供默认值**。这些默认值可以包含可执行文件，也可以省略可执行文件，在这种情况下，您还必须指定一条 `ENTRYPOINT` 指令。

If `CMD` is used to provide default arguments for the `ENTRYPOINT` instruction, both the `CMD` and `ENTRYPOINT` instructions should be specified with the JSON array format.

如果使用 `CMD` 为 `ENTRYPOINT` 指令提供默认参数，则 `CMD` 和 `ENTRYPOINT` 指令均应使用 JSON 数组格式指定。

> **Note**
> 
> The _exec_ form is parsed as a JSON array, which means that you must use double-quotes (") around words not single-quotes (').

> **注意**
>
> _exec_ 形式被解析为 JSON 数组，这意味着您必须使用双引号（"）而非单引号（'）包围单词。

Unlike the _shell_ form, the _exec_ form does not invoke a command shell. This means that normal shell processing does not happen. For example, `CMD [ "echo", "$HOME" ]` will not do variable substitution on `$HOME`. If you want shell processing then either use the _shell_ form or execute a shell directly, for example: `CMD [ "sh", "-c", "echo $HOME" ]`. When using the exec form and executing a shell directly, as in the case for the shell form, it is the shell that is doing the environment variable expansion, not docker.

与 _shell_ 格式不同，_exec_ 格式不调用命令外壳程序。这意味着不会进行常规的外壳处理。例如，`CMD [ "echo", "$HOME" ]` 不会在 `$HOME` 上进行变量替换。如果要进行 shell 处理，则可以使用 _shell_ 形式或直接执行 shell，例如： `CMD [ "sh", "-c", "echo $HOME" ]`。当使用 exec 格式并直接执行 shell 时，是由 shell 进行环境变量扩展，而不是 docker。

When used in the shell or exec formats, the `CMD` instruction sets the command to be executed when running the image.

当以 shell 或 exec 格式使用时， `CMD` 指令设置运行镜像时要执行的命令。

If you use the _shell_ form of the `CMD`, then the `<command>` will execute in `/bin/sh -c`:

如果使用 `CMD` 的 _shell_ 形式，则 `<command>` 将在 `/bin/sh -c` 中执行：

```Dockerfile
FROM ubuntu
CMD echo "This is a test." | wc -
```

If you want to **run your** `<command>` **without a shell** then you must express the command as a JSON array and give the full path to the executable. **This array form is the preferred format of `CMD`.** Any additional parameters must be individually expressed as strings in the array:

如果要在没有外壳的情况下运行您的 `<command>`，则必须将该命令表示为 JSON 数组，并提供可执行文件的完整路径。 **数组形式是 `CMD` 的首选格式。** 任何其他参数必须在数组中分别表示为字符串：

```Dockerfile
FROM ubuntu
CMD ["/usr/bin/wc","--help"]
```

If you would like your container to run the same executable every time, then you should consider using `ENTRYPOINT` in combination with `CMD`. See [_ENTRYPOINT_](https://docs.docker.com/engine/reference/builder/#entrypoint).

如果您希望容器每次都运行相同的可执行文件，则应考虑将 `ENTRYPOINT` 与 `CMD` 结合使用。请参阅[_ENTRYPOINT_](https://docs.docker.com/engine/reference/builder/#entrypoint)。

If the user specifies arguments to `docker run` then they will override the default specified in `CMD`.

如果用户为 `docker run` 指定了参数，则它们将覆盖 `CMD` 中指定的默认值。

> **Note**
> 
> Do not confuse `RUN` with `CMD`. `RUN` actually runs a command and commits the result; `CMD` does not execute anything at build time, but specifies the intended command for the image.

> **注意**
>
>不要将 `RUN` 和 `CMD` 混淆。`RUN`实际上运行命令并提交结果。`CMD` 在生成时不执行任何操作，但是指定了镜像的预期命令。

## 示例

### Dockerfile

```Dockerfile
FROM busybox
CMD echo jiangbo
```

### 结果

```sh
[root@master env]# docker build -t jiangbo:0.0.1 .
Sending build context to Docker daemon  3.584kB
Step 1/2 : FROM busybox
 ---> dc3bacd8b5ea
Step 2/2 : CMD echo jiangbo
 ---> Running in 8eb219d5669f
Removing intermediate container 8eb219d5669f
 ---> c1f4d3207e37
Successfully built c1f4d3207e37
Successfully tagged jiangbo:0.0.1
[root@master env]# docker run -it jiangbo:0.0.1
jiangbo
```

## 总结

介绍了 Dockerfile 中 RUN 指令的使用。