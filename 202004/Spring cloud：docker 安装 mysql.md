# Spring cloud：docker 安装 mysql

## 环境

1. spring cloud Edgware.SR6
2. jdk 7
3. sts 4.6.0
4. docker 19.03.8
5. mysql 5.7

## 背景

使用数据库来持久化微服务产生的数据。

## 搭建步骤

在搭建之前，需要保证 docker 环境已经安装好了。如果没有，可以参考 [Docker：在 wsl2 上安装 docker][1]

### 查看 docker 版本

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

Server: Docker Engine - Community
 Engine:
  Version:          19.03.8
  API version:      1.40 (minimum version 1.12)
  Go version:       go1.12.17
  Git commit:       afacb8b7f0
  Built:            Wed Mar 11 01:24:19 2020
  OS/Arch:          linux/amd64
  Experimental:     false
 containerd:
  Version:          1.2.13
  GitCommit:        7ad184331fa3e55e52b890ea95e65ba581ae3429
 runc:
  Version:          1.0.0-rc10
  GitCommit:        dc9208a3303feef5b3839f4323d9beb36df0a9dd
 docker-init:
  Version:          0.18.0
  GitCommit:        fec3683
```

### 拉取镜像

```shell
jiangbo@jiangbo-work:~$ docker pull mysql:5.7
5.7: Pulling from library/mysql
Digest: sha256:fbaeced79cfdae5d3c8d4a8c41e883f254f72ed7428c6b93a498824b76d97121
Status: Image is up to date for mysql:5.7
docker.io/library/mysql:5.7
```

### 启动 mysql

```shell
docker run --name mysql -e MYSQL_ROOT_PASSWORD=jiangbo -d mysql:5.7
```

其中 MYSQL_ROOT_PASSWORD 后的参数是表示 root 账号的密码，可以根据实际情况修改。


### 检查连接

检查 mysql 是否可以连接，这里使用 DBeaver 来进行连接测试。

![spring-cloud-mysql][2]



[1]:https://www.cnblogs.com/jiangbo44/p/12637389.html
[2]:../../images/spring-cloud-mysql.png