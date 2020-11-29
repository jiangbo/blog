# 【Docker】Dockerfile 之 ENV

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## ENV

```Dockerfile
ENV <key>=<value> ...
```

The `ENV` instruction sets the environment variable `<key>` to the value `<value>`. This value will be in the environment for all subsequent instructions in the build stage and can be [replaced inline](https://docs.docker.com/engine/reference/builder/#environment-replacement) in many as well. The value will be interpreted for other environment variables, so quote characters will be removed if they are not escaped. Like command line parsing, quotes and backslashes can be used to include spaces within values.

`ENV` 指令将环境变量 `<key>` 设置为值 `<value>`。此值将在构建阶段的所有后续指令的环境中使用，也可以在很多情况下[内联替换](https://docs.docker.com/engine/reference/builder/#environment-replacement)。该值将被其他环境变量解释，因此如果不对引号字符进行转义，则将其删除。与命令行解析一样，引号和反斜杠可用于在值中包含空格。

Example:

例如：

```Dockerfile
ENV MY_NAME="John Doe"
ENV MY_DOG=Rex\ The\ Dog
ENV MY_CAT=fluffy
```

The `ENV` instruction allows for multiple `<key>=<value> ...` variables to be set at one time, and the example below will yield the same net results in the final image:

`ENV` 指令允许一次设置多个 `<key> = <value> ...` 变量，下面的示例将在最终镜像中产生相同的最终结果：

```Dockerfile
ENV MY_NAME="John Doe" MY_DOG=Rex\ The\ Dog \
    MY_CAT=fluffy
```

The environment variables set using `ENV` will persist when a container is run from the resulting image. You can view the values using `docker inspect`, and change them using `docker run --env <key>=<value>`.

当从结果镜像运行容器时，使用 `ENV` 设置的环境变量将保留。您可以使用 `docker inspect` 查看值，并使用 `docker run --env <key> = <value>` 更改它们。

Environment variable persistence can cause unexpected side effects. For example, setting `ENV DEBIAN_FRONTEND=noninteractive` changes the behavior of `apt-get`, and may confuse users of your image.

环境变量的持久性可能导致意外的副作用。例如，设置 `ENV DEBIAN_FRONTEND=noninteractive` 会更改 `apt-get` 的行为，并可能使镜像用户感到困惑。

If an environment variable is only needed during build, and not in the final image, consider setting a value for a single command instead:

如果仅在构建过程中需要环境变量，而在最终映像中则不需要，请考虑为单个命令设置一个值：

```Dockerfile
RUN DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install -y ...
```

Or using [`ARG`](https://docs.docker.com/engine/reference/builder/#arg), which is not persisted in the final image:

或使用 [`ARG`](https://docs.docker.com/engine/reference/builder/#arg)，它不会保留在最终镜像中：

```Dockerfile
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y ...
```

> **Alternative syntax**
>
> **替代语法**
> 
> The `ENV` instruction also allows an alternative syntax `ENV <key> <value>`, omitting the `=`. For example:
>
>`ENV` 指令还允许使用替代语法 `ENV <key> <value>`，而忽略 `=`。例如：
> 
> ```Dockerfile
> ENV MY_VAR my-value
> ```
>
> This syntax does not allow for multiple environment-variables to be set in a single `ENV` instruction, and can be confusing. For example, the following sets a single environment variable (`ONE`) with value `"TWO= THREE=world"`:
>
> 这种语法不允许在单个 `ENV` 指令中设置多个环境变量，这可能会造成混淆。例如，以下代码将单个环境变量（`ONE`）的值设置为 `"TWO= THREE=world"`：
>
> ```Dockerfile
> ENV ONE TWO= THREE=world
> ```
> 
> The alternative syntax is supported for backward compatibility, but discouraged for the reasons outlined above, and may be removed in a future release.
>
>支持备用语法以实现向后兼容，但出于上述原因，不建议使用该语法，并且在将来的版本中可能会删除该语法。

## 示例

### Dockerfile 文件

```Dockerfile
FROM busybox
LABEL author=jiangbo
ENV name=jiangbo
CMD echo $name
```

### 构建结果

```sh
[root@master env]# docker build -t jiangbo:0.0.1 .
Sending build context to Docker daemon  3.584kB
Step 1/4 : FROM busybox
 ---> dc3bacd8b5ea
Step 2/4 : LABEL author=jiangbo
 ---> Running in e7a093e0fc49
Removing intermediate container e7a093e0fc49
 ---> 8129cae696ad
Step 3/4 : ENV name=jiangbo
 ---> Running in 8bdcffbe711e
Removing intermediate container 8bdcffbe711e
 ---> c8a646bfdab3
Step 4/4 : CMD echo $name
 ---> Running in 9cfc4f234843
Removing intermediate container 9cfc4f234843
 ---> b6f80c7fac3f
Successfully built b6f80c7fac3f
Successfully tagged jiangbo:0.0.1
```

### 查看结果

```sh
[root@master env]# docker run jiangbo:0.0.1
jiangbo
```


## 总结

介绍了 Dockerfile 中 ENV 指令的使用。