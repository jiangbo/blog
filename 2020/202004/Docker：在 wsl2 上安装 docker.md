# Docker：在 wsl2 上安装 docker

## 环境

1. wondwos 10
2. wsl 2
3. ubuntu 18.04

## 背景

从 wsl 2 开始，可以直接在 windows 上安装 docker，以下基于 windwos 10 环境安装，由于国外安装速度太慢了，使用阿里云的镜像进行安装。

阿里云镜像安装的教程参考：https://developer.aliyun.com/mirror/docker-ce ，可以根据不同和系统和需求进行选择。

## 安装 docker

### 更新 apt

```shell
jiangbo@jiangbo-work:~$ sudo apt-get update
Hit:1 http://mirrors.aliyun.com/ubuntu bionic InRelease
Get:2 http://mirrors.aliyun.com/ubuntu bionic-security InRelease [88.7 kB]
Hit:3 https://mirrors.aliyun.com/docker-ce/linux/ubuntu bionic InRelease
Get:4 http://mirrors.aliyun.com/ubuntu bionic-updates InRelease [88.7 kB]
Get:5 http://mirrors.aliyun.com/ubuntu bionic-proposed InRelease [242 kB]
Get:6 http://mirrors.aliyun.com/ubuntu bionic-backports InRelease [74.6 kB]
Get:7 http://mirrors.aliyun.com/ubuntu bionic-security/main Sources [146 kB]
Get:8 http://mirrors.aliyun.com/ubuntu bionic-security/main amd64 Packages [677 kB]
Get:9 http://mirrors.aliyun.com/ubuntu bionic-security/universe amd64 Packages [653 kB]
Get:10 http://mirrors.aliyun.com/ubuntu bionic-updates/main Sources [314 kB]
Get:11 http://mirrors.aliyun.com/ubuntu bionic-updates/main amd64 Packages [897 kB]
Get:12 http://mirrors.aliyun.com/ubuntu bionic-updates/universe amd64 Packages [1061 kB]
Fetched 4242 kB in 1s (3335 kB/s)
Reading package lists... Done
```

### 安装必要的系统工具

```shell
jiangbo@jiangbo-work:~$ sudo apt-get -y install apt-transport-https ca-certificates curl software-properties-common
Reading package lists... Done
Building dependency tree
Reading state information... Done
ca-certificates is already the newest version (20180409).
curl is already the newest version (7.58.0-2ubuntu3.8).
software-properties-common is already the newest version (0.96.24.32.12).
apt-transport-https is already the newest version (1.6.12).
The following packages were automatically installed and are no longer required:
  aufs-tools cgroupfs-mount containerd.io docker-ce-cli libltdl7 pigz
Use 'sudo apt autoremove' to remove them.
0 upgraded, 0 newly installed, 0 to remove and 151 not upgraded.
```

### 安装GPG证书

```shell
jiangbo@jiangbo-work:~$ curl -fsSL https://mirrors.aliyun.com/docker-ce/linux/ubuntu/gpg | sudo apt-key add -
OK
```

### 写入软件源信息并更新

```shell
jiangbo@jiangbo-work:~$ sudo add-apt-repository "deb [arch=amd64] https://mirrors.aliyun.com/docker-ce/linux/ubuntu $(lsb_release -cs) stable"
Hit:1 http://mirrors.aliyun.com/ubuntu bionic InRelease
Hit:2 http://mirrors.aliyun.com/ubuntu bionic-security InRelease
Hit:3 http://mirrors.aliyun.com/ubuntu bionic-updates InRelease
Hit:4 http://mirrors.aliyun.com/ubuntu bionic-proposed InRelease
Hit:5 http://mirrors.aliyun.com/ubuntu bionic-backports InRelease
Hit:6 https://mirrors.aliyun.com/docker-ce/linux/ubuntu bionic InRelease
Reading package lists... Done
jiangbo@jiangbo-work:~$ sudo apt-get -y update
Hit:1 http://mirrors.aliyun.com/ubuntu bionic InRelease
Hit:2 http://mirrors.aliyun.com/ubuntu bionic-security InRelease
Hit:3 https://mirrors.aliyun.com/docker-ce/linux/ubuntu bionic InRelease
Hit:4 http://mirrors.aliyun.com/ubuntu bionic-updates InRelease
Hit:5 http://mirrors.aliyun.com/ubuntu bionic-proposed InRelease
Hit:6 http://mirrors.aliyun.com/ubuntu bionic-backports InRelease
Reading package lists... Done
```

### 安装 docker

