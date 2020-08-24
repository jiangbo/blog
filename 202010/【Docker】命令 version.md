# 【Docker】命令 version

参考教程：https://docs.docker.com/engine/reference/commandline/version/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker version [OPTIONS]`

默认情况下，Docker 使用易于阅读的格式显示所有的信息。如果指定了显示的格式，将会按照指定的格式进行输出。

## 命令选项

### format

根据指定的格式显示输出信息，其中 `-f` 是 `--format` 的短命令形式。一般来说，在命令行手动输入命令时，使用短命令形式，可以减少输入。而长命令的形式，用在编写脚本的文件中，增强可读性。

## 示例

### 默认输出

```sh
$ docker version
Client: Docker Engine - Community
 Version:           19.03.6
 API version:       1.40
 Go version:        go1.12.16
 Git commit:        369ce74a3c
 Built:             Thu Feb 13 01:27:49 2020
 OS/Arch:           linux/amd64
 Experimental:      true

Server: Docker Engine - Community
 Engine:
  Version:          19.03.6
  API version:      1.40 (minimum version 1.12)
  Go version:       go1.12.16
  Git commit:       369ce74a3c
  Built:            Thu Feb 13 01:26:21 2020
  OS/Arch:          linux/amd64
  Experimental:     false
 containerd:
  Version:          1.2.10
  GitCommit:        b34a5c8af56e510852c35414db4c1f4fa6172339
 runc:
  Version:          1.0.0-rc8+dev
  GitCommit:        3e425f80a8c931f88e6d94a8c831b9d5aa481657
 docker-init:
  Version:          0.18.0
  GitCommit:        fec3683
```

### 获取版本信息

```sh
$ docker version --format '{{.Server.Version}}'
19.03.6
```

### 输出 json 格式

```sh
$ docker version --format '{{json .}}'
{"Client":{"Platform":{"Name":"Docker Engine - Community"},"Version":"19.03.6","ApiVersion":"1.40","DefaultAPIVersion":"1.40","GitCommit":"369ce74a3c","GoVersion":"go1.12.16","Os":"linux","Arch":"amd64","BuildTime":"Thu Feb 13 01:27:49 2020","Experimental":true},"Server":{"Platform":{"Name":"Docker Engine - Community"},"Components":[{"Name":"Engine","Version":"19.03.6","Details":{"ApiVersion":"1.40","Arch":"amd64","BuildTime":"Thu Feb 13 01:26:21 2020","Experimental":"false","GitCommit":"369ce74a3c","GoVersion":"go1.12.16","KernelVersion":"4.15.0-88-generic","MinAPIVersion":"1.12","Os":"linux"}},{"Name":"containerd","Version":"1.2.10","Details":{"GitCommit":"b34a5c8af56e510852c35414db4c1f4fa6172339"}},{"Name":"runc","Version":"1.0.0-rc8+dev","Details":{"GitCommit":"3e425f80a8c931f88e6d94a8c831b9d5aa481657"}},{"Name":"docker-init","Version":"0.18.0","Details":{"GitCommit":"fec3683"}}],"Version":"19.03.6","ApiVersion":"1.40","MinAPIVersion":"1.12","GitCommit":"369ce74a3c","GoVersion":"go1.12.16","Os":"linux","Arch":"amd64","KernelVersion":"4.15.0-88-generic","BuildTime":"2020-02-13T01:26:21.000000000+00:00"}}
```

## 总结

介绍了 version 命令的使用，可以输出 docker 的版本信息。介绍了 `--format` 选项的作用，可以获取指定的值，也可以对输出的内容进行格式化。

