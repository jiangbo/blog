# 【Docker】Dockerfile 介绍

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

Docker can build images automatically by reading the instructions from a `Dockerfile`. A `Dockerfile` is a text document that contains all the commands a user could call on the command line to assemble an image. Using `docker build` users can create an automated build that executes several command-line instructions in succession.

Docker 可以通过读取 Dockerfile 中的指令来自动构建镜像。Dockerfile 是一个文本文档，其中包含用户可以在命令行上调用以组装镜像的所有命令。用户可以使用 `docker build` 来创建自动执行的构建，该构建可以连续执行几个命令行指令。

This page describes the commands you can use in a `Dockerfile`. When you are done reading this page, refer to the [`Dockerfile` Best Practices](https://docs.docker.com/engine/userguide/eng-image/dockerfile_best-practices/) for a tip-oriented guide.

本页描述了可以在 Dockerfile 中使用的命令。阅读完此页面后，请参考[`Dockerfile`最佳实践](https://docs.docker.com/engine/userguide/eng-image/dockerfile_best-practices/)以获得面向技巧的指南。

## 使用

The [docker build](https://docs.docker.com/engine/reference/commandline/build/) command builds an image from a `Dockerfile` and a _context_. The build’s context is the set of files at a specified location `PATH` or `URL`. The `PATH` is a directory on your local filesystem. The `URL` is a Git repository location.

[docker build](https://docs.docker.com/engine/reference/commandline/build/)命令从 Dockerfile 和context 构建镜像。构建的上下文是指定位置 `PATH` 或 `URL` 处的目录。`PATH` 是本地文件系统上的目录。`URL` 是一个 Git 存储库位置。

A context is processed recursively. So, a `PATH` includes any subdirectories and the `URL` includes the repository and its submodules. This example shows a build command that uses the current directory as context:

上下文是递归处理的。因此，`PATH` 包含所有子目录，`URL` 包含存储库及其子模块。此示例显示了一个使用当前目录作为上下文的构建命令：

```sh
$ docker build .

Sending build context to Docker daemon  6.51 MB
...
```

The build is run by the Docker daemon, not by the CLI. The first thing a build process does is send the entire context (recursively) to the daemon. In most cases, it’s best to start with an empty directory as context and keep your Dockerfile in that directory. Add only the files needed for building the Dockerfile.

构建是由 Docker 守护程序而不是 CLI 运行的。构建过程要做的第一件事是将整个上下文（递归）发送到守护程序。在大多数情况下，最好以空目录作为上下文，并将 Dockerfile 保留在该目录中。仅添加构建 Dockerfile 所需的文件。

> **Warning**
> 
> Do not use your root directory, `/`, as the `PATH` as it causes the build to transfer the entire contents of your hard drive to the Docker daemon.

> **警告**
>
> 请勿将您的根目录 `/` 用作 `PATH`，因为它会导致构建将硬盘驱动器的全部内容传输到 Docker 守护程序。

To use a file in the build context, the `Dockerfile` refers to the file specified in an instruction, for example, a `COPY` instruction. To increase the build’s performance, exclude files and directories by adding a `.dockerignore` file to the context directory. For information about how to [create a `.dockerignore` file](https://docs.docker.com/engine/reference/builder/#dockerignore-file) see the documentation on this page.

要在构建上下文中使用文件，`Dockerfile` 中指令（例如，`COPY`指令）中指定的文件。为了提高构建的性能，请在上下文目录中添加一个 .dockerignore 文件，以排除文件和目录。有关如何[创建`.dockerignore`文件](https://docs.docker.com/engine/reference/builder/#dockerignore-file)的信息，请参阅此页面上的文档。

Traditionally, the `Dockerfile` is called `Dockerfile` and located in the root of the context. You use the `-f` flag with `docker build` to point to a Dockerfile anywhere in your file system.

传统上，`Dockerfile` 称为 `Dockerfile`，位于上下文的根目录中。您可以将 `-f` 标志与 `docker build` 一起使用，以指向文件系统中任意位置的 Dockerfile。

```sh
$ docker build -f /path/to/a/Dockerfile .
```

You can specify a repository and tag at which to save the new image if the build succeeds:
如果构建成功，则可以指定存储新镜像的存储库和标签：

```sh
$ docker build -t shykes/myapp .
```

To tag the image into multiple repositories after the build, add multiple `-t` parameters when you run the `build` command:

要在构建后将镜像标记到多个存储库中，请在运行  `build`命令时添加多个 `-t` 参数：

```
$ docker build -t shykes/myapp:1.0.2 -t shykes/myapp:latest .
```

Before the Docker daemon runs the instructions in the `Dockerfile`, it performs a preliminary validation of the `Dockerfile` and returns an error if the syntax is incorrect:

在 Docker 守护程序运行 Dockerfile 中的指令之前，它会对 Dockerfile 执行初步验证，如果语法不正确，则返回错误：

```
$ docker build -t test/myapp .

Sending build context to Docker daemon 2.048 kB
Error response from daemon: Unknown instruction: RUNCMD
```

The Docker daemon runs the instructions in the `Dockerfile` one-by-one, committing the result of each instruction to a new image if necessary, before finally outputting the ID of your new image. The Docker daemon will automatically clean up the context you sent.

Docker 守护程序逐个运行 Dockerfile 中的指令，如有必要，将每个指令的结果提交到新映像，然后最终输出新映像的 ID。 Docker 守护程序将自动清理您发送的上下文。

Note that each instruction is run independently, and causes a new image to be created - so `RUN cd /tmp` will not have any effect on the next instructions.

注意，每条指令都是独立运行的，并会导致创建新的镜像，因此 `RUN cd /tmp` 对下一条指令不会有任何影响。

Whenever possible, Docker will re-use the intermediate images (cache), to accelerate the `docker build` process significantly. This is indicated by the `Using cache` message in the console output. (For more information, see the [`Dockerfile` best practices guide](https://docs.docker.com/engine/userguide/eng-image/dockerfile_best-practices/):

Docker 将尽可能重用中间镜像（缓存），以显着加速 `docker build` 进程。这由控制台输出中的 `Using cache` 消息指示。（有关更多信息，请参阅[`Dockerfile`最佳实践指南](https://docs.docker.com/engine/userguide/eng-image/dockerfile_best-practices/)：

```sh
$ docker build -t svendowideit/ambassador .

Sending build context to Docker daemon 15.36 kB
Step 1/4 : FROM alpine:3.2
 ---> 31f630c65071
Step 2/4 : MAINTAINER SvenDowideit@home.org.au
 ---> Using cache
 ---> 2a1c91448f5f
Step 3/4 : RUN apk update &&      apk add socat &&        rm -r /var/cache/
 ---> Using cache
 ---> 21ed6e7fbb73
Step 4/4 : CMD env | grep _TCP= | (sed 's/.*_PORT_\([0-9]*\)_TCP=tcp:\/\/\(.*\):\(.*\)/socat -t 100000000 TCP4-LISTEN:\1,fork,reuseaddr TCP4:\2:\3 \&/' && echo wait) | sh
 ---> Using cache
 ---> 7ea8aef582cc
Successfully built 7ea8aef582cc
```

Build cache is only used from images that have a local parent chain. This means that these images were created by previous builds or the whole chain of images was loaded with `docker load`. If you wish to use build cache of a specific image you can specify it with `--cache-from` option. Images specified with `--cache-from` do not need to have a parent chain and may be pulled from other registries.

构建缓存仅用于具有本地父链的镜像。这意味着这些镜像是由以前的版本创建的，或者整个图像链都已通过 `docker load` 加载。如果您希望使用特定镜像的构建缓存，则可以使用 `--cache-from` 选项指定它。用 `--cache-from`指定的镜像不需要父链，也可以从其他注册表中拉取。

When you’re done with your build, you’re ready to look into [_Pushing a repository to its registry_](https://docs.docker.com/engine/tutorials/dockerrepos/#/contributing-to-docker-hub).

完成构建后，就可以开始研究[将存储库推送到其注册表](https://docs.docker.com/engine/tutorials/dockerrepos/#/contributing-to-docker-hub)。

## 总结

介绍了 Dockerfile 的基础概念和怎么构建镜像。