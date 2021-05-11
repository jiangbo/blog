# 【k8s】Service

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

Service 表示可以访问的服务，是为了给 pod 提供的服务一个稳定的访问入口。
可以简写为 svc。

## 示例

### 查看已有 Service

```
[root@master ~]# kubectl get service
NAME         TYPE        CLUSTER-IP   EXTERNAL-IP   PORT(S)   AGE
kubernetes   ClusterIP   10.96.0.1    <none>        443/TCP   79d
[root@master ~]#
```

### 查看详情

```
[root@master ~]# kubectl describe service kubernetes
Name:              kubernetes
Namespace:         default
Labels:            component=apiserver
                   provider=kubernetes
Annotations:       <none>
Selector:          <none>
Type:              ClusterIP
IP Families:       <none>
IP:                10.96.0.1
IPs:               10.96.0.1
Port:              https  443/TCP
TargetPort:        6443/TCP
Endpoints:         192.168.56.101:6443
Session Affinity:  None
Events:            <none>
[root@master ~]#
```

## 总结

Service 表示可以访问的服务地址。

## 附录
