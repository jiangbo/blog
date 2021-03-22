# 【Kubernetes】通过名称访问 Service

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

之前，我们看到通过 Service 名称组合而成的环境变量访问的问题。通过名称访问应该是较好的方式，
但是怎么解决动态更新的问题？那就是使用 DNS。

## 测试名称访问

### 查看当前 Pod

```
[root@master ~]# kubectl get pod -o wide
NAME            READY   STATUS    RESTARTS   AGE   IP             NODE    NOMINATED NODE   READINESS GATES
rc-demo-46dq7   1/1     Running   0          61m   10.244.1.113   node1   <none>           <none>
rc-demo-dxvlv   1/1     Running   0          61m   10.244.1.111   node1   <none>           <none>
rc-demo-j48b8   1/1     Running   0          61m   10.244.1.112   node1   <none>           <none>
```

### 测试连通性

```
[root@master kubernetes]# kubectl exec -it rc-demo-j48b8 -- sh
/ # ping svc-pg
PING svc-pg (10.108.222.50): 56 data bytes
64 bytes from 10.108.222.50: seq=0 ttl=64 time=0.202 ms
64 bytes from 10.108.222.50: seq=1 ttl=64 time=0.134 ms
64 bytes from 10.108.222.50: seq=2 ttl=64 time=0.590 ms
64 bytes from 10.108.222.50: seq=3 ttl=64 time=0.146 ms
^C
--- svc-pg ping statistics ---
4 packets transmitted, 4 packets received, 0% packet loss
round-trip min/avg/max = 0.134/0.268/0.590 ms
/ #
```

## 查看 DNS 解析

### dnsutils

```
[root@master kubernetes]# kubectl run dnsutils --image=tutum/dnsutils --command -- sleep infinity
pod/dnsutils created
```

### nslookup

```
[root@master kubernetes]# kubectl exec -it dnsutils -- bash
root@dnsutils:/# nslookup svc-pg
Server:         10.96.0.10
Address:        10.96.0.10#53

Name:   svc-pg.default.svc.cluster.local
Address: 10.108.222.50

root@dnsutils:/# exit
exit
[root@master kubernetes]# kubectl get service svc-pg
NAME     TYPE        CLUSTER-IP      EXTERNAL-IP   PORT(S)    AGE
svc-pg   ClusterIP   10.108.222.50   <none>        5432/TCP   138m
```

通过对比，发现通过服务名得到的 IP 地址和 Service 一样的。

## 修改 Service

### 删除 svc-pg

```
[root@master kubernetes]# kubectl delete service svc-pg
service "svc-pg" deleted
```

### 新建 svc-pg

```yaml
apiVersion: v1
kind: Service
metadata:
  name: svc-pg
spec:
  ports:
    - port: 5432
```

### 查看 Service 信息

```
[root@master kubernetes]# kubectl describe  service svc-pg
Name:              svc-pg
Namespace:         default
Labels:            <none>
Annotations:       <none>
Selector:          <none>
Type:              ClusterIP
IP Families:       <none>
IP:                10.108.243.92
IPs:               10.108.243.92
Port:              <unset>  5432/TCP
TargetPort:        5432/TCP
Endpoints:         <none>
Session Affinity:  None
Events:            <none>
```

## 验证

可以看到 DNS 的记录信息也同步更新了。

```
[root@master kubernetes]# kubectl exec -it dnsutils -- bash
root@dnsutils:/# nslookup svc-pg
Server:         10.96.0.10
Address:        10.96.0.10#53

Name:   svc-pg.default.svc.cluster.local
Address: 10.108.243.92

```

## 总结

介绍了通过服务名访问的方式，这是 k8s 推荐的方式，也是最方便的一种方式。
不过有个小问题，端口号还需要自己指定，所以建议端口一般使用默认，就不需要指定了。

## 附录
