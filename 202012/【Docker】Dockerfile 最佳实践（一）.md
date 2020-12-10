# 【Docker】Dockerfile 最佳实践（一）

参考教程：https://docs.docker.com/develop/develop-images/dockerfile_best-practices/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 概述

This document covers recommended best practices and methods for building efficient images.

本文档介绍了用于构建有效镜像的推荐最佳实践和方法。

Docker builds images automatically by reading the instructions from a `Dockerfile` -- a text file that contains all commands, in order, needed to build a given image. A `Dockerfile` adheres to a specific format and set of instructions which you can find at [Dockerfile reference](https://docs.docker.com/engine/reference/builder/).

Docker 通过读取 Dockerfile 中的指令自动构建镜像，Dockerfile 是一个文本文件，其中依次包含构建给定镜像所需的所有命令。Dockerfile 遵循特定的格式和指令集，您可以在 [Dockerfile参考](https://docs.docker.com/engine/reference/builder/) 中找到。

A Docker image consists of read-only layers each of which represents a Dockerfile instruction. The layers are stacked and each one is a delta of the changes from the previous layer. Consider this `Dockerfile`:

Docker 镜像由只读层组成，每个只读层代表一个 Dockerfile 指令。这些层是堆叠的，每个层都是上一层的变化的增量。考虑一下这个 `Dockerfile`：

```Dockerfile
FROM ubuntu:18.04
COPY . /app
RUN make /app
CMD python /app/app.py
```

Each instruction creates one layer:

- `FROM` creates a layer from the `ubuntu:18.04` Docker image.
- `COPY` adds files from your Docker client’s current directory.
- `RUN` builds your application with `make`.
- `CMD` specifies what command to run within the container.

每条指令创建一层：

- `FROM` 从 Docker 镜像 `ubuntu:18.04` 创建一个层。
- `COPY` 从 Docker 客户端的当前目录添加文件。
- `RUN` 使用 `make` 构建您的应用程序。
- `CMD` 指定在容器中运行什么命令。

When you run an image and generate a container, you add a new _writable layer_ (the “container layer”) on top of the underlying layers. All changes made to the running container, such as writing new files, modifying existing files, and deleting files, are written to this thin writable container layer.

运行镜像并生成容器时，可以在基础层之上添加一个新的 _writable layer_（也叫容器层）。对运行中的容器所做的所有更改（例如写入新文件，修改现有文件和删除文件）都将写入此薄可写容器层。

For more on image layers (and how Docker builds and stores images), see [About storage drivers](https://docs.docker.com/storage/storagedriver/).

有关镜像层（以及Docker 如何构建和存储镜像）的更多信息，请参阅[关于存储驱动程序](https://docs.docker.com/storage/storagedriver/)。


## 一般准则和建议

### 创建临时容器

The image defined by your `Dockerfile` should generate containers that are as ephemeral as possible. By “ephemeral”, we mean that the container can be stopped and destroyed, then rebuilt and replaced with an absolute minimum set up and configuration.

您的 Dockerfile 定义的镜像应生成尽可能短暂的容器。“短暂”是指可以停止并销毁容器，然后对其进行重建和替换，并采用绝对的最低限度的设置和配置。

Refer to [Processes](https://12factor.net/processes) under _The Twelve-factor App_ methodology to get a feel for the motivations of running containers in such a stateless fashion.

请参阅“十二因子应用程序”方法下的 [Processes](https://12factor.net/processes)，以了解以这种无状态方式运行容器的动机。

### 了解构建上下文

When you issue a `docker build` command, the current working directory is called the _build context_. By default, the Dockerfile is assumed to be located here, but you can specify a different location with the file flag (`-f`). Regardless of where the `Dockerfile` actually lives, all recursive contents of files and directories in the current directory are sent to the Docker daemon as the build context.

发出 `docker build` 命令时，当前工作目录称为 _build context_。默认情况下，假定 Dockerfile 位于此处，但是您可以使用文件标志（`-f`）指定其他位置。无论 Dockerfile 实际位于何处，当前目录中文件和目录的所有递归内容都将作为构建上下文发送到 Docker 守护程序。

Build context example
构建示例

> Create a directory for the build context and `cd` into it. Write “hello” into a text file named `hello` and create a Dockerfile that runs `cat` on it. Build the image from within the build context (`.`):
> 
> 为构建上下文创建目录，并在其中 `cd` 进入。将“hello”写入名为 `hello` 的文本文件，并创建一个在其上运行 `cat` 的 Dockerfile。从构建上下文（`.`）中构建镜像：
>
> ```sh
> mkdir myproject && cd myproject
> echo "hello" > hello
> echo -e "FROM busybox\nCOPY /hello /\nRUN cat /hello" > Dockerfile
> docker build -t helloapp:v1 .
> ```
> 
> Move `Dockerfile` and `hello` into separate directories and build a second version of the image (without relying on cache from the last build). Use `-f` to point to the Dockerfile and specify the directory of the build context:
> 将 Dockerfile 和 hello 移到单独的目录中，并构建镜像的第二个版本（不依赖于上次构建的缓存）。使用 `-f` 指向 Dockerfile 并指定构建上下文的目录：
> 
> ```sh
> mkdir -p dockerfiles context
> mv Dockerfile dockerfiles && mv hello context
> docker build --no-cache -t helloapp:v2 -f dockerfiles/Dockerfile context
> ```

Inadvertently including files that are not necessary for building an image results in a larger build context and larger image size. This can increase the time to build the image, time to pull and push it, and the container runtime size. To see how big your build context is, look for a message like this when building your `Dockerfile`:

无意间包含了构建镜像所不需要的文件会导致较大的构建上下文和较大的镜像大小。这会增加生成镜像的时间，拉取和推送镜像的时间以及容器运行时的大小。要查看您的构建上下文有多大，请在构建 `Dockerfile` 时查找如下消息：

```sh
Sending build context to Docker daemon  187.8MB
```

## 总结

介绍了 Dockerfile 最佳实践的概述和一般性准则。