# 【k8s】deploy-rollout

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

在之前，提到过 deploy 相比较 rs，多了升级的一些功能。
下面演示 deploy 的滚动更新。

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
  replicas: 8
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

### 修改镜像版本

`kubectl set image deployment/busybox busybox=busybox:1.31.0 --record`

### 查看滚动操作

```
[root@master ~]# kubectl rollout status deployment busybox
Waiting for deployment "busybox" rollout to finish: 4 out of 8 new replicas have been updated...
Waiting for deployment "busybox" rollout to finish: 4 out of 8 new replicas have been updated...
Waiting for deployment "busybox" rollout to finish: 4 out of 8 new replicas have been updated...
Waiting for deployment "busybox" rollout to finish: 4 out of 8 new replicas have been updated...
Waiting for deployment "busybox" rollout to finish: 5 out of 8 new replicas have been updated...
Waiting for deployment "busybox" rollout to finish: 5 out of 8 new replicas have been updated...
Waiting for deployment "busybox" rollout to finish: 6 out of 8 new replicas have been updated...
Waiting for deployment "busybox" rollout to finish: 6 out of 8 new replicas have been updated...
Waiting for deployment "busybox" rollout to finish: 6 out of 8 new replicas have been updated...
Waiting for deployment "busybox" rollout to finish: 6 out of 8 new replicas have been updated...
Waiting for deployment "busybox" rollout to finish: 6 out of 8 new replicas have been updated...
Waiting for deployment "busybox" rollout to finish: 7 out of 8 new replicas have been updated...
Waiting for deployment "busybox" rollout to finish: 7 out of 8 new replicas have been updated...
Waiting for deployment "busybox" rollout to finish: 7 out of 8 new replicas have been updated...
Waiting for deployment "busybox" rollout to finish: 7 out of 8 new replicas have been updated...
Waiting for deployment "busybox" rollout to finish: 2 old replicas are pending termination...
Waiting for deployment "busybox" rollout to finish: 1 old replicas are pending termination...
Waiting for deployment "busybox" rollout to finish: 1 old replicas are pending termination...
Waiting for deployment "busybox" rollout to finish: 1 old replicas are pending termination...
Waiting for deployment "busybox" rollout to finish: 6 of 8 updated replicas are available...
Waiting for deployment "busybox" rollout to finish: 7 of 8 updated replicas are available...
deployment "busybox" successfully rolled out
```

可以看到旧的 Pod 慢慢被新的替换，直接全部变成新的版本。

## 总结

执行了 deploy 默认的滚动更新操作。在更新的过程中，并不是先把旧的完全结束，再创建新的。
而是先停一部分，启动一部分这样循环，直接完全替换，这就是所说的滚动更新。

## 附录
