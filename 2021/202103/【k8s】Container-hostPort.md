# 【k8s】Container-hostPort

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

在指定容器的端口时，可以指定 hostPort 字段，这样就可以将服务暴露到宿主机的端口上进行访问。
不过这样有一些缺点，首先是多个服务不能使用同一个端口，其次是只能访问调度宿主机的 IP。

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
          hostPort: 8081
```

### 容器 IP 访问

```
NAME         READY   STATUS    RESTARTS   AGE   IP             NODE    NOMINATED NODE   READINESS GATES
spring-k8s   1/1     Running   0          7s    10.244.1.197   node1   <none>           <none>
[root@master ~]# curl 10.244.1.197:8080/hostname;echo
spring-k8s
```

### 宿主机 IP 访问

```
[root@master ~]# curl 192.168.56.102:8081/hostname;echo
spring-k8s
```

### localhost 访问

在 192.168.56.102 主机上，通过 localhost 访问。

```
[root@node1 ~]# curl localhost:8081/hostname;echo
spring-k8s
```

## 总结

将容器中提供的服务暴露到宿主机上，可以很方便的使用，但是这样也有一些缺点。
首先是多个服务不能使用同一个端口，其次是只能访问调度宿主机的 IP。

## 附录
