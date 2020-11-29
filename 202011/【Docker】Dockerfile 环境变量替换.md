# 【Docker】Dockerfile 环境变量替换

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## Environment replacement

Environment variables (declared with [the `ENV` statement](https://docs.docker.com/engine/reference/builder/#env)) can also be used in certain instructions as variables to be interpreted by the `Dockerfile`. Escapes are also handled for including variable-like syntax into a statement literally.

环境变量（用 [ENV语句](https://docs.docker.com/engine/reference/builder/#env) 声明）也可以在某些指令中用作由 Dockerfile 解释的变量。转义也可以通过将类似变量的语法实际包含在语句中来进行处理。

Environment variables are notated in the `Dockerfile` either with `$variable_name` or `${variable_name}`. They are treated equivalently and the brace syntax is typically used to address issues with variable names with no whitespace, like `${foo}_bar`.

环境变量在 Dockerfile 中用 `$variable_name` 或 `${variable_name}` 表示。它们被同等对待，并且大括号语法通常用于解决变量名称没有空格的问题，例如 `${foo}_bar`。

The `${variable_name}` syntax also supports a few of the standard `bash` modifiers as specified below:

 `${variable_name}` 语法还支持一些标准的 bash 修饰符，如下所示：

- `${variable:-word}` indicates that if `variable` is set then the result will be that value. If `variable` is not set then `word` will be the result.
- `${variable:+word}` indicates that if `variable` is set then `word` will be the result, otherwise the result is the empty string.

- `${variable:-word}` 表示如果设置了 `variable` 则结果将是该值。如果未设置 `variable`，那么将是 `word`。
- `${variable:+word}`表示如果设置了 `variable` 则将以 `word` 作为结果，否则结果为空字符串。

In all cases, `word` can be any string, including additional environment variables.

在所有情况下，`word` 可以是任何字符串，包括其他环境变量。

Escaping is possible by adding a `\` before the variable: `\$foo` or `\${foo}`, for example, will translate to `$foo` and `${foo}` literals respectively.

可以通过在变量前添加一个`\`来进行转义：例如，`\$foo` 或 `\${foo}` 将分别转换为 `$foo` 和 `${foo}`。

Example (parsed representation is displayed after the `#`):

示例（解析的表示形式显示在 `＃` 之后）：

```Dockerfile
FROM busybox
ENV FOO=/bar
WORKDIR ${FOO}   # WORKDIR /bar
ADD . $FOO       # ADD . /bar
COPY \$FOO /quux # COPY $FOO /quux
```

Environment variables are supported by the following list of instructions in the `Dockerfile`:

Dockerfile 中的以下指令列表支持环境变量：

- `ADD`
- `COPY`
- `ENV`
- `EXPOSE`
- `FROM`
- `LABEL`
- `STOPSIGNAL`
- `USER`
- `VOLUME`
- `WORKDIR`
- `ONBUILD` (when combined with one of the supported instructions above)

Environment variable substitution will use the same value for each variable throughout the entire instruction. In other words, in this example:

在整个指令中，环境变量替换将对每个变量使用相同的值。换句话说，在此示例中：

```Dockerfile
ENV abc=hello
ENV abc=bye def=$abc
ENV ghi=$abc
```

will result in `def` having a value of `hello`, not `bye`. However, `ghi` will have a value of `bye` because it is not part of the same instruction that set `abc` to `bye`.

将导致 `def` 的值为 `hello`，而不是 `bye`。但是，`ghi` 将具有 `bye` 的值，因为它不是将 `abc` 设置为 `bye` 的同一指令的一部分。

```sh
[root@master env]# docker build .
Sending build context to Docker daemon  2.048kB
Step 1/6 : FROM busybox
 ---> dc3bacd8b5ea
Step 2/6 : ENV abc=hello
 ---> Using cache
 ---> 2c674628ad0d
Step 3/6 : ENV abc=bye def=$abc
 ---> Using cache
 ---> 27401f4e57f7
Step 4/6 : ENV ghi=$abc
 ---> Using cache
 ---> ef6aa3e1c3ea
Step 5/6 : RUN echo $def
 ---> Running in 4cf63c33a97d
hello
Removing intermediate container 4cf63c33a97d
 ---> 72c1cae07a7c
Step 6/6 : RUN echo $ghi
 ---> Running in 3cba9330d595
bye
Removing intermediate container 3cba9330d595
 ---> a43f85633ec2
Successfully built a43f85633ec2
[root@master env]# cat Dockerfile
FROM busybox
ENV abc=hello
ENV abc=bye def=$abc
ENV ghi=$abc
RUN echo $def
RUN echo $ghi
[root@master env]#
```

## 总结

介绍了 Dockerfile 中的环境变量和 ENV 指令的使用。