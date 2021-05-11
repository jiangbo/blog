# 【k8s】svc-externalIPs

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

Service 的 externalIPs 可以设置一个外部的 IP 地址，并且将流量导入到集群内部。

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
  externalIPs:
    - "111.111.111.111"
  selector:
    app: spring-k8s
  ports:
    - port: 80
      targetPort: 8080
```

### 查看

```
[root@master ~]# kubectl get pod,svc
NAME                              READY   STATUS    RESTARTS   AGE
pod/spring-k8s-79f74b55d7-bzbt7   1/1     Running   0          56s

NAME                 TYPE        CLUSTER-IP      EXTERNAL-IP       PORT(S)   AGE
service/kubernetes   ClusterIP   10.96.0.1       <none>            443/TCP   79d
service/spring-k8s   ClusterIP   10.104.121.41   111.111.111.111   80/TCP    13m
[root@master ~]# curl 111.111.111.111/hostname;echo
spring-k8s-79f74b55d7-bzbt7
[root@master ~]#
```

可以看到访问 111.111.111.111 地址，访问的是集群内部的接口。

## 总结

externalIPs 可以将集群外部的流量转发到集群内部来。

## 附录
