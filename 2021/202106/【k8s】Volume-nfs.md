# 【k8s】Volume-nfs

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-RC1

## 目标

nfs 是一种网络文件存储，可以不和节点绑定，也可以在 node 节点之外，只要网络可达就可以。

## 示例

### 设置 nfs 环境

每个 node 上都需要安装 nfs-utils 工具包 `yum install -y nfs-utils`。

以下的操作都在 master（192.168.56.101） 节点上操作

nfs 配置文件

```
[root@master ~]# cat /etc/exports.d/k8s.exports
/opt/nfs    *(rw)
```

启动 nfs 服务

```
[root@master ~]# systemctl start nfs-server
[root@master ~]# systemctl status nfs-server
● nfs-server.service - NFS server and services
   Loaded: loaded (/usr/lib/systemd/system/nfs-server.service; disabled; vendor preset: disabled)
  Drop-In: /run/systemd/generator/nfs-server.service.d
           └─order-with-mounts.conf
   Active: active (exited) since Tue 2021-05-18 23:04:29 HKT; 7s ago
  Process: 9589 ExecStartPost=/bin/sh -c if systemctl -q is-active gssproxy; then systemctl reload gssproxy ; fi (code=exited, status=0/SUCCESS)
  Process: 9574 ExecStart=/usr/sbin/rpc.nfsd $RPCNFSDARGS (code=exited, status=0/SUCCESS)
  Process: 9573 ExecStartPre=/usr/sbin/exportfs -r (code=exited, status=0/SUCCESS)
 Main PID: 9574 (code=exited, status=0/SUCCESS)
    Tasks: 0
   Memory: 0B
   CGroup: /system.slice/nfs-server.service

May 18 23:04:28 master systemd[1]: Starting NFS server and services...
May 18 23:04:29 master systemd[1]: Started NFS server and services.
```

### Pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
spec:
  containers:
    - name: spring-k8s
      image: jiangbo920827/spring-k8s:liveness
      volumeMounts:
        - mountPath: /opt/volume/nfs
          name: v1
      ports:
        - containerPort: 8080
  volumes:
    - name: v1
      nfs:
        path: /opt/nfs
        server: 192.168.56.101
```

### 查看

```
[root@master nfs]# kubectl get pod -o wide
NAME         READY   STATUS    RESTARTS   AGE    IP            NODE    NOMINATED NODE   READINESS GATES
spring-k8s   1/1     Running   0          103s   10.244.1.14   node1   <none>           <none>
[root@master nfs]# kubectl exec spring-k8s -- sh -c "echo 'jiangbo'>/opt/volume/nfs/name.txt"
[root@master nfs]# cat /opt/nfs/name.txt
jiangbo
```

容器被调度到了 node1 节点，但是还是可以向 master 节点的 nfs 目录写入。

## 总结

nfs 可以定义一个网络文件存储，无论 Pod 被调度到哪个节点，都可以进行读写。

## 附录
