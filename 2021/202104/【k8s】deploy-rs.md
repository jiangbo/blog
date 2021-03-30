# 【k8s】deploy-rs

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

执行了更新操作后，可以看到 rs 变成了两个。一个是之前版本的 rs，一个是新版本的 rs。
其实所谓的 deploy 滚动升级，就是新增了一个 rs，让新的 rs 再次创建副本，并且缩小之前 rs 的副本数。

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

### 查看

```
[root@master ~]# kubectl get deploy,rs,pod
NAME                      READY   UP-TO-DATE   AVAILABLE   AGE
deployment.apps/busybox   8/8     8            8           5m41s

NAME                                 DESIRED   CURRENT   READY   AGE
replicaset.apps/busybox-59c95c7d4b   8         8         8       5m27s
replicaset.apps/busybox-6bd65c67cf   0         0         0       5m41s

NAME                           READY   STATUS    RESTARTS   AGE
pod/busybox-59c95c7d4b-5p5s7   1/1     Running   0          5m24s
pod/busybox-59c95c7d4b-5qzf2   1/1     Running   0          5m27s
pod/busybox-59c95c7d4b-8pgqk   1/1     Running   0          5m26s
pod/busybox-59c95c7d4b-c4v4l   1/1     Running   0          5m24s
pod/busybox-59c95c7d4b-czgnm   1/1     Running   0          5m26s
pod/busybox-59c95c7d4b-dnjgs   1/1     Running   0          5m27s
pod/busybox-59c95c7d4b-l89f5   1/1     Running   0          5m23s
pod/busybox-59c95c7d4b-z96jm   1/1     Running   0          5m24s
```

可以看到 deploy 还是之前的没有变化，但是 rs 出现了两个。一个是新版本的，并且现在管理着 Pod；
旧版本的 rs，没有管理 Pod，但是确实存在着。

## 总结

执行了 deploy 默认的滚动更新操作。在更新的过程中，并不是先把旧的完全结束，再创建新的。
而是先停一部分，启动一部分这样循环，直接完全替换，这就是所说的滚动更新。

## 附录
