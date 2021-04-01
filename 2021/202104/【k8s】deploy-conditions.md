# 【k8s】deploy-conditions

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

deploy 的 conditions 条件默认只有两个。

## 示例

### Deployment.yaml

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

### 查看

```
status:
  availableReplicas: 1
  conditions:
  - lastTransitionTime: "2021-04-01T15:42:33Z"
    lastUpdateTime: "2021-04-01T15:42:33Z"
    message: Deployment has minimum availability.
    reason: MinimumReplicasAvailable
    status: "True"
    type: Available
  - lastTransitionTime: "2021-04-01T15:42:30Z"
    lastUpdateTime: "2021-04-01T15:42:33Z"
    message: ReplicaSet "spring-k8s-79f74b55d7" has successfully progressed.
    reason: NewReplicaSetAvailable
    status: "True"
    type: Progressing
```

一个条件是达到最小的可用副本数就变为 True，另一个是新的 rs 可用了就变为 True。

## 总结

介绍了 deploy 的 conditions 字段。

## 附录
