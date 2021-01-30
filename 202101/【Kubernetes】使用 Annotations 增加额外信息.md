# 【Kubernetes】使用 Annotations 增加额外信息

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

使用 Annotations 来额外描述资源信息。

## Annotations

Annotations 这里翻译成注释，是用来增加额外的描述信息，相当于代码中的注释。

### Pod.yaml

创建一个带有 Annotations 的 Pod。

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: pod3
  annotations:
    author: jiangbo
    version: actuator
spec:
  containers:
    - name: pod3
      image: jiangbo920827/spring-demo:actuator
      ports:
        - containerPort: 8080
      resources:
        limits:
          memory: 200Mi
          cpu: 200m

```

### 查看 Pod 状态

```
[root@master kubernetes]# kubectl describe pod pod3
Name:         pod3
Namespace:    default
Priority:     0
Node:         node1/192.168.56.102
Start Time:   Sat, 30 Jan 2021 12:57:41 +0800
Labels:       <none>
Annotations:  author: jiangbo
              version: actuator
Status:       Running
IP:           10.244.1.76
IPs:
  IP:  10.244.1.76
Containers:
...
```

### 说明

一般来说，注释由工具自动生成的。但是我们也可以自己添加自己需要的注释。

### 查看 Node 注释

```
[root@master ~]# kubectl describe nodes master
Name:               master
Roles:              control-plane,master
Labels:             beta.kubernetes.io/arch=amd64
                    beta.kubernetes.io/os=linux
                    kubernetes.io/arch=amd64
                    kubernetes.io/hostname=master
                    kubernetes.io/os=linux
                    node-role.kubernetes.io/control-plane=
                    node-role.kubernetes.io/master=
Annotations:        flannel.alpha.coreos.com/backend-data: {"VNI":1,"VtepMAC":"56:62:2e:01:9d:b3"}
                    flannel.alpha.coreos.com/backend-type: vxlan
                    flannel.alpha.coreos.com/kube-subnet-manager: true
                    flannel.alpha.coreos.com/public-ip: 192.168.56.101
                    kubeadm.alpha.kubernetes.io/cri-socket: /var/run/dockershim.sock
                    node.alpha.kubernetes.io/ttl: 0
                    volumes.kubernetes.io/controller-managed-attach-detach: true
CreationTimestamp:  Sun, 24 Jan 2021 15:44:44 +0800
Taints:             node-role.kubernetes.io/master:NoSchedule
Unschedulable:      false
Lease:
  HolderIdentity:  master
  AcquireTime:     <unset>
  RenewTime:       Sat, 30 Jan 2021 12:54:00 +0800
...
```

其它不相关的信息未显示。

## 总结

介绍了注释的作用和怎么添加注释。

## 附录
