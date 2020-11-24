# 【Docker】选择存储驱动

参考教程：https://docs.docker.com/storage/storagedriver/select-storage-driver/
以下内容来自官方文档翻译

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## Docker 的存储驱动

Ideally, very little data is written to a container’s writable layer, and you use Docker volumes to write data. However, some workloads require you to be able to write to the container’s writable layer. This is where storage drivers come in.

理想情况下，很少有数据被写入容器的可写层，并且您应该使用 Docker 数据卷来写入数据。但是在某些工作情况下，要求您能够写入容器的可写层。 这是存储驱动程序的由来。

Docker supports several different storage drivers, using a pluggable architecture. The storage driver controls how images and containers are stored and managed on your Docker host.

Docker 使用可插拔架构支持几种不同的存储驱动程序。存储驱动程序控制在 Docker 主机上如何存储和管理镜像和容器。

After you have read the storage driver overview, the next step is to choose the best storage driver for your workloads. In making this decision, there are three high-level factors to consider:

阅读存储驱动程序概述之后，下一步就是为您的工作负载选择最佳的存储驱动程序。在做出此决定时，需要考虑三个高层因素：

If multiple storage drivers are supported in your kernel, Docker has a prioritized list of which storage driver to use if no storage driver is explicitly configured, assuming that the storage driver meets the prerequisites.

如果您的内核支持多个存储驱动程序，则在未明确配置存储驱动程序的情况下，Docker 会优先列出要使用的存储驱动程序，前提是该存储驱动程序满足先决条件。

Use the storage driver with the best overall performance and stability in the most usual scenarios.

在大多数情况下，请使用具有最佳整体性能和稳定性的存储驱动程序。

Docker supports the following storage drivers:

Docker 支持以下存储驱动程序：

- `overlay2` is the preferred storage driver, for all currently supported Linux distributions, and requires no extra configuration.

- 对于所有当前支持的 Linux 发行版，`overlay2` 是首选的存储驱动程序，不需要任何额外的配置。

- `aufs` was the preferred storage driver for Docker 18.06 and older, when running on Ubuntu 14.04 on kernel 3.13 which had no support for `overlay2`.

- 在内核 3.13 上的 Ubuntu 14.04 上运行时，`aufs` 是 Docker 18.06 及更早版本的首选存储驱动程序，因为内核不支持`overlay2`。

- `devicemapper` is supported, but requires `direct-lvm` for production environments, because `loopback-lvm`, while zero-configuration, has very poor performance. `devicemapper` was the recommended storage driver for CentOS and RHEL, as their kernel version did not support `overlay2`. However, current versions of CentOS and RHEL now have support for `overlay2`, which is now the recommended driver.

- 虽然支持 `devicemapper`，但是在生产环境中需要 `direct-lvm`，因为 `loopback lvm` 零配置时性能很差。 `devicemapper` 是 CentOS 和 RHEL 的推荐存储驱动程序，在它们的内核版本不支持 `overlay2` 的时候。但是，当前版本的 CentOS 和 RHEL 现在支持 `overlay2`，现在是推荐的驱动程序。

- The `btrfs` and `zfs` storage drivers are used if they are the backing filesystem (the filesystem of the host on which Docker is installed). These filesystems allow for advanced options, such as creating “snapshots”, but require more maintenance and setup. Each of these relies on the backing filesystem being configured correctly.

- 如果 btrfs 和 zfs 存储驱动程序是后备文件系统（安装了 Docker 的主机的文件系统），则使用它们。这些文件系统允许使用高级选项，例如创建快照，但需要更多的维护和设置。这些中的每一个都依赖于正确配置的后备文件系统。

- The `vfs` storage driver is intended for testing purposes, and for situations where no copy-on-write filesystem can be used. Performance of this storage driver is poor, and is not generally recommended for production use.

- `vfs` 存储驱动程序用于测试目的，以及无法使用写时复制文件系统的情况。此存储驱动程序的性能很差，通常不建议在生产中使用。

