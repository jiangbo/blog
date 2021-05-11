# 【k8s】svc-clusterIPs

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

Service 的 clusterIPs 可以指定 IP 地址列表。

## 示例

### Service.yaml

```yaml
apiVersion: v1
kind: Service
metadata:
  name: test4
spec:
  clusterIP: 10.107.191.144
  clusterIPs: ["10.107.191.144", "10.107.191.155", "10.107.191.166"]
  ports:
    - port: 80
```

### 查看

```
[root@master ~]# kubectl describe service test4
Name:              test4
Namespace:         default
Labels:            <none>
Annotations:       <none>
Selector:          <none>
Type:              ClusterIP
IP Families:       <none>
IP:                10.107.191.144
IPs:               10.107.191.144
Port:              <unset>  80/TCP
TargetPort:        80/TCP
Endpoints:         <none>
Session Affinity:  None
Events:            <none>
```

可以看到指定的 IP 地址列表还是只有一个，不清楚为什么。

## 总结

clusterIPs 可以设置集群地址的 IP 列表，不过好像就第一个生效。

## 附录
