# 【Docker】关于存储驱动

参考教程：https://docs.docker.com/storage/storagedriver/
以下内容来自官方文档翻译

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

To use storage drivers effectively, it’s important to know how Docker builds and stores images, and how these images are used by containers. You can use this information to make informed choices about the best way to persist data from your applications and avoid performance problems along the way.

为了高效地使用存储驱动，了解 Docker 如何生成和存储镜像以及容器如何使用这些镜像非常重要。您可以使用此信息来做出明智的选择，并以最佳方式存储应用程序中的数据，避免使用过程中的性能问题。

Storage drivers allow you to create data in the writable layer of your container. The files won’t be persisted after the container is deleted, and both read and write speeds are lower than native file system performance.

存储驱动允许您在容器的可写层中创建数据。删除容器后，文件不会保留，读取和写入速度都低于本机文件系统性能。

> Note: Operations that are known to be problematic include write-intensive database storage, particularly when pre-existing data exists in the read-only layer. More details are provided in this document.

> 注意：已知有问题的操作包括密集型写入数据库存储，特别是当预先存在的数据存在于只读层中时。本文档提供了更多详细信息。

## 镜像和层

A Docker image is built up from a series of layers. Each layer represents an instruction in the image’s Dockerfile. Each layer except the very last one is read-only. Consider the following Dockerfile:

Docker 镜像由一系列图层构建。每一层表示镜像的 Dockerfile 中的指令。除了最后一个图层之外，每个图层都是只读的。请考虑以下 Dockerfile：

```dockerfile
FROM ubuntu:18.04
COPY . /app
RUN make /app
CMD python /app/app.py
```

This Dockerfile contains four commands, each of which creates a layer. The FROM statement starts out by creating a layer from the ubuntu:18.04 image. The COPY command adds some files from your Docker client’s current directory. The RUN command builds your application using the make command. Finally, the last layer specifies what command to run within the container.

此 Dockerfile 包含四个命令，每个命令创建一个图层。FROM 语句根据镜像 ubuntu:18.04 创建一层。COPY 命令从 Docker 客户端的当前目录中添加一些文件。RUN 命令使用 make 命令生成应用程序。最后，最后一层指定要在容器中运行的命令。

Each layer is only a set of differences from the layer before it. The layers are stacked on top of each other. When you create a new container, you add a new writable layer on top of the underlying layers. This layer is often called the “container layer”. All changes made to the running container, such as writing new files, modifying existing files, and deleting files, are written to this thin writable container layer. The diagram below shows a container based on the Ubuntu 15.04 image.

每一层只是与上一层不同的集合。这些层彼此堆叠。创建新容器时，在基础层之上添加新的可写层。该层通常称为“容器层”。对运行中的容器所做的所有更改（例如写入新文件，修改现有文件和删除文件）都将写入很薄的可写容器层。下图显示了基于 Ubuntu 15.04 镜像的容器。

![container-layers][1]

A storage driver handles the details about the way these layers interact with each other. Different storage drivers are available, which have advantages and disadvantages in different situations.

存储驱动处理有关这些层相互交互的方式的详细信息。Docker 提供了不同的存储驱动，它们在不同情况下各有利弊。

## 容器和层

The major difference between a container and an image is the top writable layer. All writes to the container that add new or modify existing data are stored in this writable layer. When the container is deleted, the writable layer is also deleted. The underlying image remains unchanged.

容器和镜像之间的主要区别是可写顶层。在容器中添加新数据或修改现有数据的所有写操作都存储在此可写层中。删除容器后，可写层也会被删除。基础镜像保持不变。

Because each container has its own writable container layer, and all changes are stored in this container layer, multiple containers can share access to the same underlying image and yet have their own data state. The diagram below shows multiple containers sharing the same Ubuntu 15.04 image.

因为每个容器都有其自己的可写容器层，并且所有更改都存储在该容器层中，所以多个容器可以共享对同一基础镜像的访问，但具有自己的数据状态。下图显示了共享同一 Ubuntu 15.04 镜像的多个容器。

![sharing-layers][2]

> **Note**: If you need multiple images to have shared access to the exact same data, store this data in a Docker volume and mount it into your containers.