Docker’s source code defines the selection order. You can see the order at [the source code for Docker Engine - Community 19.03](https://github.com/docker/docker-ce/blob/19.03/components/engine/daemon/graphdriver/driver_linux.go#L50)

Docker 的源代码定义了选择顺序。您可以在 Docker 的源代码中查看。

If you run a different version of Docker, you can use the branch selector at the top of the file viewer to choose a different branch.

如果运行其他版本的 Docker，则可以使用文件查看器顶部的分支选择器来选择其他分支。

Some storage drivers require you to use a specific format for the backing filesystem. If you have external requirements to use a specific backing filesystem, this may limit your choices. See [Supported backing filesystems](https://docs.docker.com/storage/storagedriver/select-storage-driver/#supported-backing-filesystems).

一些存储驱动程序要求您对后备文件系统使用特定格式。如果您有使用特定备份文件系统的外部要求，则可能会限制您的选择。请参阅[支持的备份文件系统](https://docs.docker.com/storage/storagedriver/select-storage-driver/#supported-backing-filesystems)。

After you have narrowed down which storage drivers you can choose from, your choice is determined by the characteristics of your workload and the level of stability you need. See [Other considerations](https://docs.docker.com/storage/storagedriver/select-storage-driver/#other-considerations) for help in making the final decision.

在确定了可以选择的存储驱动程序之后，您的选择取决于工作负载的特征和所需的稳定性。请参阅[其他注意事项](https://docs.docker.com/storage/storagedriver/select-storage-driver/#other-considerations)以获取最终决定的帮助。

> **NOTE**: Your choice may be limited by your operating system and distribution. For instance, `aufs` is only supported on Ubuntu and Debian, and may require extra packages to be installed, while `btrfs` is only supported on SLES, which is only supported with Docker Enterprise. See [Support storage drivers per Linux distribution](https://docs.docker.com/storage/storagedriver/select-storage-driver/#supported-storage-drivers-per-linux-distribution) for more information.

> **注意**：您的选择可能会受到操作系统和发行版的限制。例如，`aufs` 仅在 Ubuntu 和 Debian 上受支持，可能需要安装额外的软件包，而 `btrfs` 仅在 SLES 上受支持，SLES 仅受 Docker Enterprise 支持。有关更多信息，请参见[每个Linux发行版支持存储驱动程序](https://docs.docker.com/storage/storagedriver/select-storage-driver/#supported-storage-drivers-per-linux-distribution)。

## linux 发行版支持的存储驱动

At a high level, the storage drivers you can use is partially determined by the Docker edition you use.

In addition, Docker does not recommend any configuration that requires you to disable security features of your operating system, such as the need to disable `selinux` if you use the `overlay` or `overlay2` driver on CentOS.

在较高级别上，可以使用的存储驱动程序部分取决于所使用的 Docker 版本。

此外，Docker 不建议您进行任何禁用操作系统安全功能的配置，例如，如果您在 CentOS 上使用 overlay 或 overlay2 驱动程序，则需要禁用selinux。

### 社区版 Docker

For Docker Engine - Community, only some configurations are tested, and your operating system’s kernel may not support every storage driver. In general, the following configurations work on recent versions of the Linux distribution:

对于 Docker 社区版，仅测试了一些配置，并且您操作系统的内核可能不支持每个存储驱动程序。通常，以下配置适用于最新版本的Linux发行版：

| Linux distribution | Recommended storage drivers | Alternative drivers |
| --- | --- | --- |
| Docker Engine - Community on Ubuntu | `overlay2` or `aufs` (for Ubuntu 14.04 running on kernel 3.13) | `overlay`¹, `devicemapper`², `zfs`, `vfs` |
| Docker Engine - Community on Debian | `overlay2` (Debian Stretch), `aufs` or `devicemapper` (older versions) | `overlay`¹, `vfs` |
| Docker Engine - Community on CentOS | `overlay2` | `overlay`¹, `devicemapper`², `zfs`, `vfs` |
| Docker Engine - Community on Fedora | `overlay2` | `overlay`¹, `devicemapper`², `zfs`, `vfs` |

¹) The overlay storage driver is deprecated, and will be removed in a future release. It is recommended that users of the overlay storage driver migrate to overlay2.

¹）覆盖存储驱动程序已弃用，并将在以后的版本中删除。建议覆盖存储驱动程序的用户迁移到 overlay2。

²) The devicemapper storage driver is deprecated, and will be removed in a future release. It is recommended that users of the devicemapper storage driver migrate to overlay2.

²）devicemapper 存储驱动程序已过时，将在以后的版本中删除。建议 devicemapper 存储驱动程序的用户迁移到 overlay2。

When possible, overlay2 is the recommended storage driver. When installing Docker for the first time, overlay2 is used by default. Previously, aufs was used by default when available, but this is no longer the case. If you want to use aufs on new installations going forward, you need to explicitly configure it, and you may need to install extra packages, such as linux-image-extra. See aufs.

可能的话，overlay2 是推荐的存储驱动程序。首次安装 Docker 时，默认使用 overlay2。以前，默认情况下会使用 aufs（如果有），但现在情况不再如此。如果要在以后的新安装中使用 aufs，则需要对其进行显式配置，并且可能需要安装其他软件包，例如 linux-image-extra。

On existing installations using aufs, it is still used.

在使用 aufs 的现有安装中，仍会使用它。

When in doubt, the best all-around configuration is to use a modern Linux distribution with a kernel that supports the overlay2 storage driver, and to use Docker volumes for write-heavy workloads instead of relying on writing data to the container’s writable layer.

如有疑问，最好的全方位配置是使用具有支持 overlay2 存储驱动程序的内核的现代 Linux 发行版，并使用 Docker 数据卷处理繁重的工作负载，而不是依赖于将数据写入容器的可写层。

The vfs storage driver is usually not the best choice. Before using the vfs storage driver, be sure to read about its performance and storage characteristics and limitations.

vfs 存储驱动程序通常不是最佳选择。在使用vfs存储驱动程序之前，请务必阅读其性能，存储特性和限制。

## Supported backing filesystems

With regard to Docker, the backing filesystem is the filesystem where `/var/lib/docker/` is located. Some storage drivers only work with specific backing filesystems.

对于Docker，支持文件系统是 `/var/lib/docker/` 所在的文件系统。一些存储驱动程序仅适用于特定的后备文件系统。

| Storage driver | Supported backing filesystems |
| --- | --- |
| `overlay2`, `overlay` | `xfs` with ftype=1, `ext4` |
| `aufs` | `xfs`, `ext4` |
| `devicemapper` | `direct-lvm` |
| `btrfs` | `btrfs` |
| `zfs` | `zfs` |
| `vfs` | any filesystem |

## 其它注意事项

### 适合工作负载

Among other things, each storage driver has its own performance characteristics that make it more or less suitable for different workloads. Consider the following generalizations:

除其他事项外，每个存储驱动程序都有其自己的性能特征，这使其或多或少地适合于不同的工作负载。考虑以下概括：

- `overlay2`, `aufs`, and `overlay` all operate at the file level rather than the block level. This uses memory more efficiently, but the container’s writable layer may grow quite large in write-heavy workloads.
- Block-level storage drivers such as `devicemapper`, `btrfs`, and `zfs` perform better for write-heavy workloads (though not as well as Docker volumes).
- For lots of small writes or containers with many layers or deep filesystems, `overlay` may perform better than `overlay2`, but consumes more inodes, which can lead to inode exhaustion.
- `btrfs` and `zfs` require a lot of memory.
- `zfs` is a good choice for high-density workloads such as PaaS.

- `overlay2`，`aufs` 和 `overlay` 都在文件级别而不是块级别运行。这样可以更有效地使用内存，但是在写繁重的工作负载中，容器的可写层可能会变得很大。

- 诸如 `devicemapper`，`btrfs` 和 `zfs` 之类的块级存储驱动程序在写繁重的工作负载（尽管不如Docker 数据卷）上表现更好。

- 对于许多具有多个层或较深文件系统的小型写入或容器，`overlay` 的性能可能优于 `overlay2`，但会消耗更多的inode，这可能导致 inode 耗尽。

- `btrfs` 和 `zfs` 需要大量内存。
- `zfs` 是高密度工作负载（例如PaaS）的不错选择。

More information about performance, suitability, and best practices is available in the documentation for each storage driver.

有关每个存储驱动程序的性能，适用性和最佳实践的更多信息，请参见每个存储驱动的文档。

### Shared storage systems and the storage driver

If your enterprise uses SAN, NAS, hardware RAID, or other shared storage systems, they may provide high availability, increased performance, thin provisioning, deduplication, and compression. In many cases, Docker can work on top of these storage systems, but Docker does not closely integrate with them.

如果您的企业使用 SAN，NAS，硬件 RAID 或其他共享存储系统，则它们可以提供高可用性，增强的性能，自动精简配置，重复数据删除和压缩。在许多情况下，Docker 可以在这些存储系统上运行，但是 Docker 并未与其紧密集成。

Each Docker storage driver is based on a Linux filesystem or volume manager. Be sure to follow existing best practices for operating your storage driver (filesystem or volume manager) on top of your shared storage system. For example, if using the ZFS storage driver on top of a shared storage system, be sure to follow best practices for operating ZFS filesystems on top of that specific shared storage system.

每个 Docker 存储驱动程序均基于 Linux 文件系统或卷管理器。确保在共享存储系统之上遵循用于操作存储驱动程序（文件系统或卷管理器）的现有最佳实践。例如，如果在共享存储系统上使用 ZFS 存储驱动程序，请确保遵循最佳实践在该特定共享存储系统上操作 ZFS 文件系统。

### Stability

For some users, stability is more important than performance. Though Docker considers all of the storage drivers mentioned here to be stable, some are newer and are still under active development. In general, `overlay2`, `aufs`, `overlay`, and `devicemapper` are the choices with the highest stability.

对于某些用户而言，稳定性比性能更重要。尽管 Docker 认为此处提到的所有存储驱动程序都是稳定的，但其中一些是较新的并且仍在积极开发中。通常，`overlay2`，`aufs`，`overlay` 和 `devicemapper` 是具有最高稳定性的选择。

### Test with your own workloads

You can test Docker’s performance when running your own workloads on different storage drivers. Make sure to use equivalent hardware and workloads to match production conditions, so you can see which storage driver offers the best overall performance.

您可以在不同的存储驱动程序上运行自己的工作负载时测试 Docker 的性能。确保使用等效的硬件和工作负载来匹配生产条件，以便您可以看到哪个存储驱动程序提供最佳的整体性能。

## Check your current storage driver

The detailed documentation for each individual storage driver details all of the set-up steps to use a given storage driver.

每个单独的存储驱动程序的详细文档详细说明了使用给定存储驱动程序的所有设置步骤。

## 查看使用的存储驱动

```sh
[root@node1 vcloud_app]# docker info
Client:
 Debug Mode: false

Server:
 Containers: 360
  Running: 298
  Paused: 0
  Stopped: 62
 Images: 207
 Server Version: 19.03.13
 Storage Driver: overlay2
  Backing Filesystem: xfs
  Supports d_type: true
  Native Overlay Diff: true
 Logging Driver: json-file
 Cgroup Driver: systemd
 Plugins:
  Volume: local
  Network: bridge host ipvlan macvlan null overlay
  Log: awslogs fluentd gcplogs gelf journald json-file local logentries splunk syslog
 Swarm: inactive
 ...
```

To change the storage driver, see the specific instructions for the new storage driver. Some drivers require additional configuration, including configuration to physical or logical disks on the Docker host.

要更改存储驱动程序，请参阅新存储驱动程序的特定说明。一些驱动程序需要其他配置，包括对 Docker 主机上的物理或逻辑磁盘的配置。

> **Important**: When you change the storage driver, any existing images and containers become inaccessible. This is because their layers cannot be used by the new storage driver. If you revert your changes, you can access the old images and containers again, but any that you pulled or created using the new driver are then inaccessible.

> **重要**：当您更改存储驱动程序时，所有现有的镜像和容器都将无法访问。这是因为新存储驱动程序无法使用其图层。 如果还原更改，则可以再次访问旧的镜像和容器，但是使用新驱动程序拉取或创建的任何镜像和容器将无法访问。

## 总结

介绍了存储驱动的种类，以及怎么选择存储驱动，综合考虑来看，推荐使用 overlay2。