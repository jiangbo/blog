# 【Docker】Dockerfile 之 HEALTHCHECK

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## HEALTHCHECK

The `HEALTHCHECK` instruction has two forms:
`HEALTHCHECK` 指令有两种形式：

- `HEALTHCHECK [OPTIONS] CMD command` (check container health by running a command inside the container)
- `HEALTHCHECK [OPTIONS] CMD command`（通过在容器内部运行命令来检查容器的运行状况）
- `HEALTHCHECK NONE` (disable any healthcheck inherited from the base image)
- `HEALTHCHECK NONE`（禁用从基础镜像继承的任何健康检查）

The `HEALTHCHECK` instruction tells Docker how to test a container to check that it is still working. This can detect cases such as a web server that is stuck in an infinite loop and unable to handle new connections, even though the server process is still running.

`HEALTHCHECK` 指令告诉 Docker 如何测试容器以检查其是否仍在工作。这样可以检测到诸如 Web 服务器陷入无限循环并且无法处理新连接的情况，即使服务器进程仍在运行。

When a container has a healthcheck specified, it has a _health status_ in addition to its normal status. This status is initially `starting`. Whenever a health check passes, it becomes `healthy` (whatever state it was previously in). After a certain number of consecutive failures, it becomes `unhealthy`.

指定容器的运行状况检查后，除了其正常状态外，容器还具有 _health status_。此状态最初是 `starting`。只要健康检查通过，它就会变成 `healthy` 状态（无论以前处于什么状态）。在一定数量的连续故障之后，它变得 `unhealthy`。

The options that can appear before `CMD` are:
可以在 `CMD` 之前出现的选项是：

- `--interval=DURATION` (default: `30s`)
- `--timeout=DURATION` (default: `30s`)
- `--start-period=DURATION` (default: `0s`)
- `--retries=N` (default: `3`)

The health check will first run **interval** seconds after the container is started, and then again **interval** seconds after each previous check completes.

运行状况检查将在容器启动后首先运行 **interval** 秒，然后在之前每次检查完成后再次 **interval**秒。

If a single run of the check takes longer than **timeout** seconds then the check is considered to have failed.

如果单次检查花费的时间超过 **timeout** 秒，则认为检查失败。

It takes **retries** consecutive failures of the health check for the container to be considered `unhealthy`.

要使容器被视为 `unhealthy`，需要进行 **retries** 连续失败的健康检查。

**start period** provides initialization time for containers that need time to bootstrap. Probe failure during that period will not be counted towards the maximum number of retries. However, if a health check succeeds during the start period, the container is considered started and all consecutive failures will be counted towards the maximum number of retries.

**开始时间** 为需要时间进行引导的容器提供了初始化时间。在此期间内的探针故障将不计入最大重试次数。但是，如果运行状况检查在启动期间成功，则认为该容器已启动，并且所有连续失败将计入最大重试次数。

There can only be one `HEALTHCHECK` instruction in a Dockerfile. If you list more than one then only the last `HEALTHCHECK` will take effect.

Dockerfile 中只能有一条 `HEALTHCHECK` 指令。如果您列出多个，则只有最后一个 `HEALTHCHECK` 才会生效。

The command after the `CMD` keyword can be either a shell command (e.g. `HEALTHCHECK CMD /bin/check-running`) or an _exec_ array (as with other Dockerfile commands; see e.g. `ENTRYPOINT` for details).

关键字 `CMD` 之后的命令可以是 shell 命令（例如 `HEALTHCHECK CMD /bin/check-running`）或 _exec_ 数组（与其他 Dockerfile 命令一样；有关详细信息，请参见 `ENTRYPOINT`）。

The command’s exit status indicates the health status of the container. The possible values are:
命令的退出状态指示容器的健康状态。可能的值为：

- 0: success - the container is healthy and ready for use
- 1: unhealthy - the container is not working correctly
- 2: reserved - do not use this exit code
- 0：成功-容器健康且可以使用
- 1：不健康-容器无法正常工作
- 2：保留-请勿使用此退出代码

For example, to check every five minutes or so that a web-server is able to serve the site’s main page within three seconds:

例如，要每五分钟检查一次，以便网络服务器能够在三秒钟内为网站的首页提供服务：

```Dockerfile
HEALTHCHECK --interval=5m --timeout=3s \
  CMD curl -f http://localhost/ || exit 1
```

To help debug failing probes, any output text (UTF-8 encoded) that the command writes on stdout or stderr will be stored in the health status and can be queried with `docker inspect`. Such output should be kept short (only the first 4096 bytes are stored currently).

为了帮助调试失败的探针，该命令在 stdout 或 stderr 上写入的任何输出文本（UTF-8编码）将以健康状态存储，并可以通过 `docker inspect` 查询。此类输出应保持简短（当前仅存储前4096个字节）。

When the health status of a container changes, a `health_status` event is generated with the new status.

当容器的健康状态发生变化时，将使用新状态生成一个 `health_status` 事件。

## 总结

介绍了 Dockerfile 中 HEALTHCHECK 指令的用法和注意事项。