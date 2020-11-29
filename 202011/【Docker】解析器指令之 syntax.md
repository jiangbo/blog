# 【Docker】解析器指令之 syntax

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## syntax

```Dockerfile
# syntax=[remote image reference]
```

For example:
例如：

```Dockerfile
# syntax=docker/dockerfile
# syntax=docker/dockerfile:1.0
# syntax=docker.io/docker/dockerfile:1
# syntax=docker/dockerfile:1.0.0-experimental
# syntax=example.com/user/repo:tag@sha256:abcdef...
```

This feature is only enabled if the [BuildKit](https://docs.docker.com/engine/reference/builder/#buildkit) backend is used.

仅在使用 [BuildKit](https://docs.docker.com/engine/reference/builder/#buildkit) 后端时启用此功能。

> BuildKit 是实验性功能，这里不讲。

The syntax directive defines the location of the Dockerfile builder that is used for building the current Dockerfile. The BuildKit backend allows to seamlessly use external implementations of builders that are distributed as Docker images and execute inside a container sandbox environment.

语法指令定义用于构建当前 Dockerfile 的 Dockerfile 构建器的位置。BuildKit 后端允许无缝使用构建器的外部实现，这些构建器以 Docker 镜像的形式分发并在容器沙箱环境中执行。

Custom Dockerfile implementation allows you to:

自定义Dockerfile实现使您能够：

- Automatically get bugfixes without updating the daemon
- Make sure all users are using the same implementation to build your Dockerfile
- Use the latest features without updating the daemon
- Try out new experimental or third-party features

- 在不更新守护程序的情况下自动获取错误修正
- 确保所有用户都使用相同的实现来构建您的 Dockerfile
- 使用最新功能而不更新守护程序
- 试用新的实验性或第三方功能

### Official releases

Docker distributes official versions of the images that can be used for building Dockerfiles under `docker/dockerfile` repository on Docker Hub. There are two channels where new images are released: stable and experimental.

Docker 分发了可用于在 Docker Hub 上的 `docker/dockerfile` 存储库下构建Dockerfile 的镜像的正式版本。有两个发布新镜像的渠道：稳定版和实验版。

Stable channel follows semantic versioning. For example:

稳定的通道遵循语义版本控制。例如：

- `docker/dockerfile:1.0.0` - only allow immutable version `1.0.0`
- `docker/dockerfile:1.0` - allow versions `1.0.*`
- `docker/dockerfile:1` - allow versions `1.*.*`
- `docker/dockerfile:latest` - latest release on stable channel

- `docker/dockerfile:1.0.0` -仅允许不可变版本 `1.0.0`
- `docker/dockerfile:1.0` - 允许版本 `1.0.*`
- `docker/dockerfile:1` - 允许版本 `1.*.*`
- `docker/dockerfile:latest` - 稳定版本的最新版本

The experimental channel uses incremental versioning with the major and minor component from the stable channel on the time of the release. For example:

在发布时，实验频道使用稳定版本中主要和次要组件的增量版本控制。例如：

- `docker/dockerfile:1.0.1-experimental` - only allow immutable version `1.0.1-experimental`
- `docker/dockerfile:1.0-experimental` - latest experimental releases after `1.0`
- `docker/dockerfile:experimental` - latest release on experimental channel

- `docker/dockerfile:1.0.1-experimental`- 仅允许不可变版本`1.0.1-experimental`
- `docker/dockerfile:1.0-experimental` - `1.0` 之后的最新实验版本
- `docker/dockerfile:experimental` - 实验版本上的最新版本

You should choose a channel that best fits your needs. If you only want bugfixes, you should use `docker/dockerfile:1.0`. If you want to benefit from experimental features, you should use the experimental channel. If you are using the experimental channel, newer releases may not be backwards compatible, so it is recommended to use an immutable full version variant.

您应该选择最适合自己需求的渠道。如果只想修正错误，则应使用 `docker/ dockerfile:1.0`。如果您想从实验功能中受益，则应使用实验频道。如果您正在使用实验性频道，则较新的版本可能无法向后兼容，因此建议使用不可变的完整版本。

For master builds and nightly feature releases refer to the description in [the source repository](https://github.com/moby/buildkit/blob/master/README.md).

有关主版本和每晚发布的功能，请参阅 [源码仓库](https://github.com/moby/buildkit/blob/master/README.md) 中的描述。


## 总结

介绍了 Dockerfile 指令解析器的 `syntax` 用法。