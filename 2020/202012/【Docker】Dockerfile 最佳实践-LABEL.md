# 【Docker】Dockerfile 最佳实践-LABEL

参考教程：https://docs.docker.com/develop/develop-images/dockerfile_best-practices/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## LABEL

You can add labels to your image to help organize images by project, record licensing information, to aid in automation, or for other reasons. For each label, add a line beginning with `LABEL` and with one or more key-value pairs. The following examples show the different acceptable formats. Explanatory comments are included inline.

您可以在镜像上添加标签，以帮助按项目组织镜像，记录许可信息，帮助自动化或其他原因。对于每个标签，添加以 `LABEL` 开头并带有一个或多个键值对的行。以下示例显示了不同的可接受格式。内嵌包含解释性注释。

> Strings with spaces must be quoted **or** the spaces must be escaped. Inner quote characters (`"`), must also be escaped.

> 带有空格的字符串必须用引号引起来，或者必须将空格转义。内引号字符（“"”）也必须转义。

```Dockerfile
# Set one or more individual labels
LABEL com.example.version="0.0.1-beta"
LABEL vendor1="ACME Incorporated"
LABEL vendor2=ZENITH\ Incorporated
LABEL com.example.release-date="2015-02-12"
LABEL com.example.version.is-production=""
```

An image can have more than one label. Prior to Docker 1.10, it was recommended to combine all labels into a single `LABEL` instruction, to prevent extra layers from being created. This is no longer necessary, but combining labels is still supported.

一个镜像可以有多个标签。在 Docker 1.10 之前，建议将所有标签合并为一个 `LABEL` 指令，以防止创建额外的层。现在不再需要此操作，但仍支持组合标签。

```Dockerfile
# Set multiple labels on one line
LABEL com.example.version="0.0.1-beta" com.example.release-date="2015-02-12"
```

The above can also be written as:
上面也可以写成：

```Dockerfile
# Set multiple labels at once, using line-continuation characters to break long lines
LABEL vendor=ACME\ Incorporated \
      com.example.is-beta= \
      com.example.is-production="" \
      com.example.version="0.0.1-beta" \
      com.example.release-date="2015-02-12"
```

See [Understanding object labels](https://docs.docker.com/config/labels-custom-metadata/) for guidelines about acceptable label keys and values. For information about querying labels, refer to the items related to filtering in [Managing labels on objects](https://docs.docker.com/config/labels-custom-metadata/#manage-labels-on-objects). See also [LABEL](https://docs.docker.com/engine/reference/builder/#label) in the Dockerfile reference.

请参阅[了解对象标签](https://docs.docker.com/config/labels-custom-metadata/)了解有关可接受的标签键和值的准则。有关查询标签的信息，请参阅[管理对象上的标签](https://docs.docker.com/config/labels-custom-metadata/#manage-labels-on-objects)中与过滤有关的项目。另请参阅 Dockerfile 参考中的 [LABEL](https://docs.docker.com/engine/reference/builder/#label)。

## 总结

介绍了 Dockerfile 的 LABEL 指令的最佳实践。