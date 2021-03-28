# 【k8s】Pod-hostname

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

hostname 可以指定主机的名称。

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
  hostname: spring-boot-kubernetes
```

### 查看主机名

```
[root@master ~]# kubectl get pod -o wide
NAME         READY   STATUS    RESTARTS   AGE   IP             NODE    NOMINATED NODE   READINESS GATES
spring-k8s   1/1     Running   0          26s   10.244.2.171   node2   <none>           <none>
[root@master ~]# curl 10.244.2.171:8080/hostname;echo
spring-boot-kubernetes
[root@master ~]#

```

## 总结

hostname 可以指定主机的名称。

## 附录
