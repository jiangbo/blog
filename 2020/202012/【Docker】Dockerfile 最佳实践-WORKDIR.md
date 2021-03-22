# 【Docker】Dockerfile 最佳实践-WORKDIR

参考教程：https://docs.docker.com/develop/develop-images/dockerfile_best-practices/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## WORKDIR

For clarity and reliability, you should always use absolute paths for your `WORKDIR`. Also, you should use `WORKDIR` instead of proliferating instructions like `RUN cd … && do-something`, which are hard to read, troubleshoot, and maintain.

为了清楚和可靠，您应始终为 `WORKDIR` 使用绝对路径。同样，您应该使用 `WORKDIR` 来代替诸如 `RUN cd … && do-something` 之类的指令，这些指令难以阅读，排除故障和维护。

## 总结

介绍了 Dockerfile 的 WORKDIR 指令的最佳实践。