# 【k8s】svc-sessionAffinity

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

sessionAffinity 表示 session 亲和，目前可以有两种取值，一种是 None，也是默认值，表示没有，
会直接轮询 Pod。一种是 ClientIP，表示根据客户端 IP 亲和，同一个客户端 IP，会被发送到同一个 Pod 上。

## 示例

### Deploy.yaml

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: spring-k8s
spec:
  replicas: 4
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
    - port: 80
      targetPort: 8080
```

### 访问

```
[root@master ~]# curl 10.104.121.41/hostname;echo
spring-k8s-79f74b55d7-qrr5j
[root@master ~]# curl 10.104.121.41/hostname;echo
spring-k8s-79f74b55d7-bsknz
[root@master ~]# curl 10.104.121.41/hostname;echo
spring-k8s-79f74b55d7-6hfwq
[root@master ~]# curl 10.104.121.41/hostname;echo
spring-k8s-79f74b55d7-d527c
[root@master ~]# curl 10.104.121.41/hostname;echo
spring-k8s-79f74b55d7-qrr5j
[root@master ~]# curl 10.104.121.41/hostname;echo
spring-k8s-79f74b55d7-bsknz
[root@master ~]# curl 10.104.121.41/hostname;echo
```

### 增加亲和性

```yaml
apiVersion: v1
kind: Service
metadata:
  name: spring-k8s
spec:
  sessionAffinity: ClientIP
  selector:
    app: spring-k8s
  ports:
    - port: 80
      targetPort: 8080
```

### 验证

```
[root@master ~]# curl 10.104.121.41/hostname;echo
spring-k8s-79f74b55d7-qrr5j
[root@master ~]# curl 10.104.121.41/hostname;echo
spring-k8s-79f74b55d7-qrr5j
[root@master ~]# curl 10.104.121.41/hostname;echo
spring-k8s-79f74b55d7-qrr5j
[root@master ~]# curl 10.104.121.41/hostname;echo
spring-k8s-79f74b55d7-qrr5j
```

## 总结

sessionAffinity 可以设置会话亲和性，可以选择轮询或者会话固定。

## 附录
