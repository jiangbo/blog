# 【Docker】Dockerfile 最佳实践（二）

参考教程：https://docs.docker.com/develop/develop-images/dockerfile_best-practices/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 一般准则和建议

### Exclude with .dockerignore

To exclude files not relevant to the build (without restructuring your source repository) use a `.dockerignore` file. This file supports exclusion patterns similar to `.gitignore` files. For information on creating one, see the [.dockerignore file](https://docs.docker.com/engine/reference/builder/#dockerignore-file).

要排除与构建无关的文件（在不重新组织源码库的情况下），请使用 `.dockerignore` 文件。 该文件支持类似于 `.gitignore` 文件的排除模式。有关创建文件的信息，请参见[.dockerignore文件](https://docs.docker.com/engine/reference/builder/#dockerignore-file)。

### Use multi-stage builds

[Multi-stage builds](https://docs.docker.com/develop/develop-images/multistage-build/) allow you to drastically reduce the size of your final image, without struggling to reduce the number of intermediate layers and files.

[多阶段构建](https://docs.docker.com/develop/develop-images/multistage-build/) 允许您大幅度减小最终镜像的大小，而无需努力减少中间层和文件。

Because an image is built during the final stage of the build process, you can minimize image layers by [leveraging build cache](https://docs.docker.com/develop/develop-images/dockerfile_best-practices/#leverage-build-cache).

由于镜像是在生成过程的最后阶段生成的，因此您可以通过[利用生成缓存](https://docs.docker.com/develop/develop-images/dockerfile_best-practices/#leverage-build-cache)减少镜像层。

For example, if your build contains several layers, you can order them from the less frequently changed (to ensure the build cache is reusable) to the more frequently changed:

例如，如果您的构建包含多个图层，则可以将它们从更改频率较低（以确保生成缓存可重用）到更改频率较高的顺序排序：

- Install tools you need to build your application
    
- Install or update library dependencies
    
- Generate your application


- 安装构建应用程序所需的工具
    
- 安装或更新库依赖
    
- 生成您的应用程序
    

A Dockerfile for a Go application could look like:
Go 应用程序的 Dockerfile 可能类似于：

```Dockerfile
FROM golang:1.11-alpine AS build

# Install tools required for project
# Run `docker build --no-cache .` to update dependencies
RUN apk add --no-cache git
RUN go get github.com/golang/dep/cmd/dep

# List project dependencies with Gopkg.toml and Gopkg.lock
# These layers are only re-built when Gopkg files are updated
COPY Gopkg.lock Gopkg.toml /go/src/project/
WORKDIR /go/src/project/
# Install library dependencies
RUN dep ensure -vendor-only

# Copy the entire project and build it
# This layer is rebuilt when a file changes in the project directory
COPY . /go/src/project/
RUN go build -o /bin/project

# This results in a single layer image
FROM scratch
COPY --from=build /bin/project /bin/project
ENTRYPOINT ["/bin/project"]
CMD ["--help"]
```

### Don’t install unnecessary packages

To reduce complexity, dependencies, file sizes, and build times, avoid installing extra or unnecessary packages just because they might be “nice to have.” For example, you don’t need to include a text editor in a database image.

为了降低复杂性，依赖性，文件大小和构建时间，请避免仅由于它们“很容易安装”而安装多余或不必要的软件包。例如，您不需要在数据库镜像中包含文本编辑器。

### Decouple applications

Each container should have only one concern. Decoupling applications into multiple containers makes it easier to scale horizontally and reuse containers. For instance, a web application stack might consist of three separate containers, each with its own unique image, to manage the web application, database, and an in-memory cache in a decoupled manner.

每个容器应该只有一个关注点。 将应用程序解耦到多个容器中，可以更轻松地水平缩放和重复使用容器。例如，一个 Web 应用程序栈可能由三个单独的容器组成，每个容器都有自己的唯一镜像，以便以分离的方式管理 Web 应用程序，数据库和内存中的缓存。

Limiting each container to one process is a good rule of thumb, but it is not a hard and fast rule. For example, not only can containers be [spawned with an init process](https://docs.docker.com/engine/reference/run/#specify-an-init-process), some programs might spawn additional processes of their own accord. For instance, [Celery](https://docs.celeryproject.org/) can spawn multiple worker processes, and [Apache](https://httpd.apache.org/) can create one process per request.

将每个容器限制为一个进程是一个很好的经验法则，但这并不是一成不变的规则。例如，不仅可以使用初始化进程来生成容器，而且某些程序还可以自行生成其他进程。例如，Celery 可以产生多个工作进程，而 Apache 可以为每个请求创建一个进程。

Use your best judgment to keep containers as clean and modular as possible. If containers depend on each other, you can use [Docker container networks](https://docs.docker.com/network/) to ensure that these containers can communicate.

根据您的最佳判断，使容器保持清洁和模块化。如果容器彼此依赖，则可以使用 Docker 容器网络来确保这些容器可以通信。

### Minimize the number of layers

In older versions of Docker, it was important that you minimized the number of layers in your images to ensure they were performant. The following features were added to reduce this limitation:

- Only the instructions `RUN`, `COPY`, `ADD` create layers. Other instructions create temporary intermediate images, and do not increase the size of the build.
    
- Where possible, use [multi-stage builds](https://docs.docker.com/develop/develop-images/multistage-build/), and only copy the artifacts you need into the final image. This allows you to include tools and debug information in your intermediate build stages without increasing the size of the final image.

在较旧的 Docker 版本中，重要的是最小化镜像中的层数以确保其性能。添加了以下功能来减少此限制：

- 只有指令 `RUN`，`COPY`，`ADD` 会创建图层。 其他说明创建临时的中间镜像，并且不会增加构建的大小。
    
- 尽可能使用[多阶段构建](https://docs.docker.com/develop/develop-images/multistage-build/)，仅将所需的工件复制到最终镜像中。这使您可以在中间构建阶段中包含工具和调试信息，而无需增加最终镜像的大小。

### Sort multi-line arguments

Whenever possible, ease later changes by sorting multi-line arguments alphanumerically. This helps to avoid duplication of packages and make the list much easier to update. This also makes PRs a lot easier to read and review. Adding a space before a backslash (`\`) helps as well.

尽可能通过字母数字排序多行参数来简化以后的更改。这有助于避免软件包重复，并使列表更易于更新。这也使 PR 易于阅读和查看。在反斜杠（`\`）之前添加空格也有帮助。

Here’s an example from the [`buildpack-deps` image](https://github.com/docker-library/buildpack-deps):

这是来自 [`buildpack-deps` 镜像](https://github.com/docker-library/buildpack-deps)的示例：

```Dockerfile
RUN apt-get update && apt-get install -y \
  bzr \
  cvs \
  git \
  mercurial \
  subversion \
  && rm -rf /var/lib/apt/lists/*
```

### Leverage build cache

When building an image, Docker steps through the instructions in your `Dockerfile`, executing each in the order specified. As each instruction is examined, Docker looks for an existing image in its cache that it can reuse, rather than creating a new (duplicate) image.

构建镜像时，Docker 会逐步执行 Dockerfile 中的指令，并以指定的顺序执行每个指令。在检查每条指令时，Docker 会在其缓存中查找可以重用的现有镜像，而不是创建新的（重复的）镜像。

If you do not want to use the cache at all, you can use the `--no-cache=true` option on the `docker build` command. However, if you do let Docker use its cache, it is important to understand when it can, and cannot, find a matching image. The basic rules that Docker follows are outlined below:

如果根本不想使用缓存，则可以在 `docker build` 命令中使用 `--no-cache=true` 选项。但是，如果您确实让 Docker 使用其缓存，那么了解何时可以找到匹配的镜像，这一点很重要。Docker 遵循的基本规则概述如下：

- Starting with a parent image that is already in the cache, the next instruction is compared against all child images derived from that base image to see if one of them was built using the exact same instruction. If not, the cache is invalidated.

- 从已在缓存中的父镜像开始，将下一条指令与从该基本映像派生的所有子镜像进行比较，以查看是否其中一个是使用完全相同的指令构建的。如果不是，则高速缓存无效。
    
- In most cases, simply comparing the instruction in the `Dockerfile` with one of the child images is sufficient. However, certain instructions require more examination and explanation.

- 在大多数情况下，只需将 Dockerfile 中的指令与子镜像之一进行比较就足够了。但是，某些指令需要更多的检查和解释。
    
- For the `ADD` and `COPY` instructions, the contents of the file(s) in the image are examined and a checksum is calculated for each file. The last-modified and last-accessed times of the file(s) are not considered in these checksums. During the cache lookup, the checksum is compared against the checksum in the existing images. If anything has changed in the file(s), such as the contents and metadata, then the cache is invalidated.

- 对于 `ADD` 和 `COPY` 指令，将检查镜像中文件的内容，并为每个文件计算一个校验和。在这些校验和中不考虑文件的最后修改时间和最后访问时间。在缓存查找期间，将校验和与现有镜像中的校验和进行比较。如果文件中的任何内容（例如内容和元数据）发生了更改，则缓存将无效。
    
- Aside from the `ADD` and `COPY` commands, cache checking does not look at the files in the container to determine a cache match. For example, when processing a `RUN apt-get -y update` command the files updated in the container are not examined to determine if a cache hit exists. In that case just the command string itself is used to find a match.

- 除了 `ADD` 和 `COPY` 命令外，缓存检查不会查看容器中的文件来确定缓存是否匹配。例如，在处理 `RUN apt-get -y update` 命令时，不会检查容器中更新的文件以确定是否存在缓存命中。在这种情况下，仅使用命令字符串本身来查找匹配项。

Once the cache is invalidated, all subsequent `Dockerfile` commands generate new images and the cache is not used.

一旦缓存无效，所有后续的 Dockerfile 命令都会生成新镜像，并且不使用缓存。

## 总结

介绍了 Dockerfile 最佳实践的一般性准则。