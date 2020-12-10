# 【Docker】Dockerfile 之 ADD

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## ADD

ADD has two forms:

ADD 有两种形式：

```Dockerfile
ADD [--chown=<user>:<group>] <src>... <dest>
ADD [--chown=<user>:<group>] ["<src>",... "<dest>"]
```

The latter form is required for paths containing whitespace.

包含空格的路径需要后一种形式。

> **Note**
> 
> The `--chown` feature is only supported on Dockerfiles used to build Linux containers, and will not work on Windows containers. Since user and group ownership concepts do not translate between Linux and Windows, the use of `/etc/passwd` and `/etc/group` for translating user and group names to IDs restricts this feature to only be viable for Linux OS-based containers.

> **注意**
>
>`--chown` 功能仅在用于构建 Linux 容器的 Dockerfiles 上受支持，而在 Windows 容器上不起作用。由于用户和组所有权概念不会在 Linux 和 Windows 之间转换，因此使用 `/etc/passwd` 和 `/etc/group` 将用户名和组名转换为 ID 限制了该功能仅适用于基于 Linux OS 的操作系统容器。

The `ADD` instruction copies new files, directories or remote file URLs from `<src>` and adds them to the filesystem of the image at the path `<dest>`.

`ADD` 指令从 `<src>` 复制新文件，目录或远程文件 URL，并将它们添加到镜像的文件系统中的 `<dest>` 路径。

Multiple `<src>` resources may be specified but if they are files or directories, their paths are interpreted as relative to the source of the context of the build.

可以指定多个 `<src>` 资源，但是如果它们是文件或目录，则将其路径解释为相对于构建上下文源的路径。

