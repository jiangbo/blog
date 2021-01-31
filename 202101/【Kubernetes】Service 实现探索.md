# 【Kubernetes】Service 实现探索

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

通过查看 Service 的详细信息，知道 Service 和 Pod 之间还有一层 Endpoints 资源。

## 查看 Service 信息

```
[root@master kubernetes]# kubectl describe service svc-demo
Name:                     svc-demo
Namespace:                default
Labels:                   <none>
Annotations:              <none>
Selector:                 app=myapp
Type:                     NodePort
IP Families:              <none>
IP:                       10.106.217.209
IPs:                      10.106.217.209
Port:                     <unset>  8080/TCP
TargetPort:               8080/TCP
NodePort:                 <unset>  32329/TCP
Endpoints:                10.244.1.87:8080,10.244.1.88:8080,10.244.2.44:8080
Session Affinity:         None
External Traffic Policy:  Cluster
Events:                   <none>
[root@master kubernetes]#
```

其中的 Endpoints 字段，就表示 Endpoints 资源。

### 查看 Endpoints

```
[root@master kubernetes]# kubectl get endpoints
NAME         ENDPOINTS                                            AGE
kubernetes   192.168.56.101:6443                                  6d2h
svc-demo     10.244.1.87:8080,10.244.1.88:8080,10.244.2.44:8080   115m
[root@master kubernetes]#
```

其中的 kubernetes 是自带的，先不用关心。对于其中的 svc-demo，很明显就是 Service 中的 Endpoints 字段。
修改以下 Pod 的数量，来看看 Endpoints 的变化。

### 扩容到 5 个

```
[root@master kubernetes]# kubectl scale --replicas=5 replicationcontroller rc-demo
replicationcontroller/rc-demo scaled
[root@master kubernetes]# kubectl describe endpoints svc-demo
Name:         svc-demo
Namespace:    default
Labels:       <none>
Annotations:  endpoints.kubernetes.io/last-change-trigger-time: 2021-01-30T12:06:29Z
Subsets:
  Addresses:          10.244.1.87,10.244.1.88,10.244.1.89,10.244.2.44,10.244.2.45
  NotReadyAddresses:  <none>
  Ports:
    Name     Port  Protocol
    ----     ----  --------
    <unset>  8080  TCP

Events:  <none>
```

### 缩容到 1 个

```
[root@master kubernetes]# kubectl scale --replicas=1 replicationcontroller rc-demo
replicationcontroller/rc-demo scaled
[root@master kubernetes]# kubectl describe endpoints svc-demo
Name:         svc-demo
Namespace:    default
Labels:       <none>
Annotations:  <none>
Subsets:
  Addresses:          10.244.2.44
  NotReadyAddresses:  <none>
  Ports:
    Name     Port  Protocol
    ----     ----  --------
    <unset>  8080  TCP

Events:  <none>
[root@master kubernetes]# kubectl get pod -o wide
NAME            READY   STATUS    RESTARTS   AGE     IP            NODE    NOMINATED NODE   READINESS GATES
rc-demo-5tshk   1/1     Running   0          3h41m   10.244.2.44   node2   <none>           <none>
```

从上面可以看出，其中 Endpoints 就对应一个 Pod 的地址。

## 总结

介绍了 Service 其中是通过 Endpoints 关联到 Pod 的地址上的，Pod 数量的增减，和 Endpoints 中的地址变化一致。

## 附录
