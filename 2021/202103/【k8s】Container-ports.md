# 【k8s】Container-ports

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

容器的 ports 字段是一个数组，可以指定多个端口。需要注意的是，未指定的端口也不会被阻拦，
指定端口只是提供了一种信息，方便查看。

## 示例

### Pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
spec:
  containers:
    - name: spring-k8s
      image: jiangbo920827/spring-k8s:liveness
      ports:
        - containerPort: 8080
```

### 访问

```
spring-k8s[root@master ~]# curl 10.244.1.194:8080/hostname;echo
spring-k8s
```

### 不指定端口

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
spec:
  containers:
    - name: spring-k8s
      image: jiangbo920827/spring-k8s:liveness
```

### 访问测试

```
[root@master ~]# kubectl get pod -o  wide
NAME         READY   STATUS    RESTARTS   AGE   IP             NODE    NOMINATED NODE   READINESS GATES
spring-k8s   1/1     Running   0          12s   10.244.1.195   node1   <none>           <none>
[root@master ~]# curl 10.244.1.195:8080/hostname;echo
spring-k8s
```

## 总结

在 Pod 中定义容器的端口，端口只做显示使用，便于查看。

## 附录
