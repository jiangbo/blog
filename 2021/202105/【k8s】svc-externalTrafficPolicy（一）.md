# 【k8s】svc-externalTrafficPolicy（一）

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-M3

## 目标

Service 的 externalTrafficPolicy 表示外部流量策略，主要的作用是设置是否保留源 IP。
如果是 ClusterIP 类型的 Service 默认会保留源 IP。

## 示例

### 创建 Deploy

`kubectl create deployment source-ip-app --image=cilium/echoserver:1.10.2`

```
[root@master ~]# kubectl create deployment source-ip-app --image=cilium/echoserver:1.10.2
deployment.apps/source-ip-app created
[root@master ~]# kubectl get all
NAME                                 READY   STATUS    RESTARTS   AGE
pod/source-ip-app-5c78f5f6bc-fbdj8   1/1     Running   0          7s

NAME                 TYPE        CLUSTER-IP   EXTERNAL-IP   PORT(S)   AGE
service/kubernetes   ClusterIP   10.96.0.1    <none>        443/TCP   96d

NAME                            READY   UP-TO-DATE   AVAILABLE   AGE
deployment.apps/source-ip-app   1/1     1            1           8s

NAME                                       DESIRED   CURRENT   READY   AGE
replicaset.apps/source-ip-app-5c78f5f6bc   1         1         1       7s
```

### 创建 Service

`kubectl expose deployment source-ip-app --name=clusterip --port=80 --target-port=8080`

```
[root@master ~]# kubectl expose deployment source-ip-app --name=clusterip --port=80 --target-port=8080
service/clusterip exposed
[root@master ~]# kubectl get svc clusterip
NAME        TYPE        CLUSTER-IP     EXTERNAL-IP   PORT(S)   AGE
clusterip   ClusterIP   10.98.36.116   <none>        80/TCP    13s
```

### 查看

```
[root@master ~]# kubectl run busybox -it --image=busybox:stable --restart=Never --rm
If you don't see a command prompt, try pressing enter.
/ # ip a
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
    inet 127.0.0.1/8 scope host lo
       valid_lft forever preferred_lft forever
3: eth0@if11: <BROADCAST,MULTICAST,UP,LOWER_UP,M-DOWN> mtu 1450 qdisc noqueue
    link/ether ee:de:ce:a8:72:9d brd ff:ff:ff:ff:ff:ff
    inet 10.244.2.250/24 brd 10.244.2.255 scope global eth0
       valid_lft forever preferred_lft forever
/ # wget -qO - 10.98.36.116


Hostname: source-ip-app-5c78f5f6bc-fbdj8

Pod Information:
        -no pod information available-

Server values:
        server_version=nginx: 1.13.3 - lua: 10008

Request Information:
        client_address=::ffff:10.244.2.250
        method=GET
        real path=/
        query=
        request_version=1.1
        request_scheme=http
        request_uri=http://10.98.36.116:8080/

Request Headers:
        connection=close
        host=10.98.36.116
        user-agent=Wget

Request Body:
        -no body in request-
```

可以看到客户端的 IP 地址是 10.244.2.250，正好就是启动的 Pod 的 IP 地址。

## 总结

如果服务类型是 ClusterIP，默认情况，访问服务会保留源 IP。

## 附录
