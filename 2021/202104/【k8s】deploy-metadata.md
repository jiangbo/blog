# 【k8s】deploy-metadata

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

查看 Deployment 中的一些字段。

## 示例

### Deployment.yaml

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: busybox
spec:
  selector:
    matchLabels:
      app: busybox
  replicas: 4
  template:
    metadata:
      labels:
        app: busybox
    spec:
      terminationGracePeriodSeconds: 5
      containers:
        - name: busybox
          image: busybox:1.30.0
          command: ["/bin/sh", "-c", "sleep 3600"]
```

### 查看

```
[root@master docker]# kubectl describe deployments.apps busybox
Name:                   busybox
Namespace:              default
CreationTimestamp:      Tue, 30 Mar 2021 21:41:46 +0800
Labels:                 <none>
Annotations:            deployment.kubernetes.io/revision: 1
Selector:               app=busybox
Replicas:               4 desired | 4 updated | 4 total | 0 available | 4 unavailable
StrategyType:           RollingUpdate
MinReadySeconds:        0
RollingUpdateStrategy:  25% max unavailable, 25% max surge
Pod Template:
  Labels:  app=busybox
  Containers:
   busybox:
    Image:      busybox:1.30.0
    Port:       <none>
    Host Port:  <none>
    Command:
      /bin/sh
      -c
      sleep 3600
    Environment:  <none>
    Mounts:       <none>
  Volumes:        <none>
Conditions:
  Type           Status  Reason
  ----           ------  ------
  Available      False   MinimumReplicasUnavailable
  Progressing    True    ReplicaSetUpdated
OldReplicaSets:  <none>
NewReplicaSet:   busybox-6bd65c67cf (4/4 replicas created)
Events:
  Type    Reason             Age   From                   Message
  ----    ------             ----  ----                   -------
  Normal  ScalingReplicaSet  3s    deployment-controller  Scaled up replica set busybox-6bd65c67cf to 4
```

Name，Selector 等字段都已经很熟悉了；Replicas 字段和之前的有点不一样，加入了状态。
升级相关的字段之后学习；Pod Template 字段里的都学习过；Conditions 后面学习。

## 总结

查看 Deployment 中的一些字段。

## 附录
