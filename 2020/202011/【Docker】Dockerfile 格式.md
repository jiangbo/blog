# 【Docker】Dockerfile 格式

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 格式

Here is the format of the `Dockerfile`:

这是 Dockerfile 的格式：

```Dockerfile
# Comment
INSTRUCTION arguments
```

The instruction is not case-sensitive. However, convention is for them to be UPPERCASE to distinguish them from arguments more easily.

指令不区分大小写。但是，按约定将它们大写，以便更轻松地将它们与参数区分开。

Docker runs instructions in a `Dockerfile` in order. A `Dockerfile` **must begin with a `FROM` instruction**. This may be after [parser directives](https://docs.docker.com/engine/reference/builder/#parser-directives), [comments](https://docs.docker.com/engine/reference/builder/#format), and globally scoped [ARGs](https://docs.docker.com/engine/reference/builder/#arg). The `FROM` instruction specifies the [_Parent Image_](https://docs.docker.com/glossary/#parent_image) from which you are building. `FROM` may only be preceded by one or more `ARG` instructions, which declare arguments that are used in `FROM` lines in the `Dockerfile`.

Docker 依次运行在 Dockerfile 中的指令。Dockerfile **必须以 FROM 指令开头**。这可能在[parser directives](https://docs.docker.com/engine/reference/builder/#parser-directives)，[注释](https://docs.docker.com/engine/reference/builder)和全局范围内的 [ARG](https://docs.docker.com/engine/reference/builder/#arg) 之后。`FROM` 指令指定基础镜像。在 FROM 之前只能有一个或多个 ARG 指令，这些指令声明在 Dockerfile 的 FROM 行中使用的参数。

Docker treats lines that _begin_ with `#` as a comment, unless the line is a valid [parser directive](https://docs.docker.com/engine/reference/builder/#parser-directives). A `#` marker anywhere else in a line is treated as an argument. This allows statements like:

除非该行是有效的 [parser directives](https://docs.docker.com/engine/reference/builder/#parser-directives)，否则 Docker 会将以 `＃` 号开头的行作为注释。一行中其他任何地方的 `＃` 标记都被视为参数。这允许如下语句：

```sh
# Comment
RUN echo 'we are running some # of cool things'
```

Comment lines are removed before the Dockerfile instructions are executed, which means that the comment in the following example is not handled by the shell executing the `echo` command, and both examples below are equivalent:

在执行 Dockerfile 指令之前会删除注释行，这意味着以下示例中的注释不会由执行`echo` 命令的 shell 处理，并且以下两个示例是等效的：

```Dockerfile
RUN echo hello \
# comment
world
```

```Dockerfile
RUN echo hello \
world
```

Line continuation characters are not supported in comments.

注释中不支持换行符。

## 空格注意事项

For backward compatibility, leading whitespace before comments (`#`) and instructions (such as `RUN`) are ignored, but discouraged. Leading whitespace is not preserved in these cases, and the following examples are therefore equivalent:

为了向后兼容，注释（`＃`）和指令（例如 `RUN`）之前的空格将被忽略，但不鼓励使用。在这些情况下，不保留前导空格，因此以下示例是等效的：


```Dockerfile
        # this is a comment-line
    RUN echo hello
RUN echo world
```

```Dockerfile
# this is a comment-line
RUN echo hello
RUN echo world
```

Note however, that whitespace in instruction _arguments_, such as the commands following `RUN`, are preserved, so the following example prints \` hello world\` with leading whitespace as specified:

但是请注意，指令参数中的空格（例如 `RUN` 命令之后的空格）被保留，因此以下示例使用指定的前导空格打印\` hello world\`：


```sh
RUN echo "\
     hello\
     world
```

## 总结

介绍了 Dockerfile 的格式。