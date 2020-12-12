# 【Docker】Dockerfile 最佳实践-RUN

参考教程：https://docs.docker.com/develop/develop-images/dockerfile_best-practices/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## RUN

Split long or complex `RUN` statements on multiple lines separated with backslashes to make your `Dockerfile` more readable, understandable, and maintainable.

在多行上用反斜杠分隔长或复杂的 `RUN` 语句，以使您的 `Dockerfile` 更具可读性，可理解性和可维护性。

### apt-get

Probably the most common use-case for `RUN` is an application of `apt-get`. Because it installs packages, the `RUN apt-get` command has several gotchas to look out for.

`RUN` 的最常见用例可能是 `apt-get` 的应用程序。因为它安装了软件包，所以 `RUN apt-get` 命令需要注意一些陷阱。

Avoid `RUN apt-get upgrade` and `dist-upgrade`, as many of the “essential” packages from the parent images cannot upgrade inside an [unprivileged container](https://docs.docker.com/engine/reference/run/#security-configuration). If a package contained in the parent image is out-of-date, contact its maintainers. If you know there is a particular package, `foo`, that needs to be updated, use `apt-get install -y foo` to update automatically.

避免使用 `RUN apt-get upgrade` 和 `dist-upgrade`，因为来自父映像的许多“基本”软件包都无法在[非特权容器](https://docs.docker.com/engine/reference/run/#security-configuration)中进行升级。如果父映像中包含的软件包已过期，请联系其维护者。如果您知道有一个特定的软件包 `foo` 需要更新，请使用 `apt-get install -y foo` 自动更新。

Always combine `RUN apt-get update` with `apt-get install` in the same `RUN` statement. For example:

始终在同一 `RUN` 语句中将 `RUN apt-get update` 和 `apt-get install` 结合使用。例如：

```Dockerfile
RUN apt-get update && apt-get install -y \
    package-bar \
    package-baz \
    package-foo \
    && rm -rf /var/lib/apt/lists/*
```

Using `apt-get update` alone in a `RUN` statement causes caching issues and subsequent `apt-get install` instructions fail. For example, say you have a Dockerfile:

在 `RUN` 语句中单独使用 `apt-get update` 会导致缓存问题，随后的 `apt-get install` 指令也会失败。例如，假设您有一个 Dockerfile：

```Dockerfile
FROM ubuntu:18.04
RUN apt-get update
RUN apt-get install -y curl
```

After building the image, all layers are in the Docker cache. Suppose you later modify `apt-get install` by adding extra package:

构建镜像后，所有层都在 Docker 缓存中。假设您以后通过添加额外的软件包来修改 `apt-get install`：

```Dockerfile
FROM ubuntu:18.04
RUN apt-get update
RUN apt-get install -y curl nginx
```

Docker sees the initial and modified instructions as identical and reuses the cache from previous steps. As a result the `apt-get update` is _not_ executed because the build uses the cached version. Because the `apt-get update` is not run, your build can potentially get an outdated version of the `curl` and `nginx` packages.

Docker 将初始指令和修改后的指令视为相同，并重复使用先前步骤中的缓存。结果，由于构建使用了缓存的版本，因此未执行 `apt-get update`。因为没有运行 `apt-get update`，所以您的构建可能会获得 `curl` 和 `nginx` 软件包的过时版本。

Using `RUN apt-get update && apt-get install -y` ensures your Dockerfile installs the latest package versions with no further coding or manual intervention. This technique is known as “cache busting”. You can also achieve cache-busting by specifying a package version. This is known as version pinning, for example:

使用 `RUN apt-get update && apt-get install -y` 确保您的 Dockerfile 安装最新的软件包版本，而无需进一步的编码或手动干预。这种技术称为“缓存清除”。您还可以通过指定软件包版本来实现缓存清除。这称为版本固定，例如：

```Dockerfile
RUN apt-get update && apt-get install -y \
    package-bar \
    package-baz \
    package-foo=1.3.*
```

Version pinning forces the build to retrieve a particular version regardless of what’s in the cache. This technique can also reduce failures due to unanticipated changes in required packages.

版本固定会强制构建物检索特定版本，而不管缓存中的内容是什么。该技术还可以减少由于所需包装中的意外更改而导致的故障。

Below is a well-formed `RUN` instruction that demonstrates all the `apt-get` recommendations.

下面是格式正确的 `RUN` 指令，演示了所有 `apt-get` 建议。

```Dockerfile
RUN apt-get update && apt-get install -y \
    aufs-tools \
    automake \
    build-essential \
    curl \
    dpkg-sig \
    libcap-dev \
    libsqlite3-dev \
    mercurial \
    reprepro \
    ruby1.9.1 \
    ruby1.9.1-dev \
    s3cmd=1.1.* \
 && rm -rf /var/lib/apt/lists/*
```

The `s3cmd` argument specifies a version `1.1.*`. If the image previously used an older version, specifying the new one causes a cache bust of `apt-get update` and ensures the installation of the new version. Listing packages on each line can also prevent mistakes in package duplication.

`s3cmd` 参数指定版本 `1.1。*`。如果镜像先前使用的是旧版本，则指定新版本会导致 `apt-get update` 缓存崩溃，并确保安装新版本。在每一行列出软件包还可以防止软件包重复中的错误。

In addition, when you clean up the apt cache by removing `/var/lib/apt/lists` it reduces the image size, since the apt cache is not stored in a layer. Since the `RUN` statement starts with `apt-get update`, the package cache is always refreshed prior to `apt-get install`.

另外，当您通过删除 `/var/lib/apt/lists` 清理 apt 缓存时，由于 apt 缓存没有存储在图层中，因此会减小映像大小。由于 RUN 语句以 `apt-get update` 开头，因此软件包缓存总是在 `apt-get install` 之前刷新。

> Official Debian and Ubuntu images [automatically run `apt-get clean`](https://github.com/moby/moby/blob/03e2923e42446dbb830c654d0eec323a0b4ef02a/contrib/mkimage/debootstrap#L82-L105), so explicit invocation is not required.

> 官方 Debian 和 Ubuntu 镜像[自动运行 `apt-get clean`](https://github.com/moby/moby/blob/03e2923e42446dbb830c654d0eec323a0b4ef02a/contrib/mkimage/debootstrap#L82-L105)，因此不需要显式调用。

### Using pipes

Some `RUN` commands depend on the ability to pipe the output of one command into another, using the pipe character (`|`), as in the following example:

一些 `RUN` 命令取决于使用管道字符（`|`）将一个命令的输出管道到另一个命令的能力，如以下示例所示：

```Dockerfile
RUN wget -O - https://some.site | wc -l > /number
```

Docker executes these commands using the `/bin/sh -c` interpreter, which only evaluates the exit code of the last operation in the pipe to determine success. In the example above this build step succeeds and produces a new image so long as the `wc -l` command succeeds, even if the `wget` command fails.

Docker 使用 `/bin/sh -c` 解释器执行这些 命令，该解释器仅评估管道中最后一个操作的退出代码来确定成功。在上面的示例中，即使 `wget` 命令失败，只要`wc -l` 命令成功，该构建步骤就会成功并生成一个新镜像。

If you want the command to fail due to an error at any stage in the pipe, prepend `set -o pipefail &&` to ensure that an unexpected error prevents the build from inadvertently succeeding. For example:

如果您希望命令由于管道中任何阶段的错误而失败，请添加 `set -o pipefail &&` 前缀，以确保意外错误可以防止构建意外进行。例如：

```Dockerfile
RUN set -o pipefail && wget -O - https://some.site | wc -l > /number
```

> Not all shells support the `-o pipefail` option.
> 
> 并非所有的 shell 都支持 `-o pipefail` 选项。
>
> In cases such as the `dash` shell on Debian-based images, consider using the _exec_ form of `RUN` to explicitly choose a shell that does support the `pipefail` option. For example:
> 
> 在诸如基于 Debian 的镜像上的 `dash` 外壳的情况下，请考虑使用 RUN 的 _exec_ 形式来明确选择一个支持 `pipefail` 选项的外壳。例如：
>
> ```Dockerfile
> RUN ["/bin/bash", "-c", "set -o pipefail && wget -O - https://some.site | wc -l > /number"]
> ```

## 总结

介绍了 Dockerfile 的 RUN 指令的最佳实践。