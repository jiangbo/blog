# 【Docker】Dockerfile 最佳实践-ENTRYPOINT

参考教程：https://docs.docker.com/develop/develop-images/dockerfile_best-practices/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## ENTRYPOINT

The best use for ENTRYPOINT is to set the image’s main command, allowing that image to be run as though it was that command (and then use CMD as the default flags).

`ENTRYPOINT` 的最佳用途是设置镜像的主命令，使该镜像像该命令一样运行（然后使用 `CMD` 作为默认标志）。

Let’s start with an example of an image for the command line tool s3cmd:

让我们从命令行工具 `s3cmd` 的镜像示例开始：

```Dockerfile
ENTRYPOINT ["s3cmd"]
CMD ["--help"]
```

Now the image can be run like this to show the command’s help:

现在可以像这样运行镜像以显示命令的帮助：

```sh
$ docker run s3cmd
```

Or using the right parameters to execute a command:

或使用正确的参数执行命令：

```sh
$ docker run s3cmd ls s3://mybucket
```

This is useful because the image name can double as a reference to the binary as shown in the command above.

这很有用，因为镜像名称可以用作对二进制文件的引用，如上面的命令所示。

The `ENTRYPOINT` instruction can also be used in combination with a helper script, allowing it to function in a similar way to the command above, even when starting the tool may require more than one step.

`ENTRYPOINT` 指令也可以与辅助脚本结合使用，即使启动该工具可能需要一个以上的步骤，也可以使其与上述命令类似地工作。

For example, the [Postgres Official Image](https://hub.docker.com/_/postgres/) uses the following script as its `ENTRYPOINT`:

例如，[Postgres Official Image](https://hub.docker.com/_/postgres/) 使用以下脚本作为其 `ENTRYPOINT`：

```sh
#!/bin/bash
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

> Configure app as PID 1
> 将应用程序配置为PID 1
> 
> This script uses [the `exec` Bash command](https://wiki.bash-hackers.org/commands/builtin/exec) so that the final running application becomes the container’s PID 1. This allows the application to receive any Unix signals sent to the container. For more, see the [`ENTRYPOINT` reference](https://docs.docker.com/engine/reference/builder/#entrypoint).
> 该脚本使用 [exec Bash](https://wiki.bash-hackers.org/commands/builtin/exec) 命令，以便最终运行的应用程序成为容器的 PID1。这使该应用程序可以接收发送到该容器的所有 Unix 信号。有关更多信息，请参见 ENTRYPOINT 参考。

The helper script is copied into the container and run via `ENTRYPOINT` on container start:

将帮助程序脚本复制到容器中，并在容器启动时通过 `ENTRYPOINT` 运行：

```Dockerfile
COPY ./docker-entrypoint.sh /
ENTRYPOINT ["/docker-entrypoint.sh"]
CMD ["postgres"]
```

This script allows the user to interact with Postgres in several ways.

该脚本允许用户以多种方式与 Postgres 进行交互。

It can simply start Postgres:

它可以简单地启动 Postgres：

```sh
$ docker run postgres
```

Or, it can be used to run Postgres and pass parameters to the server:

或者，它可以用于运行 Postgres 并将参数传递给服务器：

```sh
$ docker run postgres postgres --help
```

Lastly, it could also be used to start a totally different tool, such as Bash:
最后，它也可以用于启动一个完全不同的工具，例如 Bash：

```
$ docker run --rm -it postgres bash
```

## 总结

介绍了 Dockerfile 的 ENTRYPOINT 指令的最佳实践。