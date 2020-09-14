# 【Docker】命令 inspect

参考教程：https://docs.docker.com/engine/reference/commandline/inspect/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker inspect [OPTIONS] NAME|ID [NAME|ID...]`

使用 inspect 命令可以查看一个容器的具体信息。

## 命令选项

| 名称 | 默认值 | 描述 |
| --- | --- | --- |
| `--format , -f` |  | 格式化输出 |
| `--size , -s` |  | 显示的最大文件大小 |
| `--type` |  | 返回指定类型的 JSON |

## 示例

### 获取 IP 地址

```sh
$ docker run -p 80:80 --name my_nginx -d nginx
Unable to find image 'nginx:latest' locally
latest: Pulling from library/nginx
d121f8d1c412: Pull complete
ebd81fc8c071: Pull complete
655316c160af: Pull complete
d15953c0e0f8: Pull complete
2ee525c5c3cc: Pull complete
Digest: sha256:9a1f8ed9e2273e8b3bbcd2e200024adac624c2e5c9b1d420988809f5c0c41a5e
Status: Downloaded newer image for nginx:latest
50522a9679f4be5aeb958fa74b9b3ad3522cafc896fe303836a52ed216dd9a8c
$ docker inspect --format='{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' my_nginx
172.18.0.6
```

### 获取 MAC 地址

```sh
$ docker inspect --format='{{range .NetworkSettings.Networks}}{{.MacAddress}}{{end}}' my_nginx
02:42:ac:12:00:06
```

### 获取日志路径

```sh
$ docker inspect --format='{{.LogPath}}' my_nginx
/var/lib/docker/containers/50522a9679f4be5aeb958fa74b9b3ad3522cafc896fe303836a52ed216dd9a8c/50522a9679f4be5aeb958fa74b9b3ad3522cafc896fe303836a52ed216dd9a8c-json.log
```

### 获取镜像名称

```sh
$ docker inspect --format='{{.Config.Image}}'  my_nginx
nginx
```

### 获取所有绑定端口

```sh
$ docker inspect --format='{{range $p, $conf := .NetworkSettings.Ports}} {{$p}} -> {{(index $conf 0).HostPort}} {{end}}' my_nginx
 80/tcp -> 80
```

### 获取子 JSON 串

```sh
$ docker inspect --format='{{json .Config}}' my_nginx
{"Hostname":"50522a9679f4","Domainname":"","User":"","AttachStdin":false,"AttachStdout":false,"AttachStderr":false,"ExposedPorts":{"80/tcp":{}},"Tty":false,"OpenStdin":false,"StdinOnce":false,"Env":["PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin","NGINX_VERSION=1.19.2","NJS_VERSION=0.4.3","PKG_RELEASE=1~buster"],"Cmd":["nginx","-g","daemon off;"],"Image":"nginx","Volumes":null,"WorkingDir":"","Entrypoint":["/docker-entrypoint.sh"],"OnBuild":null,"Labels":{"maintainer":"NGINX Docker Maintainers <docker-maint@nginx.com>"},"StopSignal":"SIGTERM"}
```

### 获取全部信息

