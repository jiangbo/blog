# 【Docker】Docker 安装

以下内容基于虚拟机 virtual box 中的 centos 7 进行 docker 的安装。
安装教程参考阿里云，教程地址：https://developer.aliyun.com/mirror/docker-ce

## 环境

1. virtual box 6.1
2. centos 7.8

## 安装系统工具

执行命令 `sudo yum install -y yum-utils device-mapper-persistent-data lvm2`

```sh
[root@10 ~]# sudo yum install -y yum-utils device-mapper-persistent-data lvm2
Loaded plugins: fastestmirror
Loading mirror speeds from cached hostfile
 * base: mirrors.aliyun.com
 * extras: mirrors.aliyun.com
 * updates: mirrors.aliyun.com
Package device-mapper-persistent-data-0.8.5-2.el7.x86_64 already installed and latest version
Package 7:lvm2-2.02.186-7.el7_8.2.x86_64 already installed and latest version
Resolving Dependencies
--> Running transaction check
---> Package yum-utils.noarch 0:1.1.31-54.el7_8 will be installed
--> Processing Dependency: python-kitchen for package: yum-utils-1.1.31-54.el7_8.noarch
--> Processing Dependency: libxml2-python for package: yum-utils-1.1.31-54.el7_8.noarch
--> Running transaction check
---> Package libxml2-python.x86_64 0:2.9.1-6.el7.4 will be installed
---> Package python-kitchen.noarch 0:1.1.1-5.el7 will be installed
--> Processing Dependency: python-chardet for package: python-kitchen-1.1.1-5.el7.noarch
--> Running transaction check
---> Package python-chardet.noarch 0:2.2.1-3.el7 will be installed
--> Finished Dependency Resolution

Dependencies Resolved

==============================================================================================================================================================
 Package                                  Arch                             Version                                    Repository                         Size
==============================================================================================================================================================
Installing:
 yum-utils                                noarch                           1.1.31-54.el7_8                            updates                           122 k
Installing for dependencies:
 libxml2-python                           x86_64                           2.9.1-6.el7.4                              base                              247 k
 python-chardet                           noarch                           2.2.1-3.el7                                base                              227 k
 python-kitchen                           noarch                           1.1.1-5.el7                                base                              267 k

Transaction Summary
==============================================================================================================================================================
Install  1 Package (+3 Dependent packages)

Total download size: 862 k
Installed size: 4.3 M
Downloading packages:
(1/4): libxml2-python-2.9.1-6.el7.4.x86_64.rpm                                                                                         | 247 kB  00:00:00
(2/4): python-kitchen-1.1.1-5.el7.noarch.rpm                                                                                           | 267 kB  00:00:00
(3/4): yum-utils-1.1.31-54.el7_8.noarch.rpm                                                                                            | 122 kB  00:00:00
(4/4): python-chardet-2.2.1-3.el7.noarch.rpm                                                                                           | 227 kB  00:00:00
--------------------------------------------------------------------------------------------------------------------------------------------------------------
Total                                                                                                                         1.3 MB/s | 862 kB  00:00:00
Running transaction check
Running transaction test
Transaction test succeeded
Running transaction
  Installing : python-chardet-2.2.1-3.el7.noarch                                                                                                          1/4
  Installing : python-kitchen-1.1.1-5.el7.noarch                                                                                                          2/4
  Installing : libxml2-python-2.9.1-6.el7.4.x86_64                                                                                                        3/4
  Installing : yum-utils-1.1.31-54.el7_8.noarch                                                                                                           4/4
  Verifying  : libxml2-python-2.9.1-6.el7.4.x86_64                                                                                                        1/4
  Verifying  : python-kitchen-1.1.1-5.el7.noarch                                                                                                          2/4
  Verifying  : yum-utils-1.1.31-54.el7_8.noarch                                                                                                           3/4
  Verifying  : python-chardet-2.2.1-3.el7.noarch                                                                                                          4/4

Installed:
  yum-utils.noarch 0:1.1.31-54.el7_8

Dependency Installed:
  libxml2-python.x86_64 0:2.9.1-6.el7.4                python-chardet.noarch 0:2.2.1-3.el7                python-kitchen.noarch 0:1.1.1-5.el7

Complete!
```