```shell
jiangbo@jiangbo-work:~$ sudo apt-get -y install docker-ce
Reading package lists... Done
Building dependency tree
Reading state information... Done
The following NEW packages will be installed:
  docker-ce
0 upgraded, 1 newly installed, 0 to remove and 151 not upgraded.
Need to get 22.9 MB of archives.
After this operation, 109 MB of additional disk space will be used.
Get:1 https://mirrors.aliyun.com/docker-ce/linux/ubuntu bionic/stable amd64 docker-ce amd64 5:19.03.8~3-0~ubuntu-bionic [22.9 MB]
Fetched 22.9 MB in 2s (9261 kB/s)
Selecting previously unselected package docker-ce.
(Reading database ... 31855 files and directories currently installed.)
Preparing to unpack .../docker-ce_5%3a19.03.8~3-0~ubuntu-bionic_amd64.deb ...
Unpacking docker-ce (5:19.03.8~3-0~ubuntu-bionic) ...
Setting up docker-ce (5:19.03.8~3-0~ubuntu-bionic) ...
invoke-rc.d: could not determine current runlevel
Processing triggers for ureadahead (0.100.0-21) ...
Processing triggers for systemd (237-3ubuntu10.21) ...
```

### 确认安装完成

```shell
jiangbo@jiangbo-work:~$ docker version
Client: Docker Engine - Community
 Version:           19.03.7
 API version:       1.40
 Go version:        go1.12.17
 Git commit:        7141c199a2
 Built:             Wed Mar  4 01:22:36 2020
 OS/Arch:           linux/amd64
 Experimental:      false
Cannot connect to the Docker daemon at unix:///var/run/docker.sock. Is the docker daemon running?
```

## 启动并配置 docker

### 启动 docker

```
jiangbo@jiangbo-work:~$ sudo service docker start
 * Starting Docker: docker
```

### 加入 docker 用户组

如果没有将当前用户加入 docker 用户组，则直接使用 docker ps 会得到下面的错误：

```shell
jiangbo@jiangbo-work:~$ docker ps
Got permission denied while trying to connect to the Docker daemon socket at unix:///var/run/docker.sock: Get http://%2Fvar%2Frun%2Fdocker.sock/v1.40/containers/json: dial unix /var/run/docker.sock: connect: permission denied
```

使用 sudo docker ps 则可以看到正常的结果：

```shell
jiangbo@jiangbo-work:~$ sudo docker ps
CONTAINER ID        IMAGE               COMMAND             CREATED             STATUS              PORTS               NAMES
```

为了方便，不用每次都加上 sudo，执行以下命令：

```
jiangbo@jiangbo-work:~$ sudo gpasswd -a $USER  docker
Adding user jiangbo to group docker
jiangbo@jiangbo-work:~$ groups
jiangbo adm dialout cdrom floppy sudo audio dip video plugdev lxd netdev
```
现在在用户组中还看不到 docker，需要重新连接一下。

### 重启 wsl

只要断开 bash，重新连接 wsl，则可以刷新当前的用户信息。断开重连，查看当前的用户组信息：

```shell
jiangbo@jiangbo-work:~$ groups
jiangbo adm dialout cdrom floppy sudo audio dip video plugdev lxd netdev docker
```
可以看到，已经加入到了 docker 用户组了。

### docker ps

```shell
jiangbo@jiangbo-work:~$ docker ps
CONTAINER ID        IMAGE               COMMAND             CREATED             STATUS              PORTS               NAMES
```

### 配置镜像加速器

由于镜像默认是到国外去下载，特别慢，需要配置国内的镜像加速器，增加下载速度。
在 /etc/docker/ 目录下，增加 daemon.json 文件，并加入以下内容：

```json
{
  "registry-mirrors": [
    "https://registry.docker-cn.com",
    "http://hub-mirror.c.163.com",
    "https://docker.mirrors.ustc.edu.cn"
  ]
}
```

重启 docker 服务，使用 sudo service docker restart 命令。

## 测试 docker

### 启动 nginx 服务

```
jiangbo@jiangbo-work:~$ docker run -d -p8080:80 nginx
Unable to find image 'nginx:latest' locally
latest: Pulling from library/nginx
c499e6d256d6: Pull complete
74cda408e262: Pull complete
ffadbd415ab7: Pull complete
Digest: sha256:282530fcb7cd19f3848c7b611043f82ae4be3781cb00105a1d593d7e6286b596
Status: Downloaded newer image for nginx:latest
03ebe5b8de21837e426b29de62407e22607e415de623e520737043f7f02fa3da
```

### 访问

浏览器访问 localhost:8080，可以看到下面的内容，表示已成功启动 nginx，docker 环境安装完成。

```text
Welcome to nginx!
If you see this page, the nginx web server is successfully installed and working. Further configuration is required.

For online documentation and support please refer to nginx.org.
Commercial support is available at nginx.com.

Thank you for using nginx.
```