> 注意：如果您需要多个镜像来共享对完全相同的数据的访问权限，请将该数据存储在 Docker 数据卷中并将其挂载到您的容器中。

Docker uses storage drivers to manage the contents of the image layers and the writable container layer. Each storage driver handles the implementation differently, but all drivers use stackable image layers and the copy-on-write (CoW) strategy.

Docker 使用存储驱动来管理镜像层和可写容器层的内容。每个存储驱动对实现的处理方式不同，但是所有驱动都使用可堆叠的镜像层和写时复制（CoW）策略。

## 容器在磁盘上的大小

To view the approximate size of a running container, you can use the `docker ps -s` command. Two different columns relate to size.

要查看正在运行的容器的大致大小，可以使用 `docker ps -s` 命令。有两个不同的列与大小有关。

- `size`: the amount of data (on disk) that is used for the writable layer of each container.

- `size`：每个容器的可写层使用的数据量（磁盘上）。

- `virtual size`: the amount of data used for the read-only image data used by the container plus the container’s writable layer `size`. Multiple containers may share some or all read-only image data. Two containers started from the same image share 100% of the read-only data, while two containers with different images which have layers in common share those common layers. Therefore, you can’t just total the virtual sizes. This over-estimates the total disk usage by a potentially non-trivial amount.

- `virtual size`：容器使用的用于只读镜像数据的数据量加上容器的可写层大小。多个容器可以共享部分或全部只读镜像数据。从同一镜像启动的两个容器共享 100％ 的只读数据，而具有不同镜像的两个容器（具有相同的层）则共享这些公共层。因此，您不能仅仅计算虚拟大小。这高估了总磁盘使用量，可能是一笔不小的数量。

The total disk space used by all of the running containers on disk is some combination of each container’s `size` and the `virtual size` values. If multiple containers started from the same exact image, the total size on disk for these containers would be SUM (`size` of containers) plus one image size (`virtual size`\- `size`).

磁盘上所有正在运行的容器使用的磁盘总空间是每个容器的大小和虚拟大小值的某种组合。如果多个容器从相同的精确镜像启动，则这些容器在磁盘上的总大小将为SUM（容器大小）加上一个镜像大小（虚拟大小－大小）。

```sh
[root@master ~]# docker ps -s
CONTAINER ID        IMAGE               COMMAND                  CREATED             STATUS              PORTS                   NAMES               SIZE
5d1021ee8b24        nginx:alpine        "/docker-entrypoint.…"   2 weeks ago         Up 3 seconds        0.0.0.0:8080->80/tcp    goofy_mendel        1.12kB (virtual 21.8MB)
497d0139b307        nginx:alpine        "/docker-entrypoint.…"   3 weeks ago         Up 4 minutes        0.0.0.0:32768->80/tcp   web                 1.12kB (virtual 21.8MB)
```

总的大小：total = 1.12kB * 2 + 21.8MB - 1.12KB

This also does not count the following additional ways a container can take up disk space:

这不包括容器可以占用磁盘空间的以下其他方式：

- Disk space used for log files if you use the `json-file` logging driver. This can be non-trivial if your container generates a large amount of logging data and log rotation is not configured.
- Volumes and bind mounts used by the container.
- Disk space used for the container’s configuration files, which are typically small.
- Memory written to disk (if swapping is enabled).
- Checkpoints, if you’re using the experimental checkpoint/restore feature.

- 如果使用 json 文件日志记录驱动，则用于日志文件的磁盘空间。如果您的容器生成大量的日志数据并且未配置日志滚动，那么这可能有较大影响。
- 容器使用的卷和绑定挂载。
- 容器的配置文件所用的磁盘空间，通常较小。
- 内存写入磁盘（如果启用了交换）。
- 检查点（如果您正在使用实验性检查点/恢复功能）。

## 写时复制（CoW）策略

Copy-on-write is a strategy of sharing and copying files for maximum efficiency. If a file or directory exists in a lower layer within the image, and another layer (including the writable layer) needs read access to it, it just uses the existing file. The first time another layer needs to modify the file (when building the image or running the container), the file is copied into that layer and modified. This minimizes I/O and the size of each of the subsequent layers. These advantages are explained in more depth below.

