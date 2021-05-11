# 【k8s】ep-addresses

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

Endpoints 的 addresses 可以指定需要访问的 IP 地址，单独的 ep 没有多少作用，需要搭配 Service 使用。
Endpoints 会和同名的 Service 绑定。

## 示例

### 创建 Service

```yaml
apiVersion: v1
kind: Service
metadata:
  name: spring-k8s
spec:
  ports:
    - port: 80
```

### 创建 Pod

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
      nodeName: node2
      containers:
        - name: spring-k8s
          image: jiangbo920827/spring-k8s:liveness
          ports:
            - containerPort: 8080
              hostPort: 80
```

创建了单 Pod 的 deploy，并且将节点选择在了 node2 上，并且在宿主机上暴露了 80 端口。

### 查看 Pod

```
[root@master ~]# kubectl get pod -o wide
NAME                          READY   STATUS    RESTARTS   AGE   IP             NODE    NOMINATED NODE   READINESS GATES
spring-k8s-784db5c64f-klhnj   1/1     Running   0          59m   10.244.2.218   node2   <none>           <none>
```

### 创建 ep

```yaml
apiVersion: v1
kind: Endpoints
metadata:
  name: spring-k8s
subsets:
  - addresses:
      - ip: 10.244.2.218
    ports:
      - port: 8080
```

和相同名称的 Service 进行了绑定。

### 访问

```
[root@master ~]# curl 10.244.2.218:8080/hostname;echo
spring-k8s-784db5c64f-klhnj
[root@master ~]# kubectl get svc
NAME         TYPE        CLUSTER-IP    EXTERNAL-IP   PORT(S)   AGE
kubernetes   ClusterIP   10.96.0.1     <none>        443/TCP   78d
spring-k8s   ClusterIP   10.107.6.66   <none>        80/TCP    73m
[root@master ~]# curl 10.107.6.66/hostname;echo
spring-k8s-784db5c64f-klhnj
```

## 总结

Endpoints 的 addresses 可以指定可访问的地址集合。

## 附录
