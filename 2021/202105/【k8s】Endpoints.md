# 【k8s】Endpoints

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

Endpoints 表示实际可访问服务的地址集合，Endpoints 的简写为 ep。

## 示例

### 查看已有 Endpoints

```
[root@master ~]# kubectl get ep
NAME         ENDPOINTS             AGE
kubernetes   192.168.56.101:6443   78d
```

刚好就是控制平面 api-server 的地址和端口。

### 查看详情

```
[root@master ~]# kubectl describe ep kubernetes
Name:         kubernetes
Namespace:    default
Labels:       endpointslice.kubernetes.io/skip-mirror=true
Annotations:  <none>
Subsets:
  Addresses:          192.168.56.101
  NotReadyAddresses:  <none>
  Ports:
    Name   Port  Protocol
    ----   ----  --------
    https  6443  TCP

Events:  <none>
```

## 总结

Endpoints 表示一组实际可访问的地址集合。

## 附录
