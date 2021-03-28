# 【k8s】rs-matchExpressions

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

rs 在标签选择器上，除了可以定义键值对的选择形式，还支持 matchExpressions 字段，可以提供多种选择。
目前支持的操作包括：

1. In
2. NotIn
3. Exists
4. DoesNotExist.

## 示例

### rc.yaml

```yaml
apiVersion: apps/v1
kind: ReplicaSet
metadata:
  name: spring-k8s
spec:
  selector:
    matchExpressions:
      - key: app
        operator: Exists
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
[root@master manifests]# kubectl get pod,rs --show-labels
NAME                   READY   STATUS    RESTARTS   AGE   LABELS
pod/spring-k8s-vpncb   1/1     Running   0          63s   app=spring-k8s

NAME                         DESIRED   CURRENT   READY   AGE   LABELS
replicaset.apps/spring-k8s   1         1         1       63s   <none>
```

## 总结

介绍了 ReplicaSet 的标签选择器 matchExpressions 的使用方法。

## 附录
