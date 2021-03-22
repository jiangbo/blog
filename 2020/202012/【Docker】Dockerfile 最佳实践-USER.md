# 【Docker】Dockerfile 最佳实践-USER

参考教程：https://docs.docker.com/develop/develop-images/dockerfile_best-practices/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## USER

If a service can run without privileges, use `USER` to change to a non-root user. Start by creating the user and group in the `Dockerfile` with something like `RUN groupadd -r postgres && useradd --no-log-init -r -g postgres postgres`.

如果服务可以在没有特权的情况下运行，请使用 `USER` 更改为非 root 用户。首先在 Dockerfile 中创建用户和组，类似于 `RUN groupadd -r postgres && useradd --no-log-init -r -g postgres postgres`。

> Consider an explicit UID/GID
> 
> Users and groups in an image are assigned a non-deterministic UID/GID in that the “next” UID/GID is assigned regardless of image rebuilds. So, if it’s critical, you should assign an explicit UID/GID.

> 考虑一个明确的 UID/GID
>
> 为镜像中的用户和组分配了不确定的 UID/GID，因为无论镜像重建如何，都将分配“下一个” UID/GID。因此，如果有必要，您应该分配一个明确的 UID/GID。

> Due to an [unresolved bug](https://github.com/golang/go/issues/13548) in the Go archive/tar package’s handling of sparse files, attempting to create a user with a significantly large UID inside a Docker container can lead to disk exhaustion because `/var/log/faillog` in the container layer is filled with NULL (\\0) characters. A workaround is to pass the `--no-log-init` flag to useradd. The Debian/Ubuntu `adduser` wrapper does not support this flag.

> 由于 Go 软件包处理稀疏文件时出现[未解决的错误](https://github.com/golang/go/issues/13548)，试图在 Docker 中创建具有非常大的 UID 的用户容器可能会导致磁盘耗尽，因为容器层中的 `/var/log/faillog` 用 NULL (\\0) 字符填充。一种解决方法是将 `--no-log-init` 标志传递给 `useradd`。 Debian/Ubuntu `adduser`包装器不支持该标志。

Avoid installing or using `sudo` as it has unpredictable TTY and signal-forwarding behavior that can cause problems. If you absolutely need functionality similar to `sudo`, such as initializing the daemon as `root` but running it as non-`root`, consider using [“gosu”](https://github.com/tianon/gosu).

避免安装或使用 `sudo`，因为它具有不可预测的 TTY 和信号转发行为，可能会导致问题。如果您绝对需要类似于 sudo 的功能，例如将守护进程初始化为 root，但将其作为非 root 运行，请考虑使用 [“gosu”](https://github.com/tianon/gosu) 。

Lastly, to reduce layers and complexity, avoid switching `USER` back and forth frequently.

最后，为了减少层次和复杂性，请避免频繁地来回切换 `USER`。

## 总结

介绍了 Dockerfile 的 USER 指令的最佳实践。