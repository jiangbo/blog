# 【Docker】使用 OverlayFS 存储驱动

参考教程：https://docs.docker.com/storage/storagedriver/overlayfs-driver/
以下内容来自官方文档翻译

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

OverlayFS is a modern union filesystem that is similar to AUFS, but faster and with a simpler implementation. Docker provides two storage drivers for OverlayFS: the original `overlay`, and the newer and more stable `overlay2`.

OverlayFS 是一种现代的联合文件系统，与 AUFS 类似，但是速度更快且实现更简单。Docker 为 OverlayFS 提供了两个存储驱动程序：原始的 `overlay` 和更新且更稳定的 `overlay2`。

This topic refers to the Linux kernel driver as `OverlayFS` and to the Docker storage driver as `overlay` or `overlay2`.

本主题将 Linux 内核驱动程序称为 `OverlayFS`，并将 Docker 存储驱动程序称为 `overlay` 或 `overlay2`。

> **Note**: If you use OverlayFS, use the `overlay2` driver rather than the `overlay` driver, because it is more efficient in terms of inode utilization. To use the new driver, you need version 4.0 or higher of the Linux kernel, or RHEL or CentOS using version 3.10.0-514 and above.

> **注意**：如果使用 OverlayFS，请使用 `overlay2` 驱动程序而不是 `overlay` 驱动程序，因为它在 inode 利用率方面更为有效。要使用新的驱动程序，您需要 Linux 内核的版本 4.0 或更高版本，或者使用 3.10.0-514 及更高版本的 RHEL 或 CentOS。

## 前置条件

OverlayFS is the recommended storage driver, and supported if you meet the following prerequisites:

OverlayFS 是推荐的存储驱动程序，如果满足以下先决条件，则受支持：

- Version 4.0 or higher of the Linux kernel, or RHEL or CentOS using version 3.10.0-514 of the kernel or higher. If you use an older kernel, you need to use the `overlay` driver, which is not recommended.

- Linux 内核的版本 4.0 或更高版本，或使用内核的版本 3.10.0-514 或更高版本的 RHEL 或 CentOS。如果您使用较旧的内核，则需要使用 `overlay` 驱动程序，不建议这样做。

- The `overlay` and `overlay2` drivers are supported on `xfs` backing filesystems, but only with `d_type=true` enabled.

- `xfs` 支持文件系统支持 `overlay` 和 `overlay2` 驱动程序，但仅在启用了d_type = true 的情况下。
  
    Use `xfs_info` to verify that the `ftype` option is set to `1`. To format an `xfs` filesystem correctly, use the flag `-n ftype=1`.

    使用`xfs_info`来验证`ftype`选项是否设置为`1`。要正确格式化 xfs 文件系统，请使用标志 `-n ftype = 1`。
    
> **Warning**: Running on XFS without d_type support now causes Docker to skip the attempt to use the `overlay` or `overlay2` driver. Existing installs will continue to run, but produce an error. This is to allow users to migrate their data. In a future version, this will be a fatal error, which will prevent Docker from starting.

 > **警告**：在不支持 d_type 的 XFS 上运行现在会导致 Docker 跳过使用 `overlay` 或 `overlay2` 驱动程序的尝试。现有安装将继续运行，但会提示错误，这是为了允许用户迁移其数据。在将来的版本中，这将是一个致命错误，它将阻止Docker启动。

- Changing the storage driver makes existing containers and images inaccessible on the local system. Use `docker save` to save any images you have built or push them to Docker Hub or a private registry before changing the storage driver, so that you do not need to re-create them later.

- 更改存储驱动程序将使现有容器和映像在本地系统上不可访问。使用 `docker save` 保存您已构建的任何映像，或在更改存储驱动程序之前将其推送到 Docker Hub 或私有注册表，这样您以后就无需重新创建它们。

## overlay2 驱动怎样工作

