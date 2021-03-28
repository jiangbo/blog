# 【k8s】Pod-nodeName

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

nodeName 可以直接指定一个调度的节点。

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
  nodeName: master
```

### 查看

```
[root@master ~]# kubectl get pod -o wide
NAME         READY   STATUS    RESTARTS   AGE     IP            NODE     NOMINATED NODE   READINESS GATES
spring-k8s   1/1     Running   0          8m25s   10.244.0.27   master   <none>           <none>
```

## 总结

nodeName 可以直接指定一个调度的节点。

## 附录