## 添加软件源信息

执行命令 `sudo yum-config-manager --add-repo https://mirrors.aliyun.com/docker-ce/linux/centos/docker-ce.repo`

```sh
[root@10 ~]# sudo yum-config-manager --add-repo https://mirrors.aliyun.com/docker-ce/linux/centos/docker-ce.repo
Loaded plugins: fastestmirror
adding repo from: https://mirrors.aliyun.com/docker-ce/linux/centos/docker-ce.repo
grabbing file https://mirrors.aliyun.com/docker-ce/linux/centos/docker-ce.repo to /etc/yum.repos.d/docker-ce.repo
repo saved to /etc/yum.repos.d/docker-ce.repo
```

## 更新

执行命令 `sudo yum makecache fast`

```sh
[root@10 ~]# sudo yum makecache fast
Loaded plugins: fastestmirror
Loading mirror speeds from cached hostfile
 * base: mirrors.aliyun.com
 * extras: mirrors.aliyun.com
 * updates: mirrors.aliyun.com
base                                                                                                                                   | 3.6 kB  00:00:00
docker-ce-stable                                                                                                                       | 3.5 kB  00:00:00
extras                                                                                                                                 | 2.9 kB  00:00:00
updates                                                                                                                                | 2.9 kB  00:00:00
(1/2): docker-ce-stable/x86_64/updateinfo                                                                                              |   55 B  00:00:00
(2/2): docker-ce-stable/x86_64/primary_db                                                                                              |  45 kB  00:00:00
Metadata Cache Created
```

## 安装 docker

执行命令 `sudo yum -y install docker-ce`