If you are still using the `overlay` driver rather than `overlay2`, see [How the overlay driver works](https://docs.docker.com/storage/storagedriver/overlayfs-driver/#how-the-overlay-driver-works) instead.

如果您仍在使用 `overlay` 驱动程序而不是 `overlay2`，请参阅 overlay 驱动程序的工作原理。

OverlayFS layers two directories on a single Linux host and presents them as a single directory. These directories are called _layers_ and the unification process is referred to as a _union mount_. OverlayFS refers to the lower directory as `lowerdir` and the upper directory a `upperdir`. The unified view is exposed through its own directory called `merged`.

OverlayFS 在单个 Linux 主机上有两个目录，并将它们合并显示为单个目录。这些目录称为 `layers`，合并的过程称为联合挂载。OverlayFS 将较低的目录称为 `lowerdir`，而将较高的目录称为 `upperdir`。 合并后的视图通过 `merged`目录暴露。

The `overlay2` driver natively supports up to 128 lower OverlayFS layers. This capability provides better performance for layer-related Docker commands such as `docker build` and `docker commit`, and consumes fewer inodes on the backing filesystem.

`overlay2` 驱动程序原生支持多达 128 个较低的 OverlayFS 层。此功能可为与层相关的 Docker 命令（如docker build 和 docker commit）提供更好的性能，并在支持文件系统上消耗更少的 inode。

### 磁盘上的镜像和容器层

After downloading a five-layer image using `docker pull ubuntu`, you can see six directories under `/var/lib/docker/overlay2`.

使用 `docker pull ubuntu` 下载五层镜像后，可以在 `/var/lib/docker/overlay2` 下看到六个目录。

> **Warning**: Do not directly manipulate any files or directories within `/var/lib/docker/`. These files and directories are managed by Docker.

> **警告**：请勿直接在 `/var/lib/docker/` 中操作任何文件或目录。这些文件和目录由 Docker 管理。

```sh
$ docker pull ubuntu
Using default tag: latest
latest: Pulling from library/ubuntu
6a5697faee43: Pull complete 
ba13d3bc422b: Pull complete 
a254829d9e55: Pull complete 
Digest: sha256:fff16eea1a8ae92867721d90c59a75652ea66d29c05294e6e2f898704bdb8cf1
Status: Downloaded newer image for ubuntu:latest
docker.io/library/ubuntu:latest

$ ls -l
total 0
drwx------    4 root     root            55 Nov 24 10:51 57bb3984fedd79355d16d090824068eb25a262897bd03f4cc903458a270fb135
drwx------    4 root     root            72 Nov 24 10:51 63492365c5b50b0b87988f7d7b90c817f87d9a4a88be4efd604ece8ce987e932
drwx------    3 root     root            47 Nov 24 10:51 654a21568916b79f8ce2b0b9795710980ef429d3b4d136a1cd6fdefabe8a4bed
brw-------    1 root     root        8,  16 Nov 24 10:50 backingFsBlockDev
drwx------    2 root     root           108 Nov 24 10:51 l
```

The new l (lowercase L) directory contains shortened layer identifiers as symbolic links. These identifiers are used to avoid hitting the page size limitation on arguments to the mount command.

新的 l（小写L）目录包含缩短的层标识符作为符号链接。这些标识符用于避免在 mount 命令的参数上达到页面大小限制。

```sh
$ ls -l l
total 0
lrwxrwxrwx    1 root     root            72 Nov 24 10:51 6ZP27KCNIOAMZOSGMQJXZPXB5J -> ../654a21568916b79f8ce2b0b9795710980ef429d3b4d136a1cd6fdefabe8a4bed/diff
lrwxrwxrwx    1 root     root            72 Nov 24 10:51 OOHUGSX2SX7GFNVJQ6TQL4ZWW7 -> ../63492365c5b50b0b87988f7d7b90c817f87d9a4a88be4efd604ece8ce987e932/diff
lrwxrwxrwx    1 root     root            72 Nov 24 10:51 TKENHGOKTAE4UDAMIQPWY65JBU -> ../57bb3984fedd79355d16d090824068eb25a262897bd03f4cc903458a270fb135/diff
```

The lowest layer contains a file called `link`, which contains the name of the shortened identifier, and a directory called `diff` which contains the layer’s contents.

最低层包含一个名为 `link` 的文件，其中包含缩短的标识符的名称；一个目录是 `diff`，其中包含该层的内容。

```ls
[node1] (local) root@192.168.0.23 /var/lib/docker/overlay2
$ cat 654a21568916b79f8ce2b0b9795710980ef429d3b4d136a1cd6fdefabe8a4bed/link 
6ZP27KCNIOAMZOSGMQJXZPXB5J[node1] (local) root@192.168.0.23 /var/lib/docker/overlay2
$ ls -l l
total 0
lrwxrwxrwx    1 root     root            72 Nov 24 10:51 6ZP27KCNIOAMZOSGMQJXZPXB5J -> ../654a21568916b79f8ce2b0b9795710980ef429d3b4d136a1cd6fdefabe8a4bed/diff
lrwxrwxrwx    1 root     root            72 Nov 24 10:51 OOHUGSX2SX7GFNVJQ6TQL4ZWW7 -> ../63492365c5b50b0b87988f7d7b90c817f87d9a4a88be4efd604ece8ce987e932/diff
lrwxrwxrwx    1 root     root            72 Nov 24 10:51 TKENHGOKTAE4UDAMIQPWY65JBU -> ../57bb3984fedd79355d16d090824068eb25a262897bd03f4cc903458a270fb135/diff
[node1] (local) root@192.168.0.23 /var/lib/docker/overlay2
$ cat 654a21568916b79f8ce2b0b9795710980ef429d3b4d136a1cd6fdefabe8a4bed/link 
6ZP27KCNIOAMZOSGMQJXZPXB5J[node1] (local) root@192.168.0.23 /var/lib/docker/overlay2
$ ls 57bb3984fedd79355d16d090824068eb25a262897bd03f4cc903458a270fb135/diff/
run
[node1] (local) root@192.168.0.23 /var/lib/docker/overlay2
$ ls 63492365c5b50b0b87988f7d7b90c817f87d9a4a88be4efd604ece8ce987e932/diff
etc  usr  var
[node1] (local) root@192.168.0.23 /var/lib/docker/overlay2
$ ls 654a21568916b79f8ce2b0b9795710980ef429d3b4d136a1cd6fdefabe8a4bed/diff
bin     dev     home    lib32   libx32  mnt     proc    run     srv     tmp     var
boot    etc     lib     lib64   media   opt     root    sbin    sys     usr
[node1] (local) root@192.168.0.23 /var/lib/docker/overlay2
$ ^C
[node1] (local) root@192.168.0.23 /var/lib/docker/overlay2
$ 
```

The second-lowest layer, and each higher layer, contain a file called `lower`, which denotes its parent, and a directory called `diff` which contains its contents. It also contains a `merged` directory, which contains the unified contents of its parent layer and itself, and a `work` directory which is used internally by OverlayFS.

第二最低的层，以及每个较高的层，包含一个名为 `lower` 的文件（表示其父文件）和一个名为 `diff` 的目录，该文件包含其内容。它还包含一个 `merged` 目录，该目录包含其父层及其本身的统一内容，以及一个 `work` 目录，供 OverlayFS 内部使用。

## 容器使用 overlay 读写

### 读取文件

Consider three scenarios where a container opens a file for read access with overlay.

考虑三种容器使用 overlay 打开文件来读取的场景。

- **The file does not exist in the container layer**: If a container opens a file for read access and the file does not already exist in the container (`upperdir`) it is read from the image (`lowerdir)`. This incurs very little performance overhead.

- **文件在容器层中不存在**：如果容器打开文件进行读取访问，并且文件在容器中不存在（`upperdir`），则从镜像（`lowerdir`）中读取。这几乎不会产生性能开销。
  
- **The file only exists in the container layer**: If a container opens a file for read access and the file exists in the container (`upperdir`) and not in the image (`lowerdir`), it is read directly from the container.

- **文件仅存在于容器层中**：如果容器打开文件进行读取访问，并且文件存在于容器中（`upperdir`）而不存在于镜像中（`lowerdir`），则直接从容器层读取。
  
- **The file exists in both the container layer and the image layer**: If a container opens a file for read access and the file exists in the image layer and the container layer, the file’s version in the container layer is read. Files in the container layer (`upperdir`) obscure files with the same name in the image layer (`lowerdir`).

- **文件同时存在于容器层和图像层中**：如果容器打开文件进行读取访问，并且文件存在于镜像层和容器层中，则将读取容器层中文件的版本。容器层（`upperdir`）中的文件会遮挡镜像层（`lowerdir`）中具有相同名称的文件。

### 修改文件或者目录

Consider some scenarios where files in a container are modified.

考虑在某些情况下修改了容器中的文件。

- **Writing to a file for the first time**: The first time a container writes to an existing file, that file does not exist in the container (`upperdir`). The `overlay`/`overlay2` driver performs a _copy\_up_ operation to copy the file from the image (`lowerdir`) to the container (`upperdir`). The container then writes the changes to the new copy of the file in the container layer.

- **第一次写入文件**：容器第一次写入现有文件时，该文件在容器中不存在（`upperdir`）。`overlay`/`overlay2` 驱动程序执行 `copy_up` 操作，以将文件从镜像（`lowerdir`）复制到容器（`upperdir`）。然后，容器将更改写入容器层中文件的新副本。
  
    However, OverlayFS works at the file level rather than the block level. This means that all OverlayFS copy\_up operations copy the entire file, even if the file is very large and only a small part of it is being modified. This can have a noticeable impact on container write performance. However, two things are worth noting:

    但是，OverlayFS 在文件级别而不是块级别工作。这意味着所有 OverlayFS `copy_up` 操作都将复制整个文件，即使该文件非常大且只有一小部分正在被修改。这会对容器写入性能产生明显影响。其中，有两点值得注意：
    
    - The copy\_up operation only occurs the first time a given file is written to. Subsequent writes to the same file operate against the copy of the file already copied up to the container.

    - 复制操作仅在第一次写入给定文件时发生。随后对同一文件的写入将对已经复制到容器的文件副本进行操作。
      
    - OverlayFS only works with two layers. This means that performance should be better than AUFS, which can suffer noticeable latencies when searching for files in images with many layers. This advantage applies to both `overlay` and `overlay2` drivers. `overlayfs2` is slightly less performant than `overlayfs` on initial read, because it must look through more layers, but it caches the results so this is only a small penalty.

    - OverlayFS 仅有两层。这意味着性能应优于 AUFS，因为 AUFS 在多层图像中搜索文件时会出现明显的延迟。这一优势同时适用于 `overlay` 和 `overlay2` 驱动程序。 `overlayfs2` 在初次读取时的性能比 `overlayfs` 稍差，因为它必须遍历更多的层，但是会缓存结果，因此这只是一个小小的代价。
    
- **Deleting files and directories**:

- **删除文件和目录**：

    - When a file is deleted within a container, a _whiteout_ file is created in the container (`upperdir`). The version of the file in the image layer (`lowerdir`) is not deleted (because the `lowerdir` is read-only). However, the whiteout file prevents it from being available to the container.

    - 在容器中删除文件时，会在容器中创建 _whiteout_ 文件（`upperdir`）。镜像层中的文件版本（`lowerdir`）不会被删除（因为`lowerdir`是只读的）。但是，_whiteout_ 文件会阻止容器使用它。

    - When a _directory_ is deleted within a container, an _opaque directory_ is created within the container (`upperdir`). This works in the same way as a whiteout file and effectively prevents the directory from being accessed, even though it still exists in the image (`lowerdir`).
      
    - 当在容器内删除目录时，会在容器内创建 _opaque directory_（`upperdir`）。这与 whiteout 文件的工作方式相同，并且即使该目录仍然存在于镜像中，也有效地阻止了该目录的访问（“ lowerdir”）。
    
- **Renaming directories**: Calling `rename(2)` for a directory is allowed only when both the source and the destination path are on the top layer. Otherwise, it returns `EXDEV` error (“cross-device link not permitted”). Your application needs to be designed to handle `EXDEV` and fall back to a “copy and unlink” strategy.

- **重命名目录**：仅当源路径和目标路径都位于顶层时，才允许为目录调用 `rename（2）`。否则，它将返回 `EXDEV` 错误（“不允许跨设备链接”）。您的应用程序需要设计为处理 `EXDEV` 并退回到“复制和取消链接”策略。

## OverlayFS and Docker Performance

Both `overlay2` and `overlay` drivers are more performant than `aufs` and `devicemapper`. In certain circumstances, `overlay2` may perform better than `btrfs` as well. However, be aware of the following details.

`overlay2` 和 `overlay` 驱动程序都比 `aufs` 和 `devicemapper` 性能更高。在某些情况下，`overlay2` 的效果也可能会优于 `btrfs`。但是，请注意以下详细信息。

- **Page Caching**. OverlayFS supports page cache sharing. Multiple containers accessing the same file share a single page cache entry for that file. This makes the `overlay` and `overlay2` drivers efficient with memory and a good option for high-density use cases such as PaaS.

- **页面缓存**。 OverlayFS 支持页面缓存共享。访问同一文件的多个容器共享该文件的单个页面缓存条目。这使得 `overlay` 和 `overlay2` 驱动程序可以有效地利用内存，并且是高密度用例（例如PaaS）的不错选择。

- **copy\_up**. As with AUFS, OverlayFS performs copy-up operations whenever a container writes to a file for the first time. This can add latency into the write operation, especially for large files. However, once the file has been copied up, all subsequent writes to that file occur in the upper layer, without the need for further copy-up operations.

- **复制**。与 AUFS 一样，每当容器第一次写入文件时，OverlayFS 都会执行复制操作。这会增加写入操作的延迟，尤其是对于大文件。但是，一旦文件被复制，对该文件的所有后续写入都将在上层进行，而无需进行进一步的复制操作。
  
    The OverlayFS `copy_up` operation is faster than the same operation with AUFS, because AUFS supports more layers than OverlayFS and it is possible to incur far larger latencies if searching through many AUFS layers. `overlay2` supports multiple layers as well, but mitigates any performance hit with caching.

    OverlayFS 的 `copy_up` 操作比 AUFS 的相同操作要快，这是因为 AUFS 支持的图层比 OverlayFS 还要多，并且如果搜索许多 AUFS 图层，可能会产生更大的延迟。`overlay2` 也支持多层，但是可以减轻缓存对性能的影响。
    
- **Inode limits**. Use of the legacy `overlay` storage driver can cause excessive inode consumption. This is especially true in the presence of a large number of images and containers on the Docker host. The only way to increase the number of inodes available to a filesystem is to reformat it. To avoid running into this issue, it is highly recommended that you use `overlay2` if at all possible.

- **Inode 限制**。使用传统的 `overlay` 存储驱动程序可能会导致 inode 消耗过多。在 Docker 主机上存在大量镜像和容器的情况下尤其如此。增加文件系统可用的索引节点数量的唯一方法是对其进行重新格式化。为了避免遇到此问题，强烈建议您尽可能使用 `overlay2`。

### Performance best practices

The following generic performance best practices also apply to OverlayFS.

以下通用性能最佳实践也适用于 OverlayFS。

- **Use fast storage**: Solid-state drives (SSDs) provide faster reads and writes than spinning disks.

- **使用快速存储**：固态驱动器（SSD）提供比旋转磁盘更快的读写速度。
  
- **Use volumes for write-heavy workloads**: Volumes provide the best and most predictable performance for write-heavy workloads. This is because they bypass the storage driver and do not incur any of the potential overheads introduced by thin provisioning and copy-on-write. Volumes have other benefits, such as allowing you to share data among containers and persisting your data even if no running container is using them.
  
- **将数据卷用于繁重的工作负载**：卷可为繁重的工作负载提供最佳和最可预测的性能。 这是因为它们绕过了存储驱动程序，并且不会产生任何精简配置和写时复制所带来的潜在开销。卷还有其他好处，例如，即使没有运行中的容器正在使用它们，它也允许您在容器之间共享数据并保留数据。

## 总结

介绍了 overlay 的实现方式，以及在容器中的如何处理文件的增删改查等。