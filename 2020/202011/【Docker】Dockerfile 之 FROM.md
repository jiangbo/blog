# 【Docker】Dockerfile 之 FROM

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## FROM

```Dockerfile
FROM [--platform=<platform>] <image> [AS <name>]
```

Or

```Dockerfile
FROM [--platform=<platform>] <image>[:<tag>] [AS <name>]
```

Or

```Dockerfile
FROM [--platform=<platform>] <image>[@<digest>] [AS <name>]
```

The `FROM` instruction initializes a new build stage and sets the [_Base Image_](https://docs.docker.com/glossary/#base_image) for subsequent instructions. As such, a valid `Dockerfile` must start with a `FROM` instruction. The image can be any valid image – it is especially easy to start by **pulling an image** from the [_Public Repositories_](https://docs.docker.com/engine/tutorials/dockerrepos/).

 `FROM` 指令初始化一个新的构建阶段，并为后续指令设置[基础镜像](https://docs.docker.com/glossary/#base_image)。因此，有效的 Dockerfile 必须以 FROM 指令开头。该镜像可以是任何有效的镜像–很容易通过从[_Public Repositories_](https://docs.docker.com/engine/tutorials/dockerrepos/) 拉取一个镜像。

- `ARG` is the only instruction that may precede `FROM` in the `Dockerfile`. See [Understand how ARG and FROM interact](https://docs.docker.com/engine/reference/builder/#understand-how-arg-and-from-interact).

- `ARG` 是唯一可以在 Dockerfile 中的 FROM 之前的指令。请参阅[了解ARG和FROM之间的交互方式](https://docs.docker.com/engine/reference/builder/#understand-how-arg-and-from-interact)。

- `FROM` can appear multiple times within a single `Dockerfile` to create multiple images or use one build stage as a dependency for another. Simply make a note of the last image ID output by the commit before each new `FROM` instruction. Each `FROM` instruction clears any state created by previous instructions.

- `FROM` 可以在单个 Dockerfile 中多次出现，以创建多个镜像或将一个构建阶段用作对另一构建阶段的依赖。只需在每条新的 `FROM` 指令之前记录一次提交输出的最后一个镜像 ID。每条 `FROM` 指令清除由先前指令创建的任何状态。

- Optionally a name can be given to a new build stage by adding `AS name` to the `FROM` instruction. The name can be used in subsequent `FROM` and `COPY --from=<name>` instructions to refer to the image built in this stage.

- 可以选择在 `FROM` 指令中添加 AS 名称，从而为新的构建阶段指定名称。该名称可以在后续的 `FROM` 和 `COPY --from = <name>` 指令中使用，以引用此阶段构建的镜像。

- The `tag` or `digest` values are optional. If you omit either of them, the builder assumes a `latest` tag by default. The builder returns an error if it cannot find the `tag` value.

- `tag` 或 `digest` 值是可选的。如果您忽略其中任何一个，则默认情况下，构建器将采用 `latest` 标签。如果构建器找不到 `tag` 值，则返回错误。

The optional `--platform` flag can be used to specify the platform of the image in case `FROM` references a multi-platform image. For example, `linux/amd64`, `linux/arm64`, or `windows/amd64`. By default, the target platform of the build request is used. Global build arguments can be used in the value of this flag, for example [automatic platform ARGs](https://docs.docker.com/engine/reference/builder/#automatic-platform-args-in-the-global-scope) allow you to force a stage to native build platform (`--platform=$BUILDPLATFORM`), and use it to cross-compile to the target platform inside the stage.

可选的 `--platform` 标志可用于指定镜像的平台，以防万一 ·FROM· 引用了多平台镜像。例如，`linux/amd64`，`linux/arm64` 或 `windows/amd64`。默认情况下，使用构建请求的目标平台。可以在此标志的值中使用全局构建参数，例如 [automatic platform ARGs](https://docs.docker.com/engine/reference/builder/#automatic-platform-args-in-the-global-scope) 允许您将阶段强制为本机构建平台（`--platform = $BUILDPLATFORM`），并使用该平台将其交叉编译到阶段内部的目标平台。

### Understand how ARG and FROM interac

`FROM` instructions support variables that are declared by any `ARG` instructions that occur before the first `FROM`.

`FROM` 指令支持由出现在第一个 `FROM` 之前的任何 `ARG` 指令声明的变量。

```Dockerfile
ARG  CODE_VERSION=latest
FROM base:${CODE_VERSION}
CMD  /code/run-app

FROM extras:${CODE_VERSION}
CMD  /code/run-extras
```

An `ARG` declared before a `FROM` is outside of a build stage, so it can’t be used in any instruction after a `FROM`. To use the default value of an `ARG` declared before the first `FROM` use an `ARG` instruction without a value inside of a build stage:

在 `FROM` 之前声明的 `ARG` 在构建阶段之外，因此不能在 `FROM` 之后的任何指令中使用。要使用在第一个 `FROM` 之前声明的 `ARG` 的默认值，请使用在构建阶段内部不带值的 `ARG` 指令：

```Dockerfile
ARG VERSION=latest
FROM busybox:$VERSION
ARG VERSION
RUN echo $VERSION > image_version
```

## 总结

介绍了 Dockerfile 中 FROM 指令的使用。