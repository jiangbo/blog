# 【k8s】deploy-RollingUpdate

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

deploy 可以自定义升级策略，其中升级的策略可以是 Recreate 和 RollingUpdate。
其中 RollingUpdate 是默认值，下面演示 RollingUpdate 升级策略。

## 示例

### Deployment.yaml

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: busybox
spec:
  strategy:
    type: RollingUpdate
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

然后将版本升级到 1.31.0。
`kubectl set image deployment/busybox busybox=busybox:1.31.0 --record`

### 查看

```
busybox-59c95c7d4b-hlqcr   0/1     Pending             0          0s
busybox-59c95c7d4b-hlqcr   0/1     Pending             0          0s
busybox-6bd65c67cf-95cs7   1/1     Terminating         0          22s
busybox-59c95c7d4b-hlqcr   0/1     ContainerCreating   0          0s
busybox-59c95c7d4b-l7jjb   0/1     Pending             0          0s
busybox-59c95c7d4b-l7jjb   0/1     Pending             0          0s
busybox-59c95c7d4b-l7jjb   0/1     ContainerCreating   0          1s
busybox-59c95c7d4b-l7jjb   1/1     Running             0          2s
busybox-6bd65c67cf-dnjsp   1/1     Terminating         0          24s
busybox-59c95c7d4b-j6h7n   0/1     Pending             0          0s
busybox-59c95c7d4b-j6h7n   0/1     Pending             0          0s
busybox-59c95c7d4b-j6h7n   0/1     ContainerCreating   0          0s
busybox-59c95c7d4b-hlqcr   1/1     Running             0          3s
busybox-6bd65c67cf-59jmp   1/1     Terminating         0          25s
busybox-59c95c7d4b-xjplh   0/1     Pending             0          0s
busybox-59c95c7d4b-xjplh   0/1     Pending             0          0s
busybox-59c95c7d4b-xjplh   0/1     ContainerCreating   0          0s
busybox-59c95c7d4b-j6h7n   1/1     Running             0          2s
busybox-6bd65c67cf-x96mv   1/1     Terminating         0          26s
busybox-59c95c7d4b-xjplh   1/1     Running             0          1s
busybox-6bd65c67cf-95cs7   0/1     Terminating         0          28s
busybox-6bd65c67cf-dnjsp   0/1     Terminating         0          30s
busybox-6bd65c67cf-59jmp   0/1     Terminating         0          31s
busybox-6bd65c67cf-x96mv   0/1     Terminating         0          32s
busybox-6bd65c67cf-95cs7   0/1     Terminating         0          33s
busybox-6bd65c67cf-95cs7   0/1     Terminating         0          33s
busybox-6bd65c67cf-dnjsp   0/1     Terminating         0          33s
busybox-6bd65c67cf-dnjsp   0/1     Terminating         0          33s
busybox-6bd65c67cf-59jmp   0/1     Terminating         0          43s
busybox-6bd65c67cf-59jmp   0/1     Terminating         0          43s
busybox-6bd65c67cf-x96mv   0/1     Terminating         0          44s
busybox-6bd65c67cf-x96mv   0/1     Terminating         0          44s
```

可以看到新旧 Pod 在交替进行，并不是直接将所有的停止后再启动新的。

## 总结

RollingUpdate 表示滚动更新，是默认的更新策略。

## 附录