写入时复制是一种共享和复制文件的策略，可最大程度地提高效率。 如果文件或目录位于镜像的较低层中，而另一层（包括可写层）需要对其进行读取访问，则它仅使用现有文件。另一层第一次需要修改文件时（在构建镜像或运行容器时），文件被复制到该层并进行修改。 这样可以将 I/O 和每个后续层的大小最小化。这些优点将在下面更深入地说明。

### 共享减小镜像

When you use `docker pull` to pull down an image from a repository, or when you create a container from an image that does not yet exist locally, each layer is pulled down separately, and stored in Docker’s local storage area, which is usually `/var/lib/docker/` on Linux hosts. You can see these layers being pulled in this example:

当您使用 `docker pull` 来从存储库中拉取镜像时，或者当您从本地尚不存在的镜像中创建容器时，每一层都会被分别拉取，并存储在 Docker 的本地存储区域中，通常在 Linux 主机上的 `/var/lib/docker/`。 您可以在此示例中看到这些层被拉取：

```sh
[root@master ~]# docker pull ubuntu:18.04
18.04: Pulling from library/ubuntu
171857c49d0f: Pull complete
419640447d26: Pull complete
61e52f862619: Pull complete
Digest: sha256:646942475da61b4ce9cc5b3fadb42642ea90e5d0de46111458e100ff2c7031e6
Status: Downloaded newer image for ubuntu:18.04
docker.io/library/ubuntu:18.04
[root@master ~]#
```

Each of these layers is stored in its own directory inside the Docker host’s local storage area. To examine the layers on the filesystem, list the contents of `/var/lib/docker/<storage-driver>`. This example uses the `overlay2` storage driver:

所有这些层都存储在 Docker 主机本地存储区域内的自己的目录中。要检查文件系统上的各层，请列出`/var/lib/docker/<storage-driver>`的内容。 此示例使用`overlay2`存储驱动程序：

```sh
[root@master ~]#  ls /var/lib/docker/overlay2
07e2e128dd90a580133c0a3d01dd5cd9a160ece95c984b573b7b7758e0087a89       6f64ae41f33cfd00f26912f8b0c6f9c2a9008dfb0f2339e2a2c7246a056265e6
0b4382e9fba66c5a9b929af53dd33bab06b5ca5176acbba99f02fa15e23bb3d5       737c8cd71c0f7ead61038d43fd211760ae0e0d923587ebf913a6ce55925b9062
0dc6887647a656e49928e0c46291aff65f6c6d4900da139e6a3dce850493fed2       7aa8c39a5eea25bd9ac0950739f2e2d3e2c51bd5451941fbc525201ede4f4eb7
119250987b77acf7e4457080cdad03d3e4c96988b3cc952366d4320a44fb1f8e       8f49a8351ade76bcd02a5cd43833da1ce729ddc9a88ed593fcc8eacf1d9b530f
232fd1b5e48a094fbe94092ac8490ce3edc9afe9e9f91c1018b865b8ecf158dc       9457afb88f8b5f78cd1fc7c63ce8a81bfb4c37ac728ded3a788c87fd9badbd1a
250eb9dd5f52a851b2bf18aa3807acf4c94391c7ed86d9825d57c0fe52a469bd       a9c71bbbbf73132eeb66999122da1ab3fd1579cf128a8c5d517a67ff68e9d21f
2b31f3992eef94b3c3ea540f1df2753e2ee279c5a9b0caaa6195199b0ea0c5dc       backingFsBlockDev
347ea1b6dd30ce456cf1f99fdd93954bdbf3fcbe22337894d0016beaaa6cea37       c7ebdb0ed63bffac4e3a57212e5995e28c52961ac134c43008ef0909382122df
36fadfc8166fbf2316da5ec7efc76f94505fa9974be858b370ae176a03f9d3f5       cb9877985988bc257bb64a96d220ec09b90ed5ea9e5e8d32dabbbe3a6952b669
48729098b14376a78a821e052e2f4c8261ce26b4ae7e2ebad2e723118ca64234       daa81cc2507e2878a838cfb8ec0cc9c54c46cf08d1f791565c91d3ca5bd0a92a
4aa0e717b19de7679450c65d8f7ef23b2d7116c81ca3a5fb89eafb9abcc4039e       e2eddfed55a6bc8663ce4878376dd039607f61b99efedf8745f7d2b33a1e7112
51b36c1db9f9c2d0158da34e3397d783d6d23166a96c1a03c07e02606e544273       e3c64c744ea35a5079dd0c08058c5f26ac90c0661811be8ecb64f857424120d2
51f87163f04d9f5f50a394872918491bfbe03841c5ce35ac0d8e56931cef47b4       eacfdaab2b8295f696e9c1fc0eb99bc488a1261d82a231e3f51d529a7cd6cdf3
5238cfaa817004531c6b52aca024bc141fd8a642e862fc3cd6e96538b6eb9e04       ec0e3ae382bac3b09c95d9d362d84849ca3ea70fd49f4b55884cc0976101d098
5a9a2dcb90abc50c286c0e4eac8d75c37267b3459af079ccf35a08989acae96c       f343ddba472c672753911e20b1f12b04eba797e32210988f84c3204d7218c2f0
5a9a2dcb90abc50c286c0e4eac8d75c37267b3459af079ccf35a08989acae96c-init  f343ddba472c672753911e20b1f12b04eba797e32210988f84c3204d7218c2f0-init
6d1ffd3b06a9b0a36d791dba90bb7f4d9885742163e6ae4ba45d35fba0216d27       f45ba8e0f94a2b16bd7eadf3110d4365392301bcaa9ca8fd38624803c8378982
6d1ffd3b06a9b0a36d791dba90bb7f4d9885742163e6ae4ba45d35fba0216d27-init  f45ba8e0f94a2b16bd7eadf3110d4365392301bcaa9ca8fd38624803c8378982-init
6ebbaef61ac09511197ab4d1cf6159ba63f9aff2850b74fe7d3c1a1587f27d11       fb5cd37f0436f42ca1fb23e7cea2f11286241bb1311a367e6a2e54df0973728c
6f5e73a855f5517ce47ccd43a26da54ff25f899d9d0c64f7792197aeaa78b052       l
[root@master ~]#
```

