# 【Docker】Dockerfile 之 VOLUME

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 数据卷

```Dockerfile
VOLUME ["/data"]
```

The `VOLUME` instruction creates a mount point with the specified name and marks it as holding externally mounted volumes from native host or other containers. The value can be a JSON array, `VOLUME ["/var/log/"]`, or a plain string with multiple arguments, such as `VOLUME /var/log` or `VOLUME /var/log /var/db`. For more information/examples and mounting instructions via the Docker client, refer to [_Share Directories via Volumes_](https://docs.docker.com/storage/volumes/) documentation.

`VOLUME` 指令创建具有指定名称的挂载点，并将其标记为保存来自本地主机或其他容器的外部安装的数据卷。该值可以是 JSON 数组，`VOLUME ["/var/log/"]`，也可以是带有多个参数的纯字符串，例如 `VOLUME /var/log` 或 `VOLUME /var/log /var/db` 。有关通过 Docker 客户端的更多挂载说明，请参考 [_Share Directories via Volumes_](https://docs.docker.com/storage/volumes/) 文档。

The `docker run` command initializes the newly created volume with any data that exists at the specified location within the base image. For example, consider the following Dockerfile snippet:

`docker run` 命令使用基础镜像内指定位置上存在的任何数据初始化新创建的卷。例如，考虑以下 Dockerfile 片段：

```Dockerfile
FROM ubuntu
RUN mkdir /myvol
RUN echo "hello world" > /myvol/greeting
VOLUME /myvol
```

This Dockerfile results in an image that causes `docker run` to create a new mount point at `/myvol` and copy the `greeting` file into the newly created volume.

这个 Dockerfile 生成一个镜像，该镜像使 `docker run` 在`/myvol` 目录创建一个新的挂载点，并将 `greeting` 文件复制到新创建的卷中。

### 关于指定卷的注意事项

Keep the following things in mind about volumes in the `Dockerfile`.
关于 Dockerfile 中的卷，请记住以下几点。

- **Volumes on Windows-based containers**: When using Windows-based containers, the destination of a volume inside the container must be one of:
- **基于 Windows 的容器上的卷**：使用基于 Windows 的容器时，容器内卷的目的地必须是以下之一：

    - a non-existing or empty directory
    - a drive other than `C:`
    - 不存在或空目录
    - 除C：以外的驱动器

- **Changing the volume from within the Dockerfile**: If any build steps change the data within the volume after it has been declared, those changes will be discarded.
- **从 Dockerfile 内更改卷**：如果在声明了卷后有任何构建步骤更改了卷中的数据，则这些更改将被丢弃。
    
- **JSON formatting**: The list is parsed as a JSON array. You must enclose words with double quotes (`"`) rather than single quotes (`'`).
- **JSON 格式**：该列表被解析为 JSON 数组。您必须用双引号（`"`）而不是单引号（`'`）括起单词。
    
- **The host directory is declared at container run-time**: The host directory (the mountpoint) is, by its nature, host-dependent. This is to preserve image portability, since a given host directory can’t be guaranteed to be available on all hosts. For this reason, you can’t mount a host directory from within the Dockerfile. The `VOLUME` instruction does not support specifying a `host-dir` parameter. You must specify the mountpoint when you create or run the container.
- **主机目录是在容器运行时声明的**：主机目录（挂载点）从本质上说是依赖于主机的。这是为了保留镜像的可移植性，因为不能保证给定的主机目录在所有主机上都可用。因此，您无法从 Dockerfile 中挂载主机目录。`VOLUME` 指令不支持指定 `host-dir` 参数。创建或运行容器时，必须指定挂载点。

## 总结

介绍了 Dockerfile 中 VOLUME 指令的用法和注意事项。