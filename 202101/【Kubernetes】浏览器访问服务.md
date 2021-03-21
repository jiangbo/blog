# 【Kubernetes】浏览器访问服务

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

通过浏览器访问 Service，并访问到后端 Pod 提供的服务。

## 查看 Service

```
[root@master kubernetes]# kubectl get service
NAME         TYPE        CLUSTER-IP       EXTERNAL-IP   PORT(S)    AGE
kubernetes   ClusterIP   10.96.0.1        <none>        443/TCP    6d1h
svc-demo     ClusterIP   10.106.217.209   <none>        8080/TCP   40m
```

在 TYPE 一栏，我们看到类型是 ClusterIP，表示是集群 IP，不能通过外部访问。
之前我们在外面使用命令行访问到了，又是怎么回事了呢？

原因是因为安装了 k8s 这三台服务器，具有路由转发规则，如果是集群内部的 IP，
会被转发到 k8s 的内部，所以才能访问。如果我在宿主机上进行访问，那么是访问不到的。

## NodePort

Service 除了默认的 ClusterIP（不写）外，接下来介绍另外一种类型：NodePort。
在服务器上打开一个端口，通过该端口可以访问 k8s 的 Service。
该种方式可以实现，在 Windows 的浏览器上访问虚拟机里提供的服务。

### yaml

```yaml
apiVersion: v1
kind: Service
metadata:
  name: svc-demo
spec:
  type: NodePort
  selector:
    app: myapp
  ports:
    - port: 8080
      targetPort: 8080
```

可以看到新增了一个 type 字段，并且设置为 NodePort。

### 查看 Service

```
[root@master kubernetes]# kubectl get -f svc.yaml
NAME       TYPE       CLUSTER-IP       EXTERNAL-IP   PORT(S)          AGE
svc-demo   NodePort   10.106.217.209   <none>        8080:32329/TCP   66m
[root@master kubernetes]#
```

可以看到额外生成了一个端口：32329，通过这个端口，在 Windows 上访问试试。

## 访问测试

### 浏览器访问

![nodeport.png][1]

### 命令行访问

```
PS C:\Users\JiangBo> curl http://192.168.56.101:32329/hostname
rc-demo-5tshk actuator
PS C:\Users\JiangBo> curl http://192.168.56.101:32329/hostname
rc-demo-8xdp6 actuator
PS C:\Users\JiangBo> curl http://192.168.56.101:32329/hostname
rc-demo-d95v4 actuator
PS C:\Users\JiangBo> curl http://192.168.56.101:32329/hostname
rc-demo-5tshk actuator
PS C:\Users\JiangBo>
```

可以看到 curl 访问时，每个 Pod 轮询访问。但是如果在浏览器上访问，会看到一直是一个。
除了稍等久一会访问，才会看到新的。这是因为浏览器为了减少建立连接的时间，会重用连接。
在建立连接之后，如果没有断开，那么将一直是这个 Pod。

## 总结

介绍了通过外部的机器来访问 k8s 中的服务，包括浏览器和命令行的方式。

[1]: images/nodeport.png

## 附录