The directory names do not correspond to the layer IDs (this has been true since Docker 1.10).

目录名称与层ID不对应（自 Docker 1.10 开始就是如此）。

Now imagine that you have two different Dockerfiles. You use the first one to create an image called `acme/my-base-image:1.0`.

现在，假设您有两个不同的 Dockerfile。您使用第一个创建一个名为 `acme/my-base-image:1.0` 的镜像。

```Dockerfile
FROM ubuntu:18.04
COPY . /app
```

The second one is based on `acme/my-base-image:1.0`, but has some additional layers:

第二个基于`acme/my-base-image：1.0`，但有一些额外层：

```Dockerfile
FROM acme/my-base-image:1.0
CMD /app/hello.sh
```

The second image contains all the layers from the first image, plus a new layer with the `CMD` instruction, and a read-write container layer. Docker already has all the layers from the first image, so it does not need to pull them again. The two images share any layers they have in common.

第二个镜像包含第一个镜像中的所有层，再加上带有“CMD”指令的新层，以及一个可读写容器层。Docker 已经具有第一个镜像中的所有层，因此不需要再次将其拉取。这两个镜像共享它们共有的任何图层。

If you build images from the two Dockerfiles, you can use `docker image ls` and `docker history` commands to verify that the cryptographic IDs of the shared layers are the same.

如果您从两个Dockerfile中构建镜像，则可以使用 `docker image ls` 和 `docker history` 命令来验证共享层的 ID 是否相同。