Each `<src>` may contain wildcards and matching will be done using Go’s [filepath.Match](http://golang.org/pkg/path/filepath#Match) rules. For example:

每个 `<src>` 可能包含通配符，并且匹配将使用 Go 的 [filepath.Match](http://golang.org/pkg/path/filepath#Match) 规则进行。例如：

To add all files starting with “hom”:

要添加所有以 “hom” 开头的文件：

```Dockerfile
ADD hom* /mydir/
```

In the example below, `?` is replaced with any single character, e.g., “home.txt”.

在下面的示例中，`?` 被替换为任何单个字符，例如 “home.txt”。

```Dockerfile
ADD hom?.txt /mydir/
```

The `<dest>` is an absolute path, or a path relative to `WORKDIR`, into which the source will be copied inside the destination container.

`<dest>` 是绝对路径，或者是相对于 `WORKDIR` 的路径，源将被复制到目标容器内。

The example below uses a relative path, and adds “test.txt” to `<WORKDIR>/relativeDir/`:

下面的示例使用相对路径，并将 “test.txt” 添加到 `<WORKDIR>/relativeDir/`：

```Dockerfile
ADD test.txt relativeDir/
```

Whereas this example uses an absolute path, and adds “test.txt” to `/absoluteDir/`

而此示例使用了绝对路径，并将 “test.txt” 添加到 `/absoluteDir/` 中

```Dockerfile
ADD test.txt /absoluteDir/
```

When adding files or directories that contain special characters (such as `[` and `]`), you need to escape those paths following the Golang rules to prevent them from being treated as a matching pattern. For example, to add a file named `arr[0].txt`, use the following;

在添加包含特殊字符（例如 `[` 和 `]`）的文件或目录时，您需要按照 Golang 规则转义那些路径，以防止将它们视为匹配模式。例如，要添加名为 `arr[0].txt` 的文件，请使用以下命令；

```Dockerfile
ADD arr[[]0].txt /mydir/
```

> **Note**
> 
> The first encountered `ADD` instruction will invalidate the cache for all following instructions from the Dockerfile if the contents of `<src>` have changed. This includes invalidating the cache for `RUN` instructions. See the [`Dockerfile` Best Practices guide – Leverage build cache](https://docs.docker.com/develop/develop-images/dockerfile_best-practices/#leverage-build-cache) for more information.

> **注意**
>
>如果 `<src>` 的内容已更改，则遇到的第一个 `ADD` 指令将使 Dockerfile 中所有以下指令的缓存无效。这包括使 `RUN` 指令的高速缓存无效。有关更多信息，请参见 [Dockerfile 最佳实践指南-利用构建缓存](https://docs.docker.com/develop/develop-images/dockerfile_best-practices/#leverage-build-cache)。

`ADD` obeys the following rules:

`ADD` 遵守以下规则：

- The `<src>` path must be inside the _context_ of the build; you cannot `ADD ../something /something`, because the first step of a `docker build` is to send the context directory (and subdirectories) to the docker daemon.

- `<src>` 路径必须在构建的 _context_ 内部；您不能添加 `ADD ../something /something`，因为 `docker build` 的第一步是将上下文目录（和子目录）发送到 docker 守护进程。

- If `<src>` is a URL and `<dest>` does not end with a trailing slash, then a file is downloaded from the URL and copied to `<dest>`.

- 如果 `<src>` 是 URL，而 `<dest>` 不以斜杠结尾，则从 URL 下载文件并将其复制到 `<dest>`。

- If `<src>` is a URL and `<dest>` does end with a trailing slash, then the filename is inferred from the URL and the file is downloaded to `<dest>/<filename>`. For instance, `ADD http://example.com/foobar /` would create the file `/foobar`. The URL must have a nontrivial path so that an appropriate filename can be discovered in this case (`http://example.com` will not work).

- 如果 `<src>` 是 URL，而 `<dest>` 以斜杠结尾，则从 URL 推断文件名，并将文件下载到 `<dest>/<filename>`。例如，`ADD http://example.com/foobar /`将创建文件 `/foobar`。该 URL 必须具有非平凡的路径，以便在这种情况下可以找到适当的文件名（`http://example.com` 将不起作用）。

- If `<src>` is a directory, the entire contents of the directory are copied, including filesystem metadata.

- 如果 `<src>` 是目录，则复制目录的整个内容，包括文件系统元数据。

> **Note**
> 
> The directory itself is not copied, just its contents.

> **注意**
>
> 目录本身不会被复制，只是其内容被复制。

- If `<src>` is a _local_ tar archive in a recognized compression format (identity, gzip, bzip2 or xz) then it is unpacked as a directory. Resources from _remote_ URLs are **not** decompressed. When a directory is copied or unpacked, it has the same behavior as `tar -x`, the result is the union of:

- 如果 `<src>` 是本地的压缩格式（identity，gzip，bzip2 或 xz）存档，则将其解压缩为目录。来自远程 URL 的资源不被解压缩。复制或解压缩目录时，其行为与 `tar -x` 相同。

> **Note**
> 
> Whether a file is identified as a recognized compression format or not is done solely based on the contents of the file, not the name of the file. For example, if an empty file happens to end with `.tar.gz` this will not be recognized as a compressed file and **will not** generate any kind of decompression error message, rather the file will simply be copied to the destination.

> **注意**
>
>是否将文件识别为可识别的压缩格式，仅根据文件的内容而不是文件的名称来完成。例如，如果一个空文件碰巧以 `.tar.gz` 结尾，则该文件将不会被识别为压缩文件，并且**不会**产生任何类型的解压缩错误消息，而是将文件简单地复制到目的地。
    
- If `<src>` is any other kind of file, it is copied individually along with its metadata. In this case, if `<dest>` ends with a trailing slash `/`, it will be considered a directory and the contents of `<src>` will be written at `<dest>/base(<src>)`.

- 如果 `<src>` 是任何其他类型的文件，则会将其及其元数据一起单独复制。在这种情况下，如果`<dest>` 以尾斜杠 `/` 结尾，则它将被视为目录，并且 `<src>` 的内容将被写在 `<dest>/base(<src>)` 中。

- If multiple `<src>` resources are specified, either directly or due to the use of a wildcard, then `<dest>` must be a directory, and it must end with a slash `/`.

- 如果直接或由于使用通配符而指定了多个 `<src>` 资源，则 `<dest>` 必须是目录，并且必须以斜杠 `/` 结束。

- If `<dest>` does not end with a trailing slash, it will be considered a regular file and the contents of `<src>` will be written at `<dest>`.

- 如果 `<dest>` 不以斜杠结尾，则将其视为常规文件，并且 `<src>` 的内容将被写入`<dest>`。

- If `<dest>` doesn’t exist, it is created along with all missing directories in its path.

- 如果 `<dest>` 不存在，则会与路径中所有缺少的目录一起创建它。

## 复制到文件

### Dockerfile 文件

```Dockerfile
FROM busybox
ADD text2.txt /text2.txt
```

### 构建结果

```sh
[root@master env]# docker build -t jiangbo:0.0.1 .
Sending build context to Docker daemon  3.584kB
Step 1/2 : FROM busybox
 ---> dc3bacd8b5ea
Step 2/2 : ADD text2.txt /text2.txt
 ---> 6501eb85018e
Successfully built 6501eb85018e
Successfully tagged jiangbo:0.0.1
```

### 查看结果

```sh
[root@master env]# docker run -it jiangbo:0.0.1
/ # ls
bin        dev        etc        home       proc       root       sys        text2.txt  tmp        usr        var
/ # exit
```

## 复制到目录

### Dockerfile 文件

```Dockerfile
FROM busybox
ADD text2.txt /text2/
```

### 构建结果

```sh
[root@master env]# docker build -t jiangbo:0.0.1 .
Sending build context to Docker daemon  3.584kB
Step 1/2 : FROM busybox
 ---> dc3bacd8b5ea
Step 2/2 : ADD text2.txt /text2/
 ---> 84549e9f56c8
Successfully built 84549e9f56c8
Successfully tagged jiangbo:0.0.1
```

### 查看结果

```sh
[root@master env]# docker run -it jiangbo:0.0.1
/ # ls
bin    dev    etc    home   proc   root   sys    text2  tmp    usr    var
/ # ls text2/
text2.txt
/ #
```

## 总结

介绍了 Dockerfile 中 ADD 指令的使用。