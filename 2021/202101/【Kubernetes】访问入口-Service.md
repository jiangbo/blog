# 【Kubernetes】访问入口-Service

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

在前面副本的扩容与缩容时，我们看到 Pod 的 IP 会变化，导致访问困难。
k8s 提供了 Service 来提供稳定的访问入口，Service 也是 k8s 的一种资源。

## 说明

除了 IP 会随着扩容与缩容变化，在 Pod 重启，Node 出现故障，Pod 被重新调度时，也都会变化。
另一方面，在 Pod 启动之前，IP 是未知的，所以使用 IP 来访问 Pod，不太方便。

## 创建 Service

### 当前状态

```
[root@master kubernetes]# kubectl get pod,rc
NAME                READY   STATUS    RESTARTS   AGE
pod/rc-demo-5tshk   1/1     Running   0          4m35s
pod/rc-demo-8xdp6   1/1     Running   0          4m35s
pod/rc-demo-d95v4   1/1     Running   0          4m35s

NAME                            DESIRED   CURRENT   READY   AGE
replicationcontroller/rc-demo   3         3         3       4m35s
```

### 命令行

`kubectl expose replicationcontroller rc-demo --port 8080`

```
[root@master kubernetes]# kubectl expose replicationcontroller rc-demo --port 8080
service/rc-demo exposed
[root@master kubernetes]# kubectl get all
NAME                READY   STATUS    RESTARTS   AGE
pod/rc-demo-5tshk   1/1     Running   0          3m49s
pod/rc-demo-8xdp6   1/1     Running   0          3m49s
pod/rc-demo-d95v4   1/1     Running   0          3m49s

NAME                            DESIRED   CURRENT   READY   AGE
replicationcontroller/rc-demo   3         3         3       3m49s

NAME                 TYPE        CLUSTER-IP      EXTERNAL-IP   PORT(S)    AGE
service/kubernetes   ClusterIP   10.96.0.1       <none>        443/TCP    6d
service/rc-demo      ClusterIP   10.101.188.48   <none>        8080/TCP   5s
```

### yaml 文件

```yaml
apiVersion: v1
kind: Service
metadata:
  name: svc-demo
spec:
  selector:
    app: myapp
  ports:
    - port: 8080
      targetPort: 8080
```

```
oot@master kubernetes]# kubectl get svc
NAME         TYPE        CLUSTER-IP       EXTERNAL-IP   PORT(S)    AGE
kubernetes   ClusterIP   10.96.0.1        <none>        443/TCP    6d
rc-demo      ClusterIP   10.101.188.48    <none>        8080/TCP   2m59s
svc-demo     ClusterIP   10.106.217.209   <none>        8080/TCP   5s
```

## 访问测试

### 访问命令行创建的

```
[root@master kubernetes]# curl 10.101.188.48:8080/hostname
rc-demo-8xdp6 actuator[root@master kubernetes]# curl 10.101.188.48:8080/hostname
rc-demo-d95v4 actuator[root@master kubernetes]# curl 10.101.188.48:8080/hostname
rc-demo-5tshk actuator[root@master kubernetes]# curl 10.101.188.48:8080/hostname
rc-demo-8xdp6 actuator[root@master kubernetes]# curl 10.101.188.48:8080/hostname
rc-demo-d95v4 actuator[root@master kubernetes]# curl 10.101.188.48:8080/hostname
rc-demo-5tshk actuator[root@master kubernetes]# curl 10.101.188.48:8080/hostname
```

### 访问文件创建的

```
[root@master kubernetes]# curl 10.106.217.209:8080/hostname
rc-demo-5tshk actuator[root@master kubernetes]# curl 10.106.217.209:8080/hostname
rc-demo-8xdp6 actuator[root@master kubernetes]# curl 10.106.217.209:8080/hostname
rc-demo-d95v4 actuator[root@master kubernetes]# curl 10.106.217.209:8080/hostname
rc-demo-5tshk actuator[root@master kubernetes]# curl 10.106.217.209:8080/hostname
rc-demo-8xdp6 actuator[root@master kubernetes]# curl 10.106.217.209:8080/hostname
rc-demo-d95v4 actuator[root@master kubernetes]# curl 10.106.217.209:8080/hostname
rc-demo-5tshk actuator[root@master kubernetes]#
```

### 实现原理

可能已经猜到了 Servide 也是通过标签选择器来选择对应的 Pod，然后把请求转发上去。
在转发的同时，Service 还帮我们自动实现了负载均衡，轮询的方式访问每一个 Pod。

```
c-demo-5tshk actuator[root@master kubernetes]# kubectl get svc -o wide
NAME         TYPE        CLUSTER-IP       EXTERNAL-IP   PORT(S)    AGE     SELECTOR
kubernetes   ClusterIP   10.96.0.1        <none>        443/TCP    6d      <none>
rc-demo      ClusterIP   10.101.188.48    <none>        8080/TCP   7m12s   app=myapp
svc-demo     ClusterIP   10.106.217.209   <none>        8080/TCP   4m18s   app=myapp
```

## 总结

介绍了 Service 统一访问入口，实现的原理。并且通过命令行和文件的两种方式，创建 Service。

## 附录
