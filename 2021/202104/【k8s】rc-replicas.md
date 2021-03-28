# 【k8s】rc-replicas

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

replicas 字段可以控制启动的副本数，如果不写，默认为 1。

## 示例

### rc.yaml

```yaml
apiVersion: v1
kind: ReplicationController
metadata:
  name: spring-k8s
spec:
  replicas: 10
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

### 查看

```
[root@master manifests]# kubectl get pod -o wide
NAME               READY   STATUS    RESTARTS   AGE   IP             NODE    NOMINATED NODE   READINESS GATES
spring-k8s-22xpv   1/1     Running   0          41s   10.244.1.226   node1   <none>           <none>
spring-k8s-26c6j   1/1     Running   0          42s   10.244.2.203   node2   <none>           <none>
spring-k8s-8l5wj   1/1     Running   0          41s   10.244.2.205   node2   <none>           <none>
spring-k8s-brxmf   1/1     Running   0          42s   10.244.1.224   node1   <none>           <none>
spring-k8s-h5p5z   1/1     Running   0          41s   10.244.1.227   node1   <none>           <none>
spring-k8s-nmbmg   1/1     Running   0          41s   10.244.1.228   node1   <none>           <none>
spring-k8s-phwj7   1/1     Running   0          41s   10.244.2.206   node2   <none>           <none>
spring-k8s-r68h5   1/1     Running   0          41s   10.244.1.225   node1   <none>           <none>
spring-k8s-vfcv5   1/1     Running   0          41s   10.244.2.207   node2   <none>           <none>
spring-k8s-wjx2m   1/1     Running   0          41s   10.244.2.204   node2   <none>           <none>
```

## 总结

replicas 可以设置启动的副本数。

## 附录
