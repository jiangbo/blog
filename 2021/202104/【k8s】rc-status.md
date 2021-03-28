# 【k8s】rc-status

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

status 属于状态字段，显示额外的信息。

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
  status:
    availableReplicas: 1
    fullyLabeledReplicas: 1
    observedGeneration: 2
    readyReplicas: 1
    replicas: 1
```

availableReplicas 表示准备完成的副本数；
fullyLabeledReplicas 表示满足标签选择器的副本数；
readyReplicas 表示准备完成的副本数；
replicas 表示当前的副本数；
observedGeneration 表示 rc 的代数，修改一下，增长 1。

## 总结

介绍了 rc 的状态字段中的一些信息。

## 附录
