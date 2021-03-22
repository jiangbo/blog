# 【Docker】Dockerfile 之 ARG（二）

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## ARG

### 使用 ARG 变量

You can use an `ARG` or an `ENV` instruction to specify variables that are available to the `RUN` instruction. Environment variables defined using the `ENV` instruction always override an `ARG` instruction of the same name. Consider this Dockerfile with an `ENV` and `ARG` instruction.

您可以使用 `ARG` 或 `ENV` 指令来指定可用于 `RUN` 指令的变量。使用 `ENV` 指令定义的环境变量始终会覆盖同名的 `ARG` 指令。考虑这个带有 `ENV` 和 `ARG` 指令的 Dockerfile。

```Dockerfile
FROM ubuntu
ARG CONT_IMG_VER
ENV CONT_IMG_VER=v1.0.0
RUN echo $CONT_IMG_VER
```

Then, assume this image is built with this command:
然后，假定此镜像是使用以下命令构建的：

```sh
$ docker build --build-arg CONT_IMG_VER=v2.0.1 .
```

In this case, the `RUN` instruction uses `v1.0.0` instead of the `ARG` setting passed by the user:`v2.0.1` This behavior is similar to a shell script where a locally scoped variable overrides the variables passed as arguments or inherited from environment, from its point of definition.

在这种情况下，`RUN` 指令使用 `v1.0.0` 而不是用户传递的 `ARG` 值：`v2.0.1`。这种行为类似于 shell 脚本，其中局部作用域的变量覆盖了传递的变量。从其定义的角度出发，还是从环境继承而来的。

Using the example above but a different `ENV` specification you can create more useful interactions between `ARG` and `ENV` instructions:

使用上面的示例但使用不同的 `ENV` 规范，您可以在 `ARG` 和 `ENV` 指令之间创建更有用的交互：

```Dockerfile
FROM ubuntu
ARG CONT_IMG_VER
ENV CONT_IMG_VER=${CONT_IMG_VER:-v1.0.0}
RUN echo $CONT_IMG_VER
```

Unlike an `ARG` instruction, `ENV` values are always persisted in the built image. Consider a docker build without the `--build-arg` flag:

与 `ARG` 指令不同， `ENV` 值始终保留在生成的镜像中。考虑一个没有 `--build-arg` 标志的 docker build：

```sh
$ docker build .
```

Using this Dockerfile example, `CONT_IMG_VER` is still persisted in the image but its value would be `v1.0.0` as it is the default set in line 3 by the `ENV` instruction.

使用此 Dockerfile 示例，`CONT_IMG_VER` 仍保留在镜像中，但其值将为 `v1.0.0`，因为它是 `ENV` 指令在第 3 行中设置的默认值。

