# 【Docker】Dockerfile 之 ONBUILD

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## ONBUILD

```Dockerfile
ONBUILD <INSTRUCTION>
```

The `ONBUILD` instruction adds to the image a _trigger_ instruction to be executed at a later time, when the image is used as the base for another build. The trigger will be executed in the context of the downstream build, as if it had been inserted immediately after the `FROM` instruction in the downstream `Dockerfile`.

当镜像被用作另一个构建的基础镜像时，`ONBUILD` 指令会在镜像上添加 _trigger_ 指令，以便稍后执行。触发器将在下游构建的上下文中执行，就好像它是在下游 `Dockerfile` 中的 `FROM` 指令之后立即插入的。

Any build instruction can be registered as a trigger.

任何构建指令都可以注册为触发器。

This is useful if you are building an image which will be used as a base to build other images, for example an application build environment or a daemon which may be customized with user-specific configuration.

如果您要构建的镜像将用作构建其他镜像的基础，例如，可以使用用户特定的配置自定义的应用程序构建环境或守护程序，则此功能很有用。

For example, if your image is a reusable Python application builder, it will require application source code to be added in a particular directory, and it might require a build script to be called _after_ that. You can’t just call `ADD` and `RUN` now, because you don’t yet have access to the application source code, and it will be different for each application build. You could simply provide application developers with a boilerplate `Dockerfile` to copy-paste into their application, but that is inefficient, error-prone and difficult to update because it mixes with application-specific code.

例如，如果您的镜像是可重用的 Python 应用程序构建器，则将需要在特定目录中添加应用程序源代码，并且在那之后可能需要调用构建脚本。您不能立即调用 `ADD` 和 `RUN`，因为您还没有访问应用程序源代码的权限，并且每个应用程序构建版本的代码都不同。您可以简单地为应用程序开发人员提供模板文件 `Dockerfile`，以将其复制粘贴到他们的应用程序中，但这效率低下，容易出错且难以更新，因为它与特定于应用程序的代码混合在一起。

The solution is to use `ONBUILD` to register advance instructions to run later, during the next build stage.

解决方案是使用 `ONBUILD` 注册预先的指令，以便在下一个构建阶段稍后运行。

Here’s how it works:
运作方式如下：

1. When it encounters an `ONBUILD` instruction, the builder adds a trigger to the metadata of the image being built. The instruction does not otherwise affect the current build.
1. 构建器在遇到 `ONBUILD` 指令时，将触发器添加到正在构建的图像的元数据中。该指令不会影响当前版本。

2. At the end of the build, a list of all triggers is stored in the image manifest, under the key `OnBuild`. They can be inspected with the `docker inspect` command.
2. 在构建结束时，所有触发器的列表都存储在镜像清单中的 `OnBuild` 键下。可以使用 `docker inspect` 命令来检查它们。

3. Later the image may be used as a base for a new build, using the `FROM` instruction. As part of processing the `FROM` instruction, the downstream builder looks for `ONBUILD` triggers, and executes them in the same order they were registered. If any of the triggers fail, the `FROM` instruction is aborted which in turn causes the build to fail. If all triggers succeed, the `FROM` instruction completes and the build continues as usual.
3. 之后，可以使用 `FROM` 指令将该镜像用作新版本的基础镜像。作为处理 `FROM` 指令的一部分，下游构建器将查找 `ONBUILD` 触发器，并以与注册时相同的顺序执行它们。如果任何触发器失败，则 `FROM` 指令将中止，从而导致构建失败。如果所有触发均成功，则 `FROM` 指令完成，并且构建照常继续。

4. Triggers are cleared from the final image after being executed. In other words they are not inherited by “grand-children” builds.
4.触发器在执行后从最终镜像中清除。换句话说，它们不是“孙子代”版本所继承的。

For example you might add something like this:
例如，您可以添加以下内容：

```Dockerfile
ONBUILD ADD . /app/src
ONBUILD RUN /usr/local/bin/python-build --dir /app/src
```

> **Warning**
> 
> Chaining `ONBUILD` instructions using `ONBUILD ONBUILD` isn’t allowed.

> **警告**
>
>不允许使用 `ONBUILD ONBUILD` 链接 `ONBUILD` 指令。

> **Warning**
> 
> The `ONBUILD` instruction may not trigger `FROM` or `MAINTAINER` instructions.

> **警告**
>
> `ONBUILD` 指令可能不会触发 `FROM` 或 `MAINTAINER` 指令。

## 总结

介绍了 Dockerfile 中 ONBUILD 指令的用法和注意事项。