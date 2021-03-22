# 【Kubernetes】集群访问本地服务

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

在前面，我们将本地服务引入到了集群中，并且通过 Service 的 NodePort 暴露给宿主机访问。
下面我们直接在集群中访问引入到了集群里的数据库，同样也暴露 NodePort 端口访问。

## 配置访问 IP

### 查看 PG 数据库 IP

```
[root@master kubernetes]# kubectl describe -f svc.yaml
Name:                     svc-pg
Namespace:                default
Labels:                   <none>
Annotations:              <none>
Selector:                 <none>
Type:                     NodePort
IP Families:              <none>
IP:                       10.109.50.111
IPs:                      10.109.50.111
Port:                     <unset>  5432/TCP
TargetPort:               5432/TCP
NodePort:                 <unset>  32206/TCP
Endpoints:                192.168.56.103:5432
Session Affinity:         None
External Traffic Policy:  Cluster
Events:                   <none>
```

### 修改服务配置

已经将配置修改到 Service 的 IP 上了。

```yaml
management:
  endpoint:
    shutdown:
      enabled: true
  endpoints:
    web:
      exposure:
        include: "*"
        
spring:
  datasource:
    url: jdbc:postgresql://10.109.50.111:5432/postgres
    username: postgres
    password: 123456
```

## 创建 Pod

### 打包上传 Docker Hub

```
mvn clean package
...

docker build -t jiangbo920827/spring-demo:external .
...

docker push jiangbo920827/spring-demo:external
...
```

### 修改 rc.yaml

```yaml
apiVersion: v1
kind: ReplicationController
metadata:
  name: rc-demo
spec:
  replicas: 3
  template:
    metadata:
      labels:
        app: myapp
    spec:
      containers:
        - name: pod-demo
          image: jiangbo920827/spring-demo:external
          ports:
            - containerPort: 8080

```

### 查看 Pod 状态

```
[root@master kubernetes]# kubectl get pod -o wide
NAME            READY   STATUS    RESTARTS   AGE   IP            NODE    NOMINATED NODE   READINESS GATES
rc-demo-7mg6q   1/1     Running   0          62s   10.244.1.93   node1   <none>           <none>
rc-demo-bzxkw   1/1     Running   0          62s   10.244.2.46   node2   <none>           <none>
rc-demo-hk587   1/1     Running   0          62s   10.244.1.92   node1   <none>           <none>
```

## 访问测试

### 查看 Service 状态

```
[root@master kubernetes]# kubectl get service svc-demo
NAME       TYPE       CLUSTER-IP       EXTERNAL-IP   PORT(S)          AGE
svc-demo   NodePort   10.106.217.209   <none>        8080:32329/TCP   7h49m
```

### 访问测试

```
PS D:\workspace\sts\spring-demo> curl 192.168.56.101:32329/users
[{"name":"jiangbo","age":44}]
```

## 总结

介绍了在集群中访问外部服务的方式，通过外部服务暴露的 Service 访问外部服务，不管外部服务的 IP 变化成多少，
我们自己的服务配置都不用修改。因为中间还存在一层 Service，我们访问的是 Service 的 IP。
通过之前的练习，发现通过固定的 Service IP 访问虽然很稳定，但是存在难理解，不可读的问题。

## 附录
