# 【Docker】Dockerfile 之 COPY

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## COPY

COPY has two forms:

COPY 指令有两种格式

```Dockerfile
COPY [--chown=<user>:<group>] <src>... <dest>
COPY [--chown=<user>:<group>] ["<src>",... "<dest>"]
```

This latter form is required for paths containing whitespace

包含空格的路径需要后一种形式

The `COPY` instruction copies new files or directories from `<src>` and adds them to the filesystem of the container at the path `<dest>`.

COPY 指令从 `<src>` 复制新文件或目录，并将它们添加到容器的文件系统中，路径为 `<dest>`。

Multiple `<src>` resources may be specified but the paths of files and directories will be interpreted as relative to the source of the context of the build.

可以指定多个 `<src>` 资源，但是文件和目录的路径将被解释为相对于构建上下文。

Each `<src>` may contain wildcards and matching will be done using Go’s [filepath.Match](http://golang.org/pkg/path/filepath#Match) rules. For example:

每个 `<src>` 可能包含通配符，并且匹配将使用 Go 的 [filepath.Match](http://golang.org/pkg/path/filepath#Match) 规则进行。例如：

To add all files starting with “hom”:

要添加所有以 “hom” 开头的文件：

```Dockerfile
COPY hom* /mydir/
```

In the example below, `?` is replaced with any single character, e.g., “home.txt”.

在下面的示例中，`?` 被替换为任何单个字符，例如 “home.txt”。

```Dockerfile
COPY hom?.txt /mydir/
```

The `<dest>` is an absolute path, or a path relative to `WORKDIR`, into which the source will be copied inside the destination container.

`<dest>` 是绝对路径，或者是相对于 `WORKDIR` 的路径，源将被复制到目标容器内。

The example below uses a relative path, and adds “test.txt” to `<WORKDIR>/relativeDir/`:

下面的示例使用相对路径，并将 “test.txt” 添加到 `<WORKDIR>/relativeDir/`：

```Dockerfile
COPY test.txt relativeDir/
```

Whereas this example uses an absolute path, and adds “test.txt” to `/absoluteDir/`

而此示例使用了绝对路径，并将 “test.txt” 添加到 `/absoluteDir/` 中

```Dockerfile
COPY test.txt /absoluteDir/
```

When copying files or directories that contain special characters (such as `[` and `]`), you need to escape those paths following the Golang rules to prevent them from being treated as a matching pattern. For example, to copy a file named `arr[0].txt`, use the following;

在添加包含特殊字符（例如 `[` 和 `]`）的文件或目录时，您需要按照 Golang 规则转义那些路径，以防止将它们视为匹配模式。例如，要添加名为 `arr[0].txt` 的文件，请使用以下命令；

```Dockerfile
COPY arr[[]0].txt /mydir/
```

> **Note**
> 
> If you build using STDIN (`docker build - < somefile`), there is no build context, so `COPY` can’t be used.

> **注意**
>
>如果您使用 STDIN 进行构建（`docker build - < somefile`），则没有构建上下文，因此无法使用 `COPY`。

Optionally `COPY` accepts a flag `--from=<name>` that can be used to set the source location to a previous build stage (created with `FROM .. AS <name>`) that will be used instead of a build context sent by the user. In case a build stage with a specified name can’t be found an image with the same name is attempted to be used instead.

可选地，`COPY` 接受一个标志 `--from=<name>`，该标志可用于将源位置设置为先前的构建阶段（由 `FROM .. AS <name>` 创建），该标志将代替构建用户发送的上下文。如果找不到具有指定名称的构建阶段，则尝试改用具有相同名称的镜像。

`COPY` obeys the following rules:

`COPY` 遵守以下规则：

- The `<src>` path must be inside the _context_ of the build; you cannot `COPY ../something /something`, because the first step of a `docker build` is to send the context directory (and subdirectories) to the docker daemon.

- `<src>` 路径必须在构建的上下文内部；您不能执行 `COPY ../something /something`，因为 `docker build` 的第一步是将上下文目录（和子目录）发送到 docker 守护进程。

- If `<src>` is a directory, the entire contents of the directory are copied, including filesystem metadata.

-如果 `<src>` 是目录，则复制目录的整个内容，包括文件系统元数据。


> **Note**
> 
> The directory itself is not copied, just its contents.

> **注意**
>
>目录本身不会被复制，只是其内容被复制。

- If `<src>` is any other kind of file, it is copied individually along with its metadata. In this case, if `<dest>` ends with a trailing slash `/`, it will be considered a directory and the contents of `<src>` will be written at `<dest>/base(<src>)`.

- 如果 `<src>` 是任何其他类型的文件，则会将其及其元数据一起单独复制。在这种情况下，如果`<dest>` 以尾斜杠 `/` 结尾，则它将被视为目录，并且 `<src>` 的内容将被写在 `<dest>/base(<src>)` 中。

- If multiple `<src>` resources are specified, either directly or due to the use of a wildcard, then `<dest>` must be a directory, and it must end with a slash `/`.

- 如果直接或由于使用通配符而指定了多个 `<src>` 资源，则 `<dest>` 必须是目录，并且必须以斜杠 `/` 结束。

- If `<dest>` does not end with a trailing slash, it will be considered a regular file and the contents of `<src>` will be written at `<dest>`.

- 如果 `<dest>` 不以斜杠结尾，则将其视为常规文件，并且 `<src>` 的内容将被写入`<dest>`。

- If `<dest>` doesn’t exist, it is created along with all missing directories in its path.

- 如果 `<dest>` 不存在，则会与路径中所有缺少的目录一起创建它。

> **Note**
> 
> The first encountered `COPY` instruction will invalidate the cache for all following instructions from the Dockerfile if the contents of `<src>` have changed. This includes invalidating the cache for `RUN` instructions. See the [`Dockerfile` Best Practices guide – Leverage build cache](https://docs.docker.com/develop/develop-images/dockerfile_best-practices/#leverage-build-cache) for more information.

> **注意**
>
>如果 `<src>` 的内容已更改，则遇到的第一个 `COPY` 指令将使 Dockerfile 中所有以下指令的缓存无效。这包括使 `RUN` 指令的高速缓存无效。有关更多信息，请参见 [Dockerfile 最佳实践指南-利用构建缓存](https://docs.docker.com/develop/develop-images/dockerfile_best-practices/#leverage-build-cache)。

## 示例

### Dockerfile 文件

```Dockerfile
FROM busybox
COPY text2.txt /text2/
```

### 构建结果

```sh
[root@master env]# docker build -t jiangbo:0.0.1 .
Sending build context to Docker daemon  3.584kB
Step 1/2 : FROM busybox
 ---> dc3bacd8b5ea
Step 2/2 : COPY text2.txt /text2/
 ---> 072721a4c4d8
Successfully built 072721a4c4d8
Successfully tagged jiangbo:0.0.1
```

### 查看结果

```sh
[root@master env]# docker run -it jiangbo:0.0.1
/ # ls
bin    dev    etc    home   proc   root   sys    text2  tmp    usr    var
/ # ls t
text2/  tmp/
/ # ls text2/
text2.txt
/ #
```


## 总结

介绍了 Dockerfile 中 COPY 指令的使用。