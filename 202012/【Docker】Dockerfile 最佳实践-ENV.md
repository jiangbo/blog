# 【Docker】Dockerfile 最佳实践-ENV

参考教程：https://docs.docker.com/develop/develop-images/dockerfile_best-practices/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## ENV

To make new software easier to run, you can use `ENV` to update the `PATH` environment variable for the software your container installs. For example, `ENV PATH=/usr/local/nginx/bin:$PATH` ensures that `CMD ["nginx"]` just works.

为了使新软件易于运行，您可以使用 `ENV` 来更新容器安装的软件的 `PATH` 环境变量。例如，`ENV PATH=/usr/local/nginx/bin:$PATH` 确保 `CMD ["nginx"]` 正常工作。

The `ENV` instruction is also useful for providing required environment variables specific to services you wish to containerize, such as Postgres’s `PGDATA`.

`ENV` 指令还可用于提供特定于您希望容器化的服务的必需环境变量，例如 Postgres 的 `PGDATA`。

Lastly, `ENV` can also be used to set commonly used version numbers so that version bumps are easier to maintain, as seen in the following example:

最后，`ENV` 也可以用来设置常用的版本号，以便更容易维护版本，如以下示例所示：

```Dockerfile
ENV PG_MAJOR=9.3
ENV PG_VERSION=9.3.4
RUN curl -SL https://example.com/postgres-$PG_VERSION.tar.xz | tar -xJC /usr/src/postgress && …
ENV PATH=/usr/local/postgres-$PG_MAJOR/bin:$PATH
```

Similar to having constant variables in a program (as opposed to hard-coding values), this approach lets you change a single `ENV` instruction to auto-magically bump the version of the software in your container.

类似于在程序中具有变量（与硬编码值相反），这种方法使您可以更改单个 `ENV` 指令以自动神奇地修改容器中软件的版本。

Each `ENV` line creates a new intermediate layer, just like `RUN` commands. This means that even if you unset the environment variable in a future layer, it still persists in this layer and its value can’t be dumped. You can test this by creating a Dockerfile like the following, and then building it.

每条 `ENV` 行都会创建一个新的中间层，就像 `RUN` 命令一样。这意味着，即使您在以后的层中取消设置环境变量，它也仍然保留在该层中，并且其值也无法转储。您可以通过创建如下所示的 Dockerfile，然后对其进行构建来进行测试。

```Dockerfile
FROM alpine
ENV ADMIN_USER="mark"
RUN echo $ADMIN_USER > ./mark
RUN unset ADMIN_USER
```

```sh
$ docker run --rm test sh -c 'echo $ADMIN_USER'

mark
```

To prevent this, and really unset the environment variable, use a `RUN` command with shell commands, to set, use, and unset the variable all in a single layer. You can separate your commands with `;` or `&&`. If you use the second method, and one of the commands fails, the `docker build` also fails. This is usually a good idea. Using `\` as a line continuation character for Linux Dockerfiles improves readability. You could also put all of the commands into a shell script and have the `RUN` command just run that shell script.

为了避免这种情况，并真正取消设置环境变量，请在外壳程序中使用带有外壳命令的 `RUN` 命令来设置，使用和取消设置该变量。您可以使用 `;` 或 `&&` 分隔命令。如果您使用第二种方法，并且其中一个命令失败，则 `docker build` 也会失败。这通常是个好主意。将 `\` 用作 Linux Dockerfiles 的行继续符可提高可读性。您也可以将所有命令放入一个 shell 脚本中，并让 `RUN` 命令运行该 shell 脚本。

```Dockerfile
FROM alpine
RUN export ADMIN_USER="mark" \
    && echo $ADMIN_USER > ./mark \
    && unset ADMIN_USER
CMD sh
```

```sh
$ docker run --rm test sh -c 'echo $ADMIN_USER'
```

## 总结

介绍了 Dockerfile 的 ENV 指令的最佳实践。