```sh
[root@master ~]# mkdir cow-test
[root@master ~]# ls
anaconda-ks.cfg  cow-test  docker  html  nvm  software
[root@master ~]# cd cow-test/
[root@master cow-test]# vim hello.sh
[root@master cow-test]# cat hello.sh
#!/bin/sh
echo "Hello world"
[root@master cow-test]# chmod +x hello.sh
[root@master cow-test]# ls
hello.sh
[root@master cow-test]# vim Dockerfile.base
[root@master cow-test]# vim Dockerfile
[root@master cow-test]# docker build -t acme/my-base-image:1.0 -f Dockerfile.base .
Sending build context to Docker daemon  4.096kB
Step 1/2 : FROM ubuntu:18.04
 ---> 56def654ec22
Step 2/2 : COPY . /app
 ---> 72ee914ed695
Successfully built 72ee914ed695
Successfully tagged acme/my-base-image:1.0
[root@master cow-test]# docker build -t acme/my-final-image:1.0 -f Dockerfile .
Sending build context to Docker daemon  4.096kB
Step 1/2 : FROM acme/my-base-image:1.0
 ---> 72ee914ed695
Step 2/2 : CMD /app/hello.sh
 ---> Running in 36111dcae4b4
Removing intermediate container 36111dcae4b4
 ---> 0f8b98bb868d
Successfully built 0f8b98bb868d
Successfully tagged acme/my-final-image:1.0
[root@master cow-test]# docker image ls
REPOSITORY            TAG                 IMAGE ID            CREATED             SIZE
acme/my-final-image   1.0                 0f8b98bb868d        24 seconds ago      63.2MB
acme/my-base-image    1.0                 72ee914ed695        55 seconds ago      63.2MB
```

查看构建历史：

```sh
[root@master cow-test]# docker history 72ee914ed695
IMAGE               CREATED             CREATED BY                                      SIZE                COMMENT
72ee914ed695        4 minutes ago       /bin/sh -c #(nop) COPY dir:b5be11a9f1ec80f6d…   107B
56def654ec22        8 weeks ago         /bin/sh -c #(nop)  CMD ["/bin/bash"]            0B
<missing>           8 weeks ago         /bin/sh -c mkdir -p /run/systemd && echo 'do…   7B
<missing>           8 weeks ago         /bin/sh -c [ -z "$(apt-get indextargets)" ]     0B
<missing>           8 weeks ago         /bin/sh -c set -xe   && echo '#!/bin/sh' > /…   745B
<missing>           8 weeks ago         /bin/sh -c #(nop) ADD file:4974bb5483c392fb5…   63.2MB
[root@master cow-test]# docker history 0f8b98bb868d
IMAGE               CREATED             CREATED BY                                      SIZE                COMMENT
0f8b98bb868d        3 minutes ago       /bin/sh -c #(nop)  CMD ["/bin/sh" "-c" "/app…   0B
72ee914ed695        4 minutes ago       /bin/sh -c #(nop) COPY dir:b5be11a9f1ec80f6d…   107B
56def654ec22        8 weeks ago         /bin/sh -c #(nop)  CMD ["/bin/bash"]            0B
<missing>           8 weeks ago         /bin/sh -c mkdir -p /run/systemd && echo 'do…   7B
<missing>           8 weeks ago         /bin/sh -c [ -z "$(apt-get indextargets)" ]     0B
<missing>           8 weeks ago         /bin/sh -c set -xe   && echo '#!/bin/sh' > /…   745B
<missing>           8 weeks ago         /bin/sh -c #(nop) ADD file:4974bb5483c392fb5…   63.2MB
```

Notice that all the layers are identical except the top layer of the second image. All the other layers are shared between the two images, and are only stored once in `/var/lib/docker/`. The new layer actually doesn’t take any room at all, because it is not changing any files, but only running a command.

请注意，除了第二个镜像的顶层之外，所有层都是相同的。所有其他层在两个镜像之间共享，并且仅在 `/var/lib/docker/` 中存储一次。实际上，新层根本没有占用任何空间，因为它不会更改任何文件，而只是运行命令。

> **Note**: The `<missing>` lines in the `docker history` output indicate that those layers were built on another system and are not available locally. This can be ignored.

> **注意**：“ docker history”输出中的“ <missing>”行表示这些层是在另一个系统上构建的，并且在本地不可用。这可以忽略。

### 复制使容器高效

When you start a container, a thin writable container layer is added on top of the other layers. Any changes the container makes to the filesystem are stored here. Any files the container does not change do not get copied to this writable layer. This means that the writable layer is as small as possible.

启动容器时，将在其他层之上添加一个薄的可写容器层。容器对文件系统所做的任何更改都存储在此处。容器未更改的任何文件都不会复制到此可写层。这意味着可写层尽可能小。

When an existing file in a container is modified, the storage driver performs a copy-on-write operation. The specifics steps involved depend on the specific storage driver. For the `aufs`, `overlay`, and `overlay2` drivers, the copy-on-write operation follows this rough sequence:

