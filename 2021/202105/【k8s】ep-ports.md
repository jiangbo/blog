# 【k8s】ep-ports

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标


Endpoints 的 ports 可以指定端口信息，如果只定义了一个端口，可以省略名称。
如果有多个端口，名称不可以省略，默认的协议是 TCP。

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
        name: http
```

和相同名称的 Service 进行了绑定。

### 访问

```
spring-k8s-784db5c64f-klhnj[root@master ~]# curl node2/hostname;echo
spring-k8s-784db5c64f-klhnj
[root@master ~]#
```

## 总结

Endpoints 的 ports 可以指定端口信息。

## 附录
