# 【Kubernetes】kubeadm 安装集群（一）

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03
4. kubernetes 1.20.2

## 前提条件

准备了 virtual box 6.1 的三台虚拟机，每台虚拟机安装 centos 7.8 的系统。

## 设置主机名和解析

### 设置三台服务的 IP 地址

这里以 NAT+host only 网络为例，外部 IP 可以不关注，host only 网络的 IP 分别为：

1. 192.168.56.101
2. 192.168.56.102
3. 192.168.56.103

其中 101 将作为主节点。配置 /etc/hosts，增加如下内容，三台服务器都需要操作：

```text
192.168.56.101 master
192.168.56.102 node1
192.168.56.103 node2
```

### 设置主机名

将三台主机名分别设置为 master, node1, node2：

master:

```sh
hostnamectl set-hostname master
logout
```

node1:

```sh
hostnamectl set-hostname node1
logout
```

node2:

```sh
hostnamectl set-hostname node2
logout
```
退出重新登录查看登录主机名已经生效

## 系统设置

### 关闭防火墙

```sh
systemctl stop firewalld
systemctl disable firewalld
```

### 关闭 SELinux

```sh
setenforce 0
sed -i 's/^SELINUX=enforcing$/SELINUX=disabled/' /etc/selinux/config
```

### 关闭交换分区

```sh
swapoff -a
# 上面的命令是临时修改，还需要修改 /etc/fstab 关闭 swap 分区，在包含swap那一行最前面加一个“#”。
```

### 加载 br_netfilter 模块

```sh
modprobe br_netfilter
```

### 路由转发

```sh
cat <<EOF | sudo tee /etc/modules-load.d/k8s.conf
br_netfilter
EOF

cat <<EOF | sudo tee /etc/sysctl.d/k8s.conf
net.bridge.bridge-nf-call-ip6tables = 1
net.bridge.bridge-nf-call-iptables = 1
EOF
sudo sysctl --system
```

### 时间同步

```sh
yum install ntpdate -y
ntpdate ntp.aliyun.com
```

## 安装 Docker

Docker 的安装教程可以参考[【Docker】Docker 安装][1]。

### 查看 Docker 全部版本

```sh
yum list docker-ce.x86_64 --showduplicates | sort -r
```

### 安装指定版本

```sh
sudo yum install -y \
  containerd.io-1.2.13 \
  docker-ce-19.03.11 \
  docker-ce-cli-19.03.11
```

### 配置 Docker

```sh
cat <<EOF | sudo tee /etc/docker/daemon.json
{
  "exec-opts": ["native.cgroupdriver=systemd"],
  "log-driver": "json-file",
  "log-opts": {
    "max-size": "100m"
  },
  "registry-mirrors": ["https://hub-mirror.c.163.com","https://0kzec4yw.mirror.aliyuncs.com"],
  "storage-driver": "overlay2",
  "storage-opts": [
    "overlay2.override_kernel_check=true"
  ]
}
EOF
```

## Docker 降级

Kubernetes 在 Docker 19.03 上测试通过，所以如果安装了最新的版本的话，可以选择降级。

```sh
yum downgrade docker-ce-19.03.11-3.el7 docker-ce-cli-19.03.11-3.el7 -y
```

### 启动 Docker

```sh
sudo mkdir -p /etc/systemd/system/docker.service.d
sudo systemctl daemon-reload
sudo systemctl restart docker
sudo systemctl enable docker
```

## 总结

介绍了安装 Kubernetes 前的一些准备工作，包括系统设置和 Docker 的安装配置。

[1]: https://www.cnblogs.com/jiangbo44/p/13863519.html