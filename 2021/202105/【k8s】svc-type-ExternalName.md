# 【k8s】svc-type-ExternalName

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

k8s 支持的服务类型有四种，分别是：
1. ClusterIP
2. NodePort
3. ExternalName
4. LoadBalancer

这里介绍 ExternalName 类型，可以指定外部的域名地址。

## 示例

### Deploy.yaml

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: spring-k8s
spec:
  selector:
    matchLabels:
      app: spring-k8s
  template:
    metadata:
      labels:
        app: spring-k8s
    spec:
      containers:
        - name: spring-k8s
          image: jiangbo920827/spring-k8s:liveness
          ports:
            - containerPort: 8080
```

### Service.yaml

```yaml
apiVersion: v1
kind: Service
metadata:
  name: spring-k8s
spec:
  type: ExternalName
  externalName: cn.bing.com
```

### 查看

```
[root@master ~]# kubectl describe svc spring-k8s
Name:              spring-k8s
Namespace:         default
Labels:            <none>
Annotations:       <none>
Selector:          <none>
Type:              ExternalName
IP Families:       <none>
IP:
IPs:               <none>
External Name:     cn.bing.com
Session Affinity:  None
Events:            <none>
```

### 验证

```
kubectl exec -it spring-k8s-79f74b55d7-d527c  -- sh
/ # ping cn.bing.com
PING cn.bing.com (202.89.233.101): 56 data bytes
64 bytes from 202.89.233.101: seq=0 ttl=117 time=37.679 ms
64 bytes from 202.89.233.101: seq=1 ttl=117 time=37.467 ms
64 bytes from 202.89.233.101: seq=2 ttl=117 time=37.158 ms
64 bytes from 202.89.233.101: seq=3 ttl=117 time=37.342 ms
64 bytes from 202.89.233.101: seq=4 ttl=117 time=37.259 ms
^C
--- cn.bing.com ping statistics ---
5 packets transmitted, 5 packets received, 0% packet loss
round-trip min/avg/max = 37.158/37.381/37.679 ms
/ # ping spring-k8s
PING spring-k8s (202.89.233.101): 56 data bytes
64 bytes from 202.89.233.101: seq=0 ttl=117 time=37.315 ms
64 bytes from 202.89.233.101: seq=1 ttl=117 time=37.030 ms
64 bytes from 202.89.233.101: seq=2 ttl=117 time=37.258 ms
```

## 总结

ExternalName 可以将外部的域名服务引入到集群内部。

## 附录