```sh
$ docker inspect my_nginx
[
    {
        "Id": "50522a9679f4be5aeb958fa74b9b3ad3522cafc896fe303836a52ed216dd9a8c",
        "Created": "2020-09-14T03:04:25.630965809Z",
        "Path": "/docker-entrypoint.sh",
        "Args": [
            "nginx",
            "-g",
            "daemon off;"
        ],
        "State": {
            "Status": "running",
            "Running": true,
            "Paused": false,
            "Restarting": false,
            "OOMKilled": false,
            "Dead": false,
            "Pid": 7973,
            "ExitCode": 0,
            "Error": "",
            "StartedAt": "2020-09-14T03:04:27.882028663Z",
            "FinishedAt": "0001-01-01T00:00:00Z"
        },
        "Image": "sha256:7e4d58f0e5f3b60077e9a5d96b4be1b974b5a484f54f9393000a99f3b6816e3d",
        "ResolvConfPath": "/var/lib/docker/containers/50522a9679f4be5aeb958fa74b9b3ad3522cafc896fe303836a52ed216dd9a8c/resolv.conf",
        "HostnamePath": "/var/lib/docker/containers/50522a9679f4be5aeb958fa74b9b3ad3522cafc896fe303836a52ed216dd9a8c/hostname",
        "HostsPath": "/var/lib/docker/containers/50522a9679f4be5aeb958fa74b9b3ad3522cafc896fe303836a52ed216dd9a8c/hosts",
        "LogPath": "/var/lib/docker/containers/50522a9679f4be5aeb958fa74b9b3ad3522cafc896fe303836a52ed216dd9a8c/50522a9679f4be5aeb958fa74b9b3ad3522cafc896fe303836a52ed216dd9a8c-json.log",
        "Name": "/my_nginx",
        "RestartCount": 0,
        "Driver": "overlay",
        "Platform": "linux",
        "MountLabel": "",
        "ProcessLabel": "",
        "AppArmorProfile": "docker-default",
        "ExecIDs": null,
        "HostConfig": {
            "Binds": null,
            "ContainerIDFile": "",
            "LogConfig": {
                "Type": "json-file",
                "Config": {}
            },
            "NetworkMode": "default",
            "PortBindings": {
                "80/tcp": [
                    {
                        "HostIp": "",
                        "HostPort": "80"
                    }
                ]
            },
            "RestartPolicy": {
                "Name": "no",
                "MaximumRetryCount": 0
            },
            "AutoRemove": false,
            "VolumeDriver": "",
            "VolumesFrom": null,
            "CapAdd": null,
            "CapDrop": null,
            "Capabilities": null,
            "Dns": [],
            "DnsOptions": [],
            "DnsSearch": [],
            "ExtraHosts": null,
            "GroupAdd": null,
            "IpcMode": "private",
            "Cgroup": "",
            "Links": null,
            "OomScoreAdj": 0,
            "PidMode": "",
            "Privileged": false,
            "PublishAllPorts": false,
            "ReadonlyRootfs": false,
            "SecurityOpt": null,
            "UTSMode": "",
            "UsernsMode": "",
            "ShmSize": 67108864,
            "Runtime": "runc",
            "ConsoleSize": [
                0,
                0
            ],
            "Isolation": "",
            "CpuShares": 0,
            "Memory": 0,
            "NanoCpus": 0,
            "CgroupParent": "",
            "BlkioWeight": 0,
            "BlkioWeightDevice": [],
            "BlkioDeviceReadBps": null,
            "BlkioDeviceWriteBps": null,
            "BlkioDeviceReadIOps": null,
            "BlkioDeviceWriteIOps": null,
            "CpuPeriod": 0,
            "CpuQuota": 0,
            "CpuRealtimePeriod": 0,
            "CpuRealtimeRuntime": 0,
            "CpusetCpus": "",
            "CpusetMems": "",
            "Devices": [],
            "DeviceCgroupRules": null,
            "DeviceRequests": null,
            "KernelMemory": 0,
            "KernelMemoryTCP": 0,
            "MemoryReservation": 0,
            "MemorySwap": 0,
            "MemorySwappiness": null,
            "OomKillDisable": false,
            "PidsLimit": null,
            "Ulimits": null,
            "CpuCount": 0,
            "CpuPercent": 0,
            "IOMaximumIOps": 0,
            "IOMaximumBandwidth": 0,
            "MaskedPaths": [
                "/proc/asound",
                "/proc/acpi",
                "/proc/kcore",
                "/proc/keys",
                "/proc/latency_stats",
                "/proc/timer_list",
                "/proc/timer_stats",
                "/proc/sched_debug",
                "/proc/scsi",
                "/sys/firmware"
            ],
            "ReadonlyPaths": [
                "/proc/bus",
                "/proc/fs",
                "/proc/irq",
                "/proc/sys",
                "/proc/sysrq-trigger"
            ]
        },
        "GraphDriver": {
            "Data": {
                "LowerDir": "/var/lib/docker/overlay/4ed54e8b08493548643d12b5a30c673cc2ac452f4d7f3fa690496179d2495bf0/root",
                "MergedDir": "/var/lib/docker/overlay/cfec26407e90c703f377e623c7211dfc47b97789229ba63c976cb4c51a07209f/merged",
                "UpperDir": "/var/lib/docker/overlay/cfec26407e90c703f377e623c7211dfc47b97789229ba63c976cb4c51a07209f/upper",
                "WorkDir": "/var/lib/docker/overlay/cfec26407e90c703f377e623c7211dfc47b97789229ba63c976cb4c51a07209f/work"
            },
            "Name": "overlay"
        },
        "Mounts": [],
        "Config": {
            "Hostname": "50522a9679f4",
            "Domainname": "",
            "User": "",
            "AttachStdin": false,
            "AttachStdout": false,
            "AttachStderr": false,
            "ExposedPorts": {
                "80/tcp": {}
            },
            "Tty": false,
            "OpenStdin": false,
            "StdinOnce": false,
            "Env": [
                "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
                "NGINX_VERSION=1.19.2",
                "NJS_VERSION=0.4.3",
                "PKG_RELEASE=1~buster"
            ],
            "Cmd": [
                "nginx",
                "-g",
                "daemon off;"
            ],
            "Image": "nginx",
            "Volumes": null,
            "WorkingDir": "",
            "Entrypoint": [
                "/docker-entrypoint.sh"
            ],
            "OnBuild": null,
            "Labels": {
                "maintainer": "NGINX Docker Maintainers <docker-maint@nginx.com>"
            },
            "StopSignal": "SIGTERM"
        },
        "NetworkSettings": {
            "Bridge": "",
            "SandboxID": "710ed340bf99ed946c0e31483dd227e34e34445cfb8f3101fe1486c305ae06f7",
            "HairpinMode": false,
            "LinkLocalIPv6Address": "",
            "LinkLocalIPv6PrefixLen": 0,
            "Ports": {
                "80/tcp": [
                    {
                        "HostIp": "0.0.0.0",
                        "HostPort": "80"
                    }
                ]
            },
            "SandboxKey": "/var/run/docker/netns/710ed340bf99",
            "SecondaryIPAddresses": null,
            "SecondaryIPv6Addresses": null,
            "EndpointID": "b285fb99318cff326a7713e715d99ebd974f72c9f737976a2b6f206bd29507f4",
            "Gateway": "172.18.0.1",
            "GlobalIPv6Address": "",
            "GlobalIPv6PrefixLen": 0,
            "IPAddress": "172.18.0.6",
            "IPPrefixLen": 24,
            "IPv6Gateway": "",
            "MacAddress": "02:42:ac:12:00:06",
            "Networks": {
                "bridge": {
                    "IPAMConfig": null,
                    "Links": null,
                    "Aliases": null,
                    "NetworkID": "b63e52ee23a27038a2e5f2a0e7fc52e3797233537b430129bbc221006c7906ec",
                    "EndpointID": "b285fb99318cff326a7713e715d99ebd974f72c9f737976a2b6f206bd29507f4",
                    "Gateway": "172.18.0.1",
                    "IPAddress": "172.18.0.6",
                    "IPPrefixLen": 24,
                    "IPv6Gateway": "",
                    "GlobalIPv6Address": "",
                    "GlobalIPv6PrefixLen": 0,
                    "MacAddress": "02:42:ac:12:00:06",
                    "DriverOpts": null
                }
            }
        }
    }
]
```

## 总结

介绍了 inspect 命令的使用，可以查看容器的信息。