```sh
[root@10 ~]# sudo yum -y install docker-ce
Loaded plugins: fastestmirror
Loading mirror speeds from cached hostfile
 * base: mirrors.aliyun.com
 * extras: mirrors.aliyun.com
 * updates: mirrors.aliyun.com
Resolving Dependencies
--> Running transaction check
---> Package docker-ce.x86_64 3:19.03.12-3.el7 will be installed
--> Processing Dependency: container-selinux >= 2:2.74 for package: 3:docker-ce-19.03.12-3.el7.x86_64
--> Processing Dependency: containerd.io >= 1.2.2-3 for package: 3:docker-ce-19.03.12-3.el7.x86_64
--> Processing Dependency: docker-ce-cli for package: 3:docker-ce-19.03.12-3.el7.x86_64
--> Processing Dependency: libcgroup for package: 3:docker-ce-19.03.12-3.el7.x86_64
--> Running transaction check
---> Package container-selinux.noarch 2:2.119.2-1.911c772.el7_8 will be installed
--> Processing Dependency: policycoreutils-python for package: 2:container-selinux-2.119.2-1.911c772.el7_8.noarch
---> Package containerd.io.x86_64 0:1.2.13-3.2.el7 will be installed
---> Package docker-ce-cli.x86_64 1:19.03.12-3.el7 will be installed
---> Package libcgroup.x86_64 0:0.41-21.el7 will be installed
--> Running transaction check
---> Package policycoreutils-python.x86_64 0:2.5-34.el7 will be installed
--> Processing Dependency: setools-libs >= 3.3.8-4 for package: policycoreutils-python-2.5-34.el7.x86_64
--> Processing Dependency: libsemanage-python >= 2.5-14 for package: policycoreutils-python-2.5-34.el7.x86_64
--> Processing Dependency: audit-libs-python >= 2.1.3-4 for package: policycoreutils-python-2.5-34.el7.x86_64
--> Processing Dependency: python-IPy for package: policycoreutils-python-2.5-34.el7.x86_64
--> Processing Dependency: libqpol.so.1(VERS_1.4)(64bit) for package: policycoreutils-python-2.5-34.el7.x86_64
--> Processing Dependency: libqpol.so.1(VERS_1.2)(64bit) for package: policycoreutils-python-2.5-34.el7.x86_64
--> Processing Dependency: libapol.so.4(VERS_4.0)(64bit) for package: policycoreutils-python-2.5-34.el7.x86_64
--> Processing Dependency: checkpolicy for package: policycoreutils-python-2.5-34.el7.x86_64
--> Processing Dependency: libqpol.so.1()(64bit) for package: policycoreutils-python-2.5-34.el7.x86_64
--> Processing Dependency: libapol.so.4()(64bit) for package: policycoreutils-python-2.5-34.el7.x86_64
--> Running transaction check
---> Package audit-libs-python.x86_64 0:2.8.5-4.el7 will be installed
---> Package checkpolicy.x86_64 0:2.5-8.el7 will be installed
---> Package libsemanage-python.x86_64 0:2.5-14.el7 will be installed
---> Package python-IPy.noarch 0:0.75-6.el7 will be installed
---> Package setools-libs.x86_64 0:3.3.8-4.el7 will be installed
--> Finished Dependency Resolution

Dependencies Resolved

==============================================================================================================================================================
 Package                                   Arch                      Version                                        Repository                           Size
==============================================================================================================================================================
Installing:
 docker-ce                                 x86_64                    3:19.03.12-3.el7                               docker-ce-stable                     24 M
Installing for dependencies:
 audit-libs-python                         x86_64                    2.8.5-4.el7                                    base                                 76 k
 checkpolicy                               x86_64                    2.5-8.el7                                      base                                295 k
 container-selinux                         noarch                    2:2.119.2-1.911c772.el7_8                      extras                               40 k
 containerd.io                             x86_64                    1.2.13-3.2.el7                                 docker-ce-stable                     25 M
 docker-ce-cli                             x86_64                    1:19.03.12-3.el7                               docker-ce-stable                     38 M
 libcgroup                                 x86_64                    0.41-21.el7                                    base                                 66 k
 libsemanage-python                        x86_64                    2.5-14.el7                                     base                                113 k
 policycoreutils-python                    x86_64                    2.5-34.el7                                     base                                457 k
 python-IPy                                noarch                    0.75-6.el7                                     base                                 32 k
 setools-libs                              x86_64                    3.3.8-4.el7                                    base                                620 k

Transaction Summary
==============================================================================================================================================================
Install  1 Package (+10 Dependent packages)

Total download size: 89 M
Installed size: 365 M
Downloading packages:
(1/11): audit-libs-python-2.8.5-4.el7.x86_64.rpm                                                                                       |  76 kB  00:00:00
(2/11): container-selinux-2.119.2-1.911c772.el7_8.noarch.rpm                                                                           |  40 kB  00:00:00
(3/11): checkpolicy-2.5-8.el7.x86_64.rpm                                                                                               | 295 kB  00:00:00
warning: /var/cache/yum/x86_64/7/docker-ce-stable/packages/docker-ce-19.03.12-3.el7.x86_64.rpm: Header V4 RSA/SHA512 Signature, key ID 621e9f35: NOKEY:05 ETA
Public key for docker-ce-19.03.12-3.el7.x86_64.rpm is not installed
(4/11): docker-ce-19.03.12-3.el7.x86_64.rpm                                                                                            |  24 MB  00:00:04
(5/11): containerd.io-1.2.13-3.2.el7.x86_64.rpm                                                                                        |  25 MB  00:00:05
(6/11): libsemanage-python-2.5-14.el7.x86_64.rpm                                                                                       | 113 kB  00:00:00
(7/11): libcgroup-0.41-21.el7.x86_64.rpm                                                                                               |  66 kB  00:00:00
(8/11): policycoreutils-python-2.5-34.el7.x86_64.rpm                                                                                   | 457 kB  00:00:00
(9/11): python-IPy-0.75-6.el7.noarch.rpm                                                                                               |  32 kB  00:00:00
(10/11): setools-libs-3.3.8-4.el7.x86_64.rpm                                                                                           | 620 kB  00:00:00
(11/11): docker-ce-cli-19.03.12-3.el7.x86_64.rpm                                                                                       |  38 MB  00:00:15
--------------------------------------------------------------------------------------------------------------------------------------------------------------
Total                                                                                                                         4.4 MB/s |  89 MB  00:00:20
Retrieving key from https://mirrors.aliyun.com/docker-ce/linux/centos/gpg
Importing GPG key 0x621E9F35:
 Userid     : "Docker Release (CE rpm) <docker@docker.com>"
 Fingerprint: 060a 61c5 1b55 8a7f 742b 77aa c52f eb6b 621e 9f35
 From       : https://mirrors.aliyun.com/docker-ce/linux/centos/gpg
Running transaction check
Running transaction test
Transaction test succeeded
Running transaction
  Installing : libcgroup-0.41-21.el7.x86_64                                                                                                              1/11
  Installing : audit-libs-python-2.8.5-4.el7.x86_64                                                                                                      2/11
  Installing : setools-libs-3.3.8-4.el7.x86_64                                                                                                           3/11
  Installing : 1:docker-ce-cli-19.03.12-3.el7.x86_64                                                                                                     4/11
  Installing : python-IPy-0.75-6.el7.noarch                                                                                                              5/11
  Installing : libsemanage-python-2.5-14.el7.x86_64                                                                                                      6/11
  Installing : checkpolicy-2.5-8.el7.x86_64                                                                                                              7/11
  Installing : policycoreutils-python-2.5-34.el7.x86_64                                                                                                  8/11
  Installing : 2:container-selinux-2.119.2-1.911c772.el7_8.noarch                                                                                        9/11
  Installing : containerd.io-1.2.13-3.2.el7.x86_64                                                                                                      10/11
  Installing : 3:docker-ce-19.03.12-3.el7.x86_64                                                                                                        11/11
  Verifying  : checkpolicy-2.5-8.el7.x86_64                                                                                                              1/11
  Verifying  : libsemanage-python-2.5-14.el7.x86_64                                                                                                      2/11
  Verifying  : containerd.io-1.2.13-3.2.el7.x86_64                                                                                                       3/11
  Verifying  : 2:container-selinux-2.119.2-1.911c772.el7_8.noarch                                                                                        4/11
  Verifying  : python-IPy-0.75-6.el7.noarch                                                                                                              5/11
  Verifying  : policycoreutils-python-2.5-34.el7.x86_64                                                                                                  6/11
  Verifying  : 1:docker-ce-cli-19.03.12-3.el7.x86_64                                                                                                     7/11
  Verifying  : 3:docker-ce-19.03.12-3.el7.x86_64                                                                                                         8/11
  Verifying  : setools-libs-3.3.8-4.el7.x86_64                                                                                                           9/11
  Verifying  : audit-libs-python-2.8.5-4.el7.x86_64                                                                                                     10/11
  Verifying  : libcgroup-0.41-21.el7.x86_64                                                                                                             11/11

Installed:
  docker-ce.x86_64 3:19.03.12-3.el7

Dependency Installed:
  audit-libs-python.x86_64 0:2.8.5-4.el7         checkpolicy.x86_64 0:2.5-8.el7                     container-selinux.noarch 2:2.119.2-1.911c772.el7_8
  containerd.io.x86_64 0:1.2.13-3.2.el7          docker-ce-cli.x86_64 1:19.03.12-3.el7              libcgroup.x86_64 0:0.41-21.el7
  libsemanage-python.x86_64 0:2.5-14.el7         policycoreutils-python.x86_64 0:2.5-34.el7         python-IPy.noarch 0:0.75-6.el7
  setools-libs.x86_64 0:3.3.8-4.el7

Complete!
```

## 启动

执行命令 `sudo service docker start`

```sh
[root@10 ~]# sudo service docker start
Redirecting to /bin/systemctl start docker.service
```

## 验证

执行命令 `docker version`

```sh
[root@10 ~]# docker version
Client: Docker Engine - Community
 Version:           19.03.12
 API version:       1.40
 Go version:        go1.13.10
 Git commit:        48a66213fe
 Built:             Mon Jun 22 15:46:54 2020
 OS/Arch:           linux/amd64
 Experimental:      false

Server: Docker Engine - Community
 Engine:
  Version:          19.03.12
  API version:      1.40 (minimum version 1.12)
  Go version:       go1.13.10
  Git commit:       48a66213fe
  Built:            Mon Jun 22 15:45:28 2020
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