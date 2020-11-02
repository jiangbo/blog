# 【Docker】存储概述

参考教程：https://docs.docker.com/get-started/overview/
以下内容来自官方文档翻译

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 在 Docker 中的数据管理

By default all files created inside a container are stored on a writable container layer. This means that:
默认情况下，在容器内创建的所有文件都存储在可写的容器层上。这意味着：

- The data doesn’t persist when that container no longer exists, and it can be difficult to get the data out of the container if another process needs it.
- A container’s writable layer is tightly coupled to the host machine where the container is running. You can’t easily move the data somewhere else.
- Writing into a container’s writable layer requires a [storage driver](https://docs.docker.com/storage/storagedriver/) to manage the filesystem. The storage driver provides a union filesystem, using the Linux kernel. This extra abstraction reduces performance as compared to using _data volumes_, which write directly to the host filesystem.

- 当该容器不存在时，数据不会保留。如果另一个进程需要数据，则很难从容器中获取数据。
- 容器的可写层与运行容器的主机紧密耦合。您不能轻松地将数据移动到其他位置。
- 写入容器的可写层需要[存储驱动](https://docs.docker.com/storage/storagedriver/)来管理文件系统。存储驱动程序使用 Linux 内核提供联合文件系统。与直接写入主机文件系统的数据卷相比，这种额外的抽象降低了性能。

Docker has two options for containers to store files in the host machine, so that the files are persisted even after the container stops: volumes, and bind mounts. If you’re running Docker on Linux you can also use a tmpfs mount. If you’re running Docker on Windows you can also use a named pipe.

Docker 有两个选项用于容器在主机中存储文件，以便于即使在容器停止之后，文件仍可以保留，它们分别是：数据卷（volumes）和绑定挂载（bind mounts）。如果您在 Linux 上运行 Docker，也可以使用 tmpfs 挂载。如果在 Windows 上运行 Docker，也可以使用命名管道。

Keep reading for more information about these two ways of persisting data.

下面是这两种持久化数据方法的更多信息。

## 选择正确的挂载类型

No matter which type of mount you choose to use, the data looks the same from within the container. It is exposed as either a directory or an individual file in the container’s filesystem.

无论您选择使用哪种挂载类型，数据在容器中看起来都一样。它暴露为容器文件系统中的目录或单个文件。

An easy way to visualize the difference among volumes, bind mounts, and tmpfs mounts is to think about where the data lives on the Docker host.

一种简单区分它们之间差异是：数据卷（volumes），绑定挂载（bind mounts）和 tmpfs 之间的不同是它们位于 Docker 主机的不同位置。

![types-of-mounts][1]

- **Volumes** are stored in a part of the host filesystem which is managed by Docker (`/var/lib/docker/volumes/` on Linux). Non-Docker processes should not modify this part of the filesystem. Volumes are the best way to persist data in Docker.

- **数据卷** 存储在由 Docker 管理的主机文件系统上（在 Linux 上是`/var/lib/docker/volumes/`）。非 Docker 进程不应修改这部分的文件系统。数据卷是持久化 Docker 数据的最佳方法。
  
- **Bind mounts** may be stored anywhere on the host system. They may even be important system files or directories. Non-Docker processes on the Docker host or a Docker container can modify them at any time.

- **绑定挂载** 可以存储在在主机系统的任意位置。它们甚至可能是重要的系统文件或目录。Docker 主机或 Docker 容器上的非 Docker 进程可随时修改它们。
  
- **`tmpfs` mounts** are stored in the host system’s memory only, and are never written to the host system’s filesystem.

- **`tmpfs`** 挂载仅存储在主机系统的内存中，并且永远不会写入主机系统的文件系统。

## 更详细的挂载类型

### 数据卷

Created and managed by Docker. You can create a volume explicitly using the `docker volume create` command, or Docker can create a volume during container or service creation.

由 Docker 创建和管理。您可以使用 `docker volume create` 命令显式创建数据卷，或者 Docker 可以在容器或服务创建时创建卷。
    
When you create a volume, it is stored within a directory on the Docker host. When you mount the volume into a container, this directory is what is mounted into the container. This is similar to the way that bind mounts work, except that volumes are managed by Docker and are isolated from the core functionality of the host machine.

当你创建数据卷时，它存储在 Docker 主机上的一个目录中。当你挂载一个数据卷到容器中，这个目录也被挂载到容器中。这和绑定挂载很相似，除了数据卷是直接被 Docker 管理，并且与主机的核心功能隔离。
    
A given volume can be mounted into multiple containers simultaneously. When no running container is using a volume, the volume is still available to Docker and is not removed automatically. You can remove unused volumes using `docker volume prune`.

一个给定的数据卷可以同时挂载在多个容器中。当没有正在运行的容器使用数据卷时，Docker 仍然可以使用这个数据卷，并且不会自动删除该卷。可以使用 `docker volume prune` 删除未使用的卷。

When you mount a volume, it may be **named** or **anonymous**. Anonymous volumes are not given an explicit name when they are first mounted into a container, so Docker gives them a random name that is guaranteed to be unique within a given Docker host. Besides the name, named and anonymous volumes behave in the same ways.

挂载数据卷时，可以命名或匿名。匿名卷是首次挂载到容器时没有给定名称，因此 Docker 为它们提供了一个随机名称，该名称保证在给定的 Docker 主机中是唯一的。除了名称之外，命名卷和匿名卷的行为方式相同。

Volumes also support the use of volume drivers, which allow you to store your data on remote hosts or cloud providers, among other possibilities.

数据卷还支持使用卷驱动程序，它允许您将数据存储在远程主机或云提供商上，或者其它方式。

### 绑定挂载

Available since the early days of Docker. Bind mounts have limited functionality compared to volumes. When you use a bind mount, a file or directory on the host machine is mounted into a container. The file or directory is referenced by its full path on the host machine. The file or directory does not need to exist on the Docker host already. It is created on demand if it does not yet exist. Bind mounts are very performant, but they rely on the host machine’s filesystem having a specific directory structure available. If you are developing new Docker applications, consider using named volumes instead. You can’t use Docker CLI commands to directly manage bind mounts.

绑定挂载在 Docker的早期就可用了。与数据卷相比，绑定挂载的功能有限。使用绑定挂载时，主机上的文件或目录将挂载到容器中。文件或目录由其在主机上的完整路径引用。文件或目录不需要已经存在于 Docker 主机上。如果尚不存在，则按需创建它。绑定挂载有很高的性能，但它们依赖于具有特定的目录结构可用的主机文件系统。如果要开发新的 Docker 应用程序，请考虑使用命名卷。不能使用 Docker CLI 命令直接管理绑定挂载。

> Bind mounts allow access to sensitive files
>
> One side effect of using bind mounts, for better or for worse, is that you can change the **host** filesystem via processes running in a **container**, including creating, modifying, or deleting important system files or directories. This is a powerful ability which can have security implications, including impacting non-Docker processes on the host system.

>绑定挂载允许访问敏感文件
>
>使用绑定挂载的一个副作用是，通过容器中运行的进程（包括创建、修改或删除重要的系统文件或目录）来更改主机文件系统。这是一种强大的功能，可以产生安全影响，包括影响主机系统上的非 Docker 进程。

### tmpfs

A `tmpfs` mount is not persisted on disk, either on the Docker host or within a container. It can be used by a container during the lifetime of the container, to store non-persistent state or sensitive information. For instance, internally, swarm services use `tmpfs` mounts to mount [secrets](https://docs.docker.com/engine/swarm/secrets/) into a service’s containers.

`tmpfs` 挂载不会保存在磁盘上，也不会保存在在 Docker 主机上或容器中。容器可以在容器的生存期内使用它来存储非持久性状态或敏感信息。例如，在内部，swarm 服务使用挂载将 `secrets` 挂载到服务的容器中。

### named pipes

An `npipe` mount can be used for communication between the Docker host and a container. Common use case is to run a third-party tool inside of a container and connect to the Docker Engine API using a named pipe.

named pipes 可用于 Docker 主机和容器之间的通信。常见用例是在容器内运行第三方工具，并使用命名管道连接到 Docker Engine API。

Bind mounts and volumes can both be mounted into containers using the -v or --volume flag, but the syntax for each is slightly different. For tmpfs mounts, you can use the --tmpfs flag. We recommend using the --mount flag for both containers and services, for bind mounts, volumes, or tmpfs mounts, as the syntax is more clear.

绑定挂载和数据卷都可以使用 `-v` 或 `--volume` 标志挂载到容器中，但它们两个的语法略有不同。对于 tmpfs 挂载，可以使用 `--tmpfs` 标志。对于容器和服务，绑定挂载，数据卷和 tmpfs 挂载，我们建议使用 `--mount` 标志，因为语法更加清晰。

## 数据卷的好用例

Volumes are the preferred way to persist data in Docker containers and services. Some use cases for volumes include:

数据卷是将数据保留在 Docker 容器和服务中的首选方法。数据卷的一些用例包括：

- Sharing data among multiple  runningcontainers. If you don’t explicitly create it, a volume is created the first time it is mounted into a container. When that container stops or is removed, the volume still exists. Multiple containers can mount the same volume simultaneously, either read-write or read-only. Volumes are only removed when you explicitly remove them.

- 在多个正在运行的容器之间共享数据。如果未显式创建它，则在首次将卷装入容器时将创建数据卷。当该容器停止或删除时，卷仍然存在。多个容器可以同时挂载同一卷，无论是读写还是只读。仅当您显式删除卷时，才删除它们。

- When the Docker host is not guaranteed to have a given directory or file structure. Volumes help you decouple the configuration of the Docker host from the container runtime.

- 当 Docker 主机不能保证具有给定的目录或文件结构时。卷可帮助您将 Docker 主机的配置与容器运行时分离。

- When you want to store your container’s data on a remote host or a cloud provider, rather than locally.

- 当您想要将容器的数据存储在远程主机或云提供商上，而不是在本地存储时。

- When you need to back up, restore, or migrate data from one Docker host to another, volumes are a better choice. You can stop containers using the volume, then back up the volume’s directory (such as `/var/lib/docker/volumes/<volume-name>`).

- 当您需要备份、还原或将数据从一个 Docker 主机迁移到另一个主机时，卷是更好的选择。您可以使用卷停止容器，然后备份卷的目录（如 `/var/lib/docker/volumes/<volume-name>`）。

- When your application requires high-performance I/O on Docker Desktop. Volumes are stored in the Linux VM rather than the host, which means that the reads and writes have much lower latency and higher throughput.

- 当应用程序在 Docker Desktop 上需要高性能 I/O 时。数据卷存储在 Linux VM 中，而不是主机中，这意味着读取和写入具有更低的延迟和更高的吞吐量。

- When your application requires fully native file system behavior on Docker Desktop. For example, a database engine requires precise control over disk flushing to guarantee transaction durability. Volumes are stored in the Linux VM and can make these guarantees, whereas bind mounts are remoted to macOS or Windows, where the file systems behave slightly differently.

- 当应用程序需要在 Docker Desktop 上完全本机文件系统行为时。例如，数据库引擎需要精确控制磁盘刷新，以确保事务持久性。卷存储在 Linux VM 中，可以做出这些保证，而绑定挂载则远程访问 macOS 或 Windows，其中文件系统的行为略有不同。

## 绑定挂载的好用例

In general, you should use volumes where possible. Bind mounts are appropriate for the following types of use case:

通常，应尽可能使用数据卷。绑定挂载适用于以下类型的用例：

- Sharing configuration files from the host machine to containers. This is how Docker provides DNS resolution to containers by default, by mounting `/etc/resolv.conf` from the host machine into each container.

- 将配置文件从主机共享到容器。默认情况下，Docker 通过从主机挂载 ``/etc/resolv.conf`` 到每个容器中来为容器提供 DNS 解析。

- Sharing source code or build artifacts between a development environment on the Docker host and a container. For instance, you may mount a Maven `target/` directory into a container, and each time you build the Maven project on the Docker host, the container gets access to the rebuilt artifacts.

- 在 Docker 主机和容器上的开发环境之间共享源代码或生成项目。例如，您可以将 Maven `target/` 目录挂载到容器中，并且每次在 Docker 主机上生成 Maven 项目时，容器都会访问重建的项目。

    If you use Docker for development this way, your production Dockerfile would copy the production-ready artifacts directly into the image, rather than relying on a bind mount.

    如果使用 Docker 进行这种开发，则生产环境的 Dockerfile 应该将生产就绪工件直接复制到映像中，而不是依赖于绑定挂载。

- When the file or directory structure of the Docker host is guaranteed to be consistent with the bind mounts the containers require.

- 当 Docker 主机的文件或目录结构保证与容器需要绑定挂载一致时。

## tmpfs 的好用例

`tmpfs` mounts are best used for cases when you do not want the data to persist either on the host machine or within the container. This may be for security reasons or to protect the performance of the container when your application needs to write a large volume of non-persistent state data.

`tmpfs` 挂载最适合在不希望数据在主机或容器中保留的情况下使用。这可能是出于安全原因，或者当应用程序需要写入大量非持久性状态数据时，保护容器的性能。

## 使用数据卷和绑定挂载的提示

If you use either bind mounts or volumes, keep the following in mind:

如果使用绑定挂载或者数据卷，请记住以下事项：

- If you mount an **empty volume** into a directory in the container in which files or directories exist, these files or directories are propagated (copied) into the volume. Similarly, if you start a container and specify a volume which does not already exist, an empty volume is created for you. This is a good way to pre-populate data that another container needs.

- 如果将空**数据卷**挂载到存在文件或目录的容器中的目录中，这些文件或目录将被传播（复制）到卷中。同样，如果启动容器并指定不存在的卷，则将为你创建一个空卷。这是预填充另一个容器需要的数据的一个好方法。

- If you mount a **bind mount or non-empty volume** into a directory in the container in which some files or directories exist, these files or directories are obscured by the mount, just as if you saved files into `/mnt` on a Linux host and then mounted a USB drive into `/mnt`. The contents of `/mnt` would be obscured by the contents of the USB drive until the USB drive were unmounted. The obscured files are not removed or altered, but are not accessible while the bind mount or volume is mounted.

- 如果使用**绑定挂载或者非空数据卷**到容器中存在某些文件或目录的目录中，则这些文件或目录会被挂载所遮盖，就像将文件保存在 Linux 主机上，然后将 USB 驱动器挂载到 中一样。在 USB 驱动器卸载之前，其内容将被 USB 驱动器的内容遮盖。模糊的文件不会删除或更改，但在挂载绑定挂载或卷时无法访问。

## 总结

介绍了数据卷，绑定挂载和 tmpfs 的概念，以及它们使用场景和对比。

[1]:images/types-of-mounts.png