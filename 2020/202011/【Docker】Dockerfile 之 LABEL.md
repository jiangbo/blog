# 【Docker】Dockerfile 之 LABEL

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## LABEL

```Dockerfile
LABEL <key>=<value> <key>=<value> <key>=<value> ...
```

The `LABEL` instruction adds metadata to an image. A `LABEL` is a key-value pair. To include spaces within a `LABEL` value, use quotes and backslashes as you would in command-line parsing. A few usage examples:

`LABEL` 指令将元数据添加到镜像。`LABEL` 是键值对。要在 `LABEL` 值中包含空格，请像在命令行中一样使用引号和反斜杠。一些用法示例：

```Dockerfile
LABEL "com.example.vendor"="ACME Incorporated"
LABEL com.example.label-with-value="foo"
LABEL version="1.0"
LABEL description="This text illustrates \
that label-values can span multiple lines."
```

An image can have more than one label. You can specify multiple labels on a single line. Prior to Docker 1.10, this decreased the size of the final image, but this is no longer the case. You may still choose to specify multiple labels in a single instruction, in one of the following two ways:

一个镜像可以有多个标签。您可以在一行上指定多个标签。在 Docker 1.10 之前，这减小了最终镜像的大小，但是现在不再如此。您仍然可以选择以下两种方式之一在一条指令中指定多个标签：

```Dockerfile
LABEL multi.label1="value1" multi.label2="value2" other="value3"
```

```Dockerfile
LABEL multi.label1="value1" \
      multi.label2="value2" \
      other="value3"
```

Labels included in base or parent images (images in the `FROM` line) are inherited by your image. If a label already exists but with a different value, the most-recently-applied value overrides any previously-set value.

基础或父镜像（`FROM` 行中的镜像）中包含的标签由您的镜像继承。如果标签已经存在但具有不同的值，则最近应用的值将覆盖任何先前设置的值。

To view an image’s labels, use the `docker image inspect` command. You can use the `--format` option to show just the labels;

要查看镜像的标签，请使用 `docker image inspect` 命令。您可以使用 --format 选项仅显示标签；

```sh
docker image inspect --format='' myimage
```

```json
{
  "com.example.vendor": "ACME Incorporated",
  "com.example.label-with-value": "foo",
  "version": "1.0",
  "description": "This text illustrates that label-values can span multiple lines.",
  "multi.label1": "value1",
  "multi.label2": "value2",
  "other": "value3"
}
```

## 示例

### Dockerfile 文件

```Dockerfile
FROM busybox
LABEL author=jiangbo
CMD echo jiangbo
```

### 构建结果

```sh
[root@master env]# docker build -t jiangbo:0.0.1 . --no-cache
Sending build context to Docker daemon  3.584kB
Step 1/3 : FROM busybox
 ---> dc3bacd8b5ea
Step 2/3 : LABEL author=jiangbo
 ---> Running in 0ff7462c023b
Removing intermediate container 0ff7462c023b
 ---> bae66ec32f55
Step 3/3 : CMD echo jiangbo
 ---> Running in 2a3701bc64fc
Removing intermediate container 2a3701bc64fc
 ---> f75b03f5ec1f
Successfully built f75b03f5ec1f
Successfully tagged jiangbo:0.0.1
[root@master env]#
```

### 查看标签

```sh
[root@master env]# docker image inspect jiangbo:0.0.1 --format "{{json .ContainerConfig.Labels}}"|jq
{
  "author": "jiangbo"
}
```

## MAINTAINER

>MAINTAINER 已过时，使用 LABEL 代替。

## 总结

介绍了 Dockerfile 中 LABEL 指令的使用。