当修改容器中的现有文件时，存储驱动将执行写时复制操作。涉及的具体步骤取决于特定的存储驱动。对于 `aufs`，`overlay`和 `overlay2` 驱动程序，写时复制操作遵循以下大致顺序：

- Search through the image layers for the file to update. The process starts at the newest layer and works down to the base layer one layer at a time. When results are found, they are added to a cache to speed future operations.

- 在镜像图层中搜索要更新的文件。该过程从最新层开始，一次向下一层到基础层。找到结果后，会将它们添加到缓存中以加快将来的操作。

- Perform a `copy_up` operation on the first copy of the file that is found, to copy the file to the container’s writable layer.

- 在找到的第一个文件上执行 `copy_up` 操作，以将文件复制到容器的可写层。

- Any modifications are made to this copy of the file, and the container cannot see the read-only copy of the file that exists in the lower layer.

- 只要对该文件的副本进行了任何修改，容器就看不到存在于较低层中的文件的只读副本。

Btrfs, ZFS, and other drivers handle the copy-on-write differently. You can read more about the methods of these drivers later in their detailed descriptions.

Btrfs，ZFS 和其他驱动程序以不同方式处理写时复制。您可以在稍后的详细说明中阅读有关这些驱动程序方法的更多信息。

Containers that write a lot of data consume more space than containers that do not. This is because most write operations consume new space in the container’s thin writable top layer.

写入大量数据的容器比不写入数据的容器消耗更多的空间。这是因为大多数写操作会占用容器的薄可写顶层中的新空间。

> **Note**: for write-heavy applications, you should not store the data in the container. Instead, use Docker volumes, which are independent of the running container and are designed to be efficient for I/O. In addition, volumes can be shared among containers and do not increase the size of your container’s writable layer.

> **注意**：对于大量写应用程序，您不应将数据存储在容器中。取而代之的是使用 Docker 数据卷，它们独立于正在运行的容器，并且旨在提高 I/O 效率。此外，卷可以在容器之间共享，而不会增加容器可写层的大小。

A `copy_up` operation can incur a noticeable performance overhead. This overhead is different depending on which storage driver is in use. Large files, lots of layers, and deep directory trees can make the impact more noticeable. This is mitigated by the fact that each `copy_up` operation only occurs the first time a given file is modified.

`copy_up` 操作可能会导致明显的性能开销。该开销因所使用的存储驱动程序而异。大文件，许多层和深层目录树可以使影响更加明显。每个 `copy_up` 操作仅在第一次修改给定文件时才发生，这可以缓解这种情况。

To verify the way that copy-on-write works, the following procedures spins up 5 containers based on the `acme/my-final-image:1.0` image we built earlier and examines how much room they take up.

为了验证写时复制的工作方式，以下过程基于我们之前构建的 `acme/my-final-image:1.0` 镜像启动了5个容器，并检查了它们占用了多少空间。

