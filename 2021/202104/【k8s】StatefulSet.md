# 【k8s】StatefulSet

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

StatefulSet 是专门为有状态服务设计的，使用 StatefulSet 创建的 Pod，有稳定的网络标识。
现在可以先了解有这样一种资源，后面详细学习。

## 示例

### StatefulSet.yaml

```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: spring-k8s
spec:
  serviceName: spring
  selector:
    matchLabels:
      app: spring-k8s
  replicas: 2
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
[root@master ~]# kubectl describe sts spring-k8s
Name:               spring-k8s
Namespace:          default
CreationTimestamp:  Tue, 06 Apr 2021 20:30:19 +0800
Selector:           app=spring-k8s
Labels:             <none>
Annotations:        <none>
Replicas:           2 desired | 2 total
Update Strategy:    RollingUpdate
  Partition:        0
Pods Status:        2 Running / 0 Waiting / 0 Succeeded / 0 Failed
Pod Template:
  Labels:  app=spring-k8s
  Containers:
   spring-k8s:
    Image:        jiangbo920827/spring-k8s:liveness
    Port:         8080/TCP
    Host Port:    0/TCP
    Environment:  <none>
    Mounts:       <none>
  Volumes:        <none>
Volume Claims:    <none>
Events:
  Type    Reason            Age   From                    Message
  ----    ------            ----  ----                    -------
  Normal  SuccessfulCreate  10m   statefulset-controller  create Pod spring-k8s-0 in StatefulSet spring-k8s successful
  Normal  SuccessfulCreate  10m   statefulset-controller  create Pod spring-k8s-1 in StatefulSet spring-k8s successful
[root@master ~]#
```

## 总结

StatefulSet 可以创建有状态的服务，它们的名称从 0 开始，依次递增。

## 附录
