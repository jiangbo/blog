# 【Docker】命令 info

参考教程：https://docs.docker.com/engine/reference/commandline/info/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker info [OPTIONS]`

此命令显示有关 Docker 安装的系统信息。显示的信息包括内核版本，容器和镜像数量。显示的镜像数量是唯一镜像的数量。用不同名称标记的同一个镜像仅计算一次。如果指定了显示的格式，则会按照指定的格式进行显示。

根据所使用的存储驱动，可以显示其他额外的信息，例如池名称，数据文件，元数据文件，使用的数据空间，总数据空间，使用的元数据空间和总元数据空间。

数据文件是存储镜像的位置，元数据文件是存储与那些镜像有关的元数据的位置。首次运行时，Docker 将从 `/var/lib/ docker` 的卷上的可用空间中分配一定数量的数据空间和元数据空间。

## 命令选项

### format

根据指定的格式显示输出信息，其中 `-f` 是 `--format` 的短命令形式。一般来说，在命令行手动输入命令时，使用短命令形式，可以减少输入。而长命令的形式，用在编写脚本的文件中，增强可读性。

## 示例

### 默认输出

```sh
$ docker info
Client:
 Debug Mode: false
 Plugins:
  app: Docker Application (Docker Inc., v0.8.0)
  buildx: Build with BuildKit (Docker Inc., v0.3.1-tp-docker)

Server:
 Containers: 20
  Running: 20
  Paused: 0
  Stopped: 0
 Images: 31
 Server Version: 19.03.6
 Storage Driver: overlay
  Backing Filesystem: extfs
  Supports d_type: true
 Logging Driver: json-file
 Cgroup Driver: cgroupfs
 Plugins:
  Volume: local
  Network: bridge host ipvlan macvlan null overlay
  Log: awslogs fluentd gcplogs gelf journald json-file local logentries splunk syslog
 Swarm: inactive
 Runtimes: runc
 Default Runtime: runc
 Init Binary: docker-init
 containerd version: b34a5c8af56e510852c35414db4c1f4fa6172339
 runc version: 3e425f80a8c931f88e6d94a8c831b9d5aa481657
 init version: fec3683
 Security Options:
  apparmor
  seccomp
   Profile: default
 Kernel Version: 4.15.0-88-generic
 Operating System: Ubuntu 18.04.4 LTS
 OSType: linux
 Architecture: x86_64
 CPUs: 2
 Total Memory: 2.403GiB
 Name: minikube
 ID: YI4C:27UO:6YUV:SOB6:SD7C:MHMS:JUDN:O46N:KUPH:SZDY:TGEA:WTEQ
 Docker Root Dir: /var/lib/docker
 Debug Mode: true
  File Descriptors: 129
  Goroutines: 119
  System Time: 2020-08-22T06:06:06.139588271Z
  EventsListeners: 0
 Registry: https://index.docker.io/v1/
 Labels:
 Experimental: false
 Insecure Registries:
  registry.test.training.katacoda.com:4567
  127.0.0.0/8
 Live Restore Enabled: false

WARNING: No swap limit support
WARNING: the overlay storage-driver is deprecated, and will be removed in a future release.
```

### 显示 debug 信息

```sh
$ docker -D info
Client:
 Debug Mode: true

Server:
 Containers: 14
  Running: 3
  Paused: 1
  Stopped: 10
 Images: 52
 Server Version: 1.13.0
 Storage Driver: overlay2
  Backing Filesystem: extfs
  Supports d_type: true
  Native Overlay Diff: false
 Logging Driver: json-file
 Cgroup Driver: cgroupfs
 Plugins:
  Volume: local
  Network: bridge host macvlan null overlay
 Swarm: active
 ...
```

### 输出 json 格式

```sh
$ docker info --format '{{json .}}'

{"ID":"I54V:OLXT:HVMM:TPKO:JPHQ:CQCD:JNLC:O3BZ:4ZVJ:43XJ:PFHZ:6N2S","Containers":14, ...}
```

### 内核警告

如果操作系统不支持某些功能，当你运行 `docker info` 的时候，你可能会看到以下警告之一：

* WARNING: Your kernel does not support swap limit capabilities. Limitation discarded.
* WARNING: No swap limit support

除非您确实需要限制这些资源的能力，否则您可以忽略这些警告，在这种情况下，应查阅操作系统的文档以启用它们。

## 总结

介绍了 info 命令的使用，可以输出 docker 和系统的一些信息。介绍了 `--format` 选项的作用，可以获取指定的值，也可以对输出的内容进行格式化。

