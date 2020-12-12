# 【Docker】Dockerfile 最佳实践-ONBUILD

参考教程：https://docs.docker.com/develop/develop-images/dockerfile_best-practices/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## ONBUILD

An `ONBUILD` command executes after the current `Dockerfile` build completes. `ONBUILD` executes in any child image derived `FROM` the current image. Think of the `ONBUILD` command as an instruction the parent `Dockerfile` gives to the child `Dockerfile`.

当前的 Dockerfile 构建完成后，将执行 `ONBUILD` 命令。`ONBUILD` 在派生自 `FROM` 的任何子镜像中执行。可以将 `ONBUILD` 命令视为父 Dockerfile 给子 Dockerfile 的指令。

A Docker build executes `ONBUILD` commands before any command in a child `Dockerfile`.

Docker 构建在子 `Dockerfile` 中的任何命令之前执行 `ONBUILD` 命令。

`ONBUILD` is useful for images that are going to be built `FROM` a given image. For example, you would use `ONBUILD` for a language stack image that builds arbitrary user software written in that language within the `Dockerfile`, as you can see in [Ruby’s `ONBUILD` variants](https://github.com/docker-library/ruby/blob/c43fef8a60cea31eb9e7d960a076d633cb62ba8d/2.4/jessie/onbuild/Dockerfile).

`ONBUILD` 对于将要构建的镜像 `FROM` 给定镜像很有用。例如，您可以将 `ONBUILD` 用于语言堆栈映像，以构建在 Dockerfile 中以该语言编写的任意用户软件，如 [Ruby’s `ONBUILD` variants](https://github.com/docker-library/ruby/blob/c43fef8a60cea31eb9e7d960a076d633cb62ba8d/2.4/jessie/onbuild/Dockerfile)。

Images built with `ONBUILD` should get a separate tag, for example: `ruby:1.9-onbuild` or `ruby:2.0-onbuild`.

用 `ONBUILD` 构建的图像应获得一个单独的标签，例如：`ruby:1.9-onbuild` 或 `ruby:2.0-onbuild`。

Be careful when putting `ADD` or `COPY` in `ONBUILD`. The “onbuild” image fails catastrophically if the new build’s context is missing the resource being added. Adding a separate tag, as recommended above, helps mitigate this by allowing the `Dockerfile` author to make a choice.

将 `ADD` 或 `COPY` 放入 `ONBUILD` 时要小心。如果新构建的上下文缺少要添加的资源，则 “onbuild” 映像将灾难性地失败。如上所述，添加一个单独的标签可以通过允许 Dockerfile 作者做出选择来缓解这种情况。

## 总结

介绍了 Dockerfile 的 ONBUILD 指令的最佳实践。