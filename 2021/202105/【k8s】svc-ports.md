# 【k8s】svc-ports

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

Service 的 ports 可以定义服务和容器映射的端口，该字段是一个数组，可以定义多个端口映射。
和 ep 类似，如果定义了多个端口，需要将端口映射取一个名称。

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
  selector:
    app: spring-k8s
  ports:
    - name: port1
      port: 80
      targetPort: 8080
    - name: port2
      port: 8080
      targetPort: 8080
```

### 查看 

```
[root@master ~]# kubectl get svc
NAME         TYPE        CLUSTER-IP      EXTERNAL-IP   PORT(S)           AGE
kubernetes   ClusterIP   10.96.0.1       <none>        443/TCP           89d
spring-k8s   ClusterIP   10.104.121.41   <none>        80/TCP,8080/TCP   9d
```

可以看到暴露了两个端口，一个是 80，另一个是 8080。

### 查看详细

```
[root@master ~]# kubectl describe svc spring-k8s
Name:              spring-k8s
Namespace:         default
Labels:            <none>
Annotations:       <none>
Selector:          app=spring-k8s
Type:              ClusterIP
IP Families:       <none>
IP:                10.104.121.41
IPs:               10.104.121.41
Port:              port1  80/TCP
TargetPort:        8080/TCP
Endpoints:         10.244.2.230:8080
Port:              port2  8080/TCP
TargetPort:        8080/TCP
Endpoints:         10.244.2.230:8080
Session Affinity:  None
Events:            <none>
```

## 总结

ports 可以定义端口映射，其中端口的协议默认为 TCP，如果有需要，也可以修改成 UDP。

## 附录
