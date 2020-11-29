# 【Docker】Dockerfile 解析器指令

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 解析器指令

Parser directives are optional, and affect the way in which subsequent lines in a `Dockerfile` are handled. Parser directives do not add layers to the build, and will not be shown as a build step. Parser directives are written as a special type of comment in the form `# directive=value`. A single directive may only be used once.

解析器指令是可选的，并且会影响 Dockerfile 中后续行的处理方式。解析器指令不会在构建中添加镜像层，也不会显示为构建步骤。解析器指令以 `# directive=value` 的形式写为特殊的注释类型。单个指令只能使用一次。

Once a comment, empty line or builder instruction has been processed, Docker no longer looks for parser directives. Instead it treats anything formatted as a parser directive as a comment and does not attempt to validate if it might be a parser directive. Therefore, all parser directives must be at the very top of a `Dockerfile`.

处理完注释，空行或生成器指令后，Docker 不再寻找解析器指令。而是将格式化为解析器指令的任何内容都视为注释，并且不会尝试验证它是否可能是解析器指令。因此，所有解析器指令必须位于 Dockerfile 的最顶部。

Parser directives are not case-sensitive. However, convention is for them to be lowercase. Convention is also to include a blank line following any parser directives. Line continuation characters are not supported in parser directives.

解析器指令不区分大小写。但是，约定是小写的。约定还应在任何解析器指令之后包含一个空白行。解析器伪指令不支持行继续符。

Due to these rules, the following examples are all invalid:

由于这些规则，以下示例均无效：

Invalid due to line continuation:

由于行继续而无效：

```Dockerfile
# direc \
tive=value
```

Invalid due to appearing twice:

由于出现两次而无效：

```Dockerfile
# directive=value1
# directive=value2

FROM ImageName
```

Treated as a comment due to appearing after a builder instruction:

由于在构建指令之后出现，因此被视为注释：

```Dockerfile
FROM ImageName
# directive=value
```

Treated as a comment due to appearing after a comment which is not a parser directive:

由于出现在不是解析器指令的注释之后，因此被视为注释：

```Dockerfile
# About my dockerfile
# directive=value
FROM ImageName
```

The unknown directive is treated as a comment due to not being recognized. In addition, the known directive is treated as a comment due to appearing after a comment which is not a parser directive.

由于未被识别，未知指令被视为注释。另外，由于在非解析程序指令的注释之后出现，因此已知指令被视为注释。

```Dockerfile
# unknowndirective=value
# knowndirective=value
```

Non line-breaking whitespace is permitted in a parser directive. Hence, the following lines are all treated identically:

解析器指令中允许非换行空格。因此，以下各行都被相同地对待：

```Dockerfile
#directive=value
# directive =value
#	directive= value
# directive = value
#	  dIrEcTiVe=value
```

The following parser directives are supported:

支持以下解析器指令：

- `syntax`
- `escape`

## 总结

介绍了 Dockerfile 的解析器指令格式。