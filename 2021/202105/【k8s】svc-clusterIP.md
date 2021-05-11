# 【k8s】svc-clusterIP

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

Service 的 clusterIP 字段表示服务的集群 IP 地址，如果没有自定，会自动生成一个。
如果指定了，并且是合法的集群 IP 地址，它将会被 Service 使用。

## 示例

### Service.yaml

```yaml
apiVersion: v1
kind: Service
metadata:
  name: test
spec:
  ports:
    - port: 80
```

### 查看

```
[root@master ~]# kubectl get service
NAME         TYPE        CLUSTER-IP       EXTERNAL-IP   PORT(S)   AGE
kubernetes   ClusterIP   10.96.0.1        <none>        443/TCP   79d
test         ClusterIP   10.107.191.104   <none>        80/TCP    116s
[root@master ~]# kubectl describe service test
Name:              test
Namespace:         default
Labels:            <none>
Annotations:       <none>
Selector:          <none>
Type:              ClusterIP
IP Families:       <none>
IP:                10.107.191.104
IPs:               10.107.191.104
Port:              <unset>  80/TCP
TargetPort:        80/TCP
Endpoints:         <none>
Session Affinity:  None
Events:            <none>
[root@master ~]#
```

自动分配了 10.107.191.104 的地址。

### 手动指定

```yaml
apiVersion: v1
kind: Service
metadata:
  name: test4
spec:
  clusterIP: 10.107.191.144
  ports:
    - port: 80
```

```
[root@master ~]# kubectl get service
NAME         TYPE        CLUSTER-IP       EXTERNAL-IP   PORT(S)   AGE
kubernetes   ClusterIP   10.96.0.1        <none>        443/TCP   79d
test         ClusterIP   10.107.191.104   <none>        80/TCP    4m34s
test4        ClusterIP   10.107.191.144   <none>        80/TCP    31s
```

## 总结

clusterIP 表示 Service 的集群地址，可以自动生成，也可以手动指定。

## 附录
