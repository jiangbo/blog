# 【Docker】Dockerfile 最佳实践-ADD 和 COPY

参考教程：https://docs.docker.com/develop/develop-images/dockerfile_best-practices/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## ADD 和 COPY

Although `ADD` and `COPY` are functionally similar, generally speaking, `COPY` is preferred. That’s because it’s more transparent than `ADD`. `COPY` only supports the basic copying of local files into the container, while `ADD` has some features (like local-only tar extraction and remote URL support) that are not immediately obvious. Consequently, the best use for `ADD` is local tar file auto-extraction into the image, as in `ADD rootfs.tar.xz /`.

尽管 `ADD` and `COPY` 在功能上相似，但通常来说，`COPY` 是首选。那是因为它比 `ADD` 更透明。 `COPY` 仅支持将本地文件基本复制到容器中，而 `ADD` 具有一些功能（例如，仅本地 tar 提取和远程 URL 支持），这些功能并不是立即显而易见的。因此，`ADD` 的最佳用途是将本地 tar 文件自动提取到镜像中，如 `ADD rootfs.tar.xz /` 中所示。

If you have multiple `Dockerfile` steps that use different files from your context, `COPY` them individually, rather than all at once. This ensures that each step’s build cache is only invalidated (forcing the step to be re-run) if the specifically required files change.

如果您有多个 `Dockerfile` 步骤使用与您的上下文不同的文件，请单独复制它们，而不是一次复制所有文件。这样可以确保仅在特别需要的文件发生更改时，才使每个步骤的构建缓存无效（强制重新运行该步骤）。

For example:
例如：

```Dockerfile
COPY requirements.txt /tmp/
RUN pip install --requirement /tmp/requirements.txt
COPY . /tmp/
```

Results in fewer cache invalidations for the `RUN` step, than if you put the `COPY . /tmp/` before it.

与 `COPY . /tmp/` 相比，`RUN` 步骤导致的缓存失效更少。

Because image size matters, using `ADD` to fetch packages from remote URLs is strongly discouraged; you should use `curl` or `wget` instead. That way you can delete the files you no longer need after they’ve been extracted and you don’t have to add another layer in your image. For example, you should avoid doing things like:

由于镜像大小很重要，因此强烈建议不要使用 `ADD` 从远程 URL 获取软件包。您应该改用 `curl` 或 `wget`。这样一来，您可以删除提取后不再需要的文件，而不必在镜像中添加其他图层。例如，您应该避免做以下事情：

```Dockerfile
ADD https://example.com/big.tar.xz /usr/src/things/
RUN tar -xJf /usr/src/things/big.tar.xz -C /usr/src/things
RUN make -C /usr/src/things all
```

And instead, do something like:
相反，请执行以下操作：

```
RUN mkdir -p /usr/src/things \
    && curl -SL https://example.com/big.tar.xz \
    | tar -xJC /usr/src/things \
    && make -C /usr/src/things all
```

For other items (files, directories) that do not require `ADD`’s tar auto-extraction capability, you should always use `COPY`.

对于不需要 `ADD` 的 tar 自动提取功能的其他项目（文件，目录），则应始终使用e `COPY`。

## 总结

介绍了 Dockerfile 的  ADD 和 COPY 指令的最佳实践。