# 【Docker】Dockerfile 最佳实践-FROM

参考教程：https://docs.docker.com/develop/develop-images/dockerfile_best-practices/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## FROM

Whenever possible, use current official images as the basis for your images. We recommend the [Alpine image](https://hub.docker.com/_/alpine/) as it is tightly controlled and small in size (currently under 5 MB), while still being a full Linux distribution.

尽可能使用当前的官方镜像作为基础镜像。我们推荐 [Alpine image](https://hub.docker.com/_/alpine/)，因为它受到严格控制且尺寸较小（当前小于 5 MB），同时仍是完整的 Linux 发行版。

## 总结

介绍了 Dockerfile 的 FROM 指令的最佳实践。