```sh
[root@master ~]# docker run -dit --name my_container_1 acme/my-final-image:1.0 bash \
>   && docker run -dit --name my_container_2 acme/my-final-image:1.0 bash \
>   && docker run -dit --name my_container_3 acme/my-final-image:1.0 bash \
>   && docker run -dit --name my_container_4 acme/my-final-image:1.0 bash \
>   && docker run -dit --name my_container_5 acme/my-final-image:1.0 bash
e588c4c170c87bf410352c3bbd604a1657cffd7cc6b7f6d288f4055a437b89fe
c1a4c1a2dbf6f5f3b0a408f34cf388805f1760ab069a1feb95ad029bf8e69adf
29db4486f4a4d6680f979db6d490427bf5bf3c9121df7157da1c162debc3ec45
7fbc5d8b34d5be94d3ff0877dff61261160f71ad92e81d7e2f96583496d68afd
fff65aa3ae7b6446da0dfea988aeceb108092e174a0efd024034ad096ba26c99
[root@master ~]# docker ps
CONTAINER ID        IMAGE                     COMMAND                  CREATED             STATUS              PORTS                   NAMES
fff65aa3ae7b        acme/my-final-image:1.0   "bash"                   15 seconds ago      Up 14 seconds                               my_container_5
7fbc5d8b34d5        acme/my-final-image:1.0   "bash"                   15 seconds ago      Up 14 seconds                               my_container_4
29db4486f4a4        acme/my-final-image:1.0   "bash"                   16 seconds ago      Up 15 seconds                               my_container_3
c1a4c1a2dbf6        acme/my-final-image:1.0   "bash"                   16 seconds ago      Up 15 seconds                               my_container_2
e588c4c170c8        acme/my-final-image:1.0   "bash"                   16 seconds ago      Up 15 seconds                               my_container_1
5d1021ee8b24        nginx:alpine              "/docker-entrypoint.…"   2 weeks ago         Up 54 minutes       0.0.0.0:8080->80/tcp    goofy_mendel
497d0139b307        nginx:alpine              "/docker-entrypoint.…"   3 weeks ago         Up 58 minutes       0.0.0.0:32768->80/tcp   web
[root@master ~]# ls /var/lib/docker/containers
0a33d1388325e5dbcd0f051dbb25a82c6433b8d0c9844067e8a89021bc8c9d46  c1a4c1a2dbf6f5f3b0a408f34cf388805f1760ab069a1feb95ad029bf8e69adf
29db4486f4a4d6680f979db6d490427bf5bf3c9121df7157da1c162debc3ec45  e588c4c170c87bf410352c3bbd604a1657cffd7cc6b7f6d288f4055a437b89fe
497d0139b30750b8ce9c87df8802f48ead5dc5f1e18f29e8e52a18af8d8c8cf2  ed91cb09178c4d5bac13b93480dfa8006295dfceffa6e680cf19c5ea9f747115
5d1021ee8b2493f282512448bfebb33449422c6d7af0a231add658601658127c  fff65aa3ae7b6446da0dfea988aeceb108092e174a0efd024034ad096ba26c99
7fbc5d8b34d5be94d3ff0877dff61261160f71ad92e81d7e2f96583496d68afd
[root@master ~]# du -sh /var/lib/docker/containers/*
28K     /var/lib/docker/containers/0a33d1388325e5dbcd0f051dbb25a82c6433b8d0c9844067e8a89021bc8c9d46
24K     /var/lib/docker/containers/29db4486f4a4d6680f979db6d490427bf5bf3c9121df7157da1c162debc3ec45
28K     /var/lib/docker/containers/497d0139b30750b8ce9c87df8802f48ead5dc5f1e18f29e8e52a18af8d8c8cf2
28K     /var/lib/docker/containers/5d1021ee8b2493f282512448bfebb33449422c6d7af0a231add658601658127c
24K     /var/lib/docker/containers/7fbc5d8b34d5be94d3ff0877dff61261160f71ad92e81d7e2f96583496d68afd
24K     /var/lib/docker/containers/c1a4c1a2dbf6f5f3b0a408f34cf388805f1760ab069a1feb95ad029bf8e69adf
24K     /var/lib/docker/containers/e588c4c170c87bf410352c3bbd604a1657cffd7cc6b7f6d288f4055a437b89fe
28K     /var/lib/docker/containers/ed91cb09178c4d5bac13b93480dfa8006295dfceffa6e680cf19c5ea9f747115
24K     /var/lib/docker/containers/fff65aa3ae7b6446da0dfea988aeceb108092e174a0efd024034ad096ba26c99
[root@master ~]#
```

每个容器占用 24k 的大小。

Not only does copy-on-write save space, but it also reduces start-up time. When you start a container (or multiple containers from the same image), Docker only needs to create the thin writable container layer.

写时复制不仅可以节省空间，还可以缩短启动时间。当启动一个容器（或同一镜像中的多个容器）时，Docker 只需要创建可写的薄容器层。

If Docker had to make an entire copy of the underlying image stack each time it started a new container, container start times and disk space used would be significantly increased. This would be similar to the way that virtual machines work, with one or more virtual disks per virtual machine.

如果 Docker 每次启动新容器都必须制作基础镜像堆栈的完整副本，则容器启动时间和使用的磁盘空间将大大增加。这将类似于虚拟机的工作方式，每个虚拟机具有一个或多个虚拟磁盘。

## 总结

介绍了存储驱动，Docker 镜像，容器和层，所占空间的大小，以及写时复制技术。

[1]: images/container-layers.jpg
[2]: images/sharing-layers.jpg