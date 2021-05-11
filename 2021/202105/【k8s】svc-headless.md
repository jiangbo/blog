# 【k8s】svc-headless

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

在 k8s 的 Service 的使用过程中，有一种比较特殊的 Service，叫做 headless。
它与其它 Service 的最大区别就是不提供负载均衡 IP，而是直接走 DNS 记录。
并且它和 sts 结合，还可以访问到固定的某个 Pod。

## 示例

### StatefulSet.yaml

```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: spring-k8s
spec:
  serviceName: spring-k8s
  selector:
    matchLabels:
      app: spring-k8s
  replicas: 2
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
  clusterIP: None
  selector:
    app: spring-k8s
  ports:
    - port: 80
      targetPort: 8080
```

### 查看全部地址

```
[root@master ~]# nslookup spring-k8s.default.svc.cluster.local 10.96.0.10
Server:         10.96.0.10
Address:        10.96.0.10#53

Name:   spring-k8s.default.svc.cluster.local
Address: 10.244.2.6
Name:   spring-k8s.default.svc.cluster.local
Address: 10.244.2.5

```

可以看到，根据服务名访问时，会得到两个 IP 地址。

### 指定访问 Pod

```
[root@master ~]# nslookup spring-k8s-0.spring-k8s.default.svc.cluster.local 10.96.0.10
Server:         10.96.0.10
Address:        10.96.0.10#53

Name:   spring-k8s-0.spring-k8s.default.svc.cluster.local
Address: 10.244.2.5

[root@master ~]# nslookup spring-k8s-1.spring-k8s.default.svc.cluster.local 10.96.0.10
Server:         10.96.0.10
Address:        10.96.0.10#53

Name:   spring-k8s-1.spring-k8s.default.svc.cluster.local
Address: 10.244.2.6

```
加上 Pod 的名称和服务名，就可以直接访问到具体的某个 Pod。

## 总结

headless 不提供负载均衡的 IP 地址，和 sts 结合可以访问指定的 Pod。

## 附录