The variable expansion technique in this example allows you to pass arguments from the command line and persist them in the final image by leveraging the `ENV` instruction. Variable expansion is only supported for [a limited set of Dockerfile instructions.](https://docs.docker.com/engine/reference/builder/#environment-replacement)

在此示例中，变量扩展技术使您可以从命令行传递参数，并利用 `ENV` 指令将其保留在最终镜像中。仅[有限的一组 Dockerfile 指令](https://docs.docker.com/engine/reference/builder/#environment-replacement)支持变量扩展。

### 预定义的变量

Docker has a set of predefined `ARG` variables that you can use without a corresponding `ARG` instruction in the Dockerfile.

Docker 有一组预定义的 `ARG` 变量，您可以在 Dockerfile 中使用它们而无需相应的 `ARG` 指令。

- `HTTP_PROXY`
- `http_proxy`
- `HTTPS_PROXY`
- `https_proxy`
- `FTP_PROXY`
- `ftp_proxy`
- `NO_PROXY`
- `no_proxy`

To use these, simply pass them on the command line using the flag:

要使用这些，只需使用以下标志在命令行中传递它们：

```sh
--build-arg <varname>=<value>
```

By default, these pre-defined variables are excluded from the output of `docker history`. Excluding them reduces the risk of accidentally leaking sensitive authentication information in an `HTTP_PROXY` variable.

默认情况下，这些预定义变量从 `docker history` 输出中排除。排除它们会降低意外泄露敏感身份验证信息到 `HTTP_PROXY` 变量中的风险。

For example, consider building the following Dockerfile using `--build-arg HTTP_PROXY=http://user:pass@proxy.lon.example.com`.

例如，考虑使用 `--build-arg HTTP_PROXY=http://user:pass@proxy.lon.example.com` 构建以下Dockerfile。

```Dockerfile
FROM ubuntu
RUN echo "Hello World"
```

In this case, the value of the `HTTP_PROXY` variable is not available in the `docker history` and is not cached. If you were to change location, and your proxy server changed to `http://user:pass@proxy.sfo.example.com`, a subsequent build does not result in a cache miss.

在这种情况下，`docker_history` 中没有 `HTTP_PROXY` 变量的值，也不被缓存。如果要更改位置，并且您的代理服务器已更改为 `http://user:pass@proxy.sfo.example.com`，则后续的构建不会导致高速缓存未命中。

If you need to override this behaviour then you may do so by adding an `ARG` statement in the Dockerfile as follows:

如果您需要覆盖此行为，则可以通过在 Dockerfile 中添加 `ARG` 语句来做到这一点，如下所示：

```Dockerfile
FROM ubuntu
ARG HTTP_PROXY
RUN echo "Hello World"
```

When building this Dockerfile, the `HTTP_PROXY` is preserved in the `docker history`, and changing its value invalidates the build cache.

构建此 Dockerfile 时，`HTTP_PROXY` 保留在 `docker history` 中，并且更改其值会使构建缓存无效。

### 对构建缓存的影响

`ARG` variables are not persisted into the built image as `ENV` variables are. However, `ARG` variables do impact the build cache in similar ways. If a Dockerfile defines an `ARG` variable whose value is different from a previous build, then a “cache miss” occurs upon its first usage, not its definition. In particular, all `RUN` instructions following an `ARG` instruction use the `ARG` variable implicitly (as an environment variable), thus can cause a cache miss. All predefined `ARG` variables are exempt from caching unless there is a matching `ARG` statement in the `Dockerfile`.

`ARG` 变量不会像 `ENV` 变量那样持久保存到构建的镜像中。但是，`ARG` 变量确实以类似的方式影响构建缓存。如果 Dockerfile 定义了一个值与先前版本不同的 `ARG` 变量，则首次使用时会发生“缓存未命中”，而不是其定义。尤其是，紧跟在 `ARG` 指令之后的所有 `RUN` 指令都隐式地使用 `ARG` 变量（作为环境变量），因此可能导致高速缓存未命中。除非在 Dockerfile 中有匹配的 `ARG` 语句，否则所有预定义的 `ARG` 变量均免于缓存。

For example, consider these two Dockerfile:

例如，考虑以下两个 Dockerfile：

```Dockerfile
FROM ubuntu
ARG CONT_IMG_VER
RUN echo $CONT_IMG_VER
```

```Dockerfile
FROM ubuntu
ARG CONT_IMG_VER
RUN echo hello
```

If you specify `--build-arg CONT_IMG_VER=<value>` on the command line, in both cases, the specification on line 2 does not cause a cache miss; line 3 does cause a cache miss.`ARG CONT_IMG_VER` causes the RUN line to be identified as the same as running `CONT_IMG_VER=<value> echo hello`, so if the `<value>` changes, we get a cache miss.

如果在命令行上指定 `--build-arg CONT_IMG_VER = <value>`，则在两种情况下，第 2 行的规范都不会导致高速缓存未命中。第 3 行确实会导致缓存未命中。`ARG CONT_IMG_VER` 会导致 RUN 行被标识为与运行 `CONT_IMG_VER = <value> echo hello` 相同，因此，如果 `<value>` 发生更改，我们将得到缓存未命中。

Consider another example under the same command line:

考虑同一命令行下的另一个示例：

```Dockerfile
FROM ubuntu
ARG CONT_IMG_VER
ENV CONT_IMG_VER=$CONT_IMG_VER
RUN echo $CONT_IMG_VER
```

In this example, the cache miss occurs on line 3. The miss happens because the variable’s value in the `ENV` references the `ARG` variable and that variable is changed through the command line. In this example, the `ENV` command causes the image to include the value.

在此示例中，高速缓存未命中发生在第 3 行。之所以发生未命中，是因为 `ENV` 中的变量值引用了 `ARG` 变量，并且该变量通过命令行进行了更改。在这个例子中，`ENV` 命令使镜像包含该值。

If an `ENV` instruction overrides an `ARG` instruction of the same name, like this Dockerfile:

如果 `ENV` 指令覆盖了同名的 `ARG` 指令，例如 Dockerfile：

```Dockerfile
FROM ubuntu
ARG CONT_IMG_VER
ENV CONT_IMG_VER=hello
RUN echo $CONT_IMG_VER
```

Line 3 does not cause a cache miss because the value of `CONT_IMG_VER` is a constant (`hello`). As a result, the environment variables and values used on the `RUN` (line 4) doesn’t change between builds.

第 3 行不会导致缓存未命中，因为 `CONT_IMG_VER` 的值是一个常量（hello）。因此，`RUN`（第4行）中使用的环境变量和值在构建之间不会更改。

## 总结

介绍了 Dockerfile 中 ARG 指令的使用方式，预定义的变量和对缓存的影响。