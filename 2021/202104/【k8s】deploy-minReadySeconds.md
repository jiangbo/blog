# 【k8s】deploy-minReadySeconds

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

deploy 在更新过程中，启动 Pod 后，minReadySeconds 可用定义该 Pod 经过多少秒后才被视为可用。
如果新的 Pod 不可用，是不会替换旧的，直接新的可用为止。

## 示例

### Deployment.yaml

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: busybox
spec:
  minReadySeconds: 30
  strategy:
    rollingUpdate:
      maxUnavailable: 0
      maxSurge: 1
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
          image: busybox:1.31.0
          command: ["/bin/sh", "-c", "sleep 3600"]
```

然后将版本升级到 1.31.0。
`kubectl set image deployment/busybox busybox=busybox:1.31.0 --record`

### 查看

```
busybox-59c95c7d4b-8lk22   0/1     Pending             0          0s
busybox-59c95c7d4b-8lk22   0/1     Pending             0          0s
busybox-59c95c7d4b-8lk22   0/1     ContainerCreating   0          0s
busybox-59c95c7d4b-8lk22   1/1     Running             0          2s
busybox-6bd65c67cf-mjn64   1/1     Terminating         0          46s
busybox-59c95c7d4b-shzxn   0/1     Pending             0          0s
busybox-59c95c7d4b-shzxn   0/1     Pending             0          0s
busybox-59c95c7d4b-shzxn   0/1     ContainerCreating   0          0s
busybox-59c95c7d4b-shzxn   1/1     Running             0          1s
busybox-6bd65c67cf-mjn64   0/1     Terminating         0          52s
busybox-6bd65c67cf-mjn64   0/1     Terminating         0          53s
busybox-6bd65c67cf-mjn64   0/1     Terminating         0          53s
busybox-6bd65c67cf-nv6hv   1/1     Terminating         0          77s
busybox-59c95c7d4b-cx6zx   0/1     Pending             0          0s
busybox-59c95c7d4b-cx6zx   0/1     Pending             0          0s
busybox-59c95c7d4b-cx6zx   0/1     ContainerCreating   0          0s
busybox-59c95c7d4b-cx6zx   1/1     Running             0          2s
busybox-6bd65c67cf-nv6hv   0/1     Terminating         0          83s
busybox-6bd65c67cf-nv6hv   0/1     Terminating         0          90s
busybox-6bd65c67cf-nv6hv   0/1     Terminating         0          90s
busybox-6bd65c67cf-5mrnx   1/1     Terminating         0          109s
busybox-59c95c7d4b-cjvg7   0/1     Pending             0          0s
busybox-59c95c7d4b-cjvg7   0/1     Pending             0          0s
busybox-59c95c7d4b-cjvg7   0/1     ContainerCreating   0          0s
busybox-59c95c7d4b-cjvg7   1/1     Running             0          2s
busybox-6bd65c67cf-5mrnx   0/1     Terminating         0          115s
busybox-6bd65c67cf-5mrnx   0/1     Terminating         0          116s
busybox-6bd65c67cf-5mrnx   0/1     Terminating         0          116s
busybox-6bd65c67cf-fz4l2   1/1     Terminating         0          2m21s
busybox-6bd65c67cf-fz4l2   0/1     Terminating         0          2m27s
busybox-6bd65c67cf-fz4l2   0/1     Terminating         0          2m31s
busybox-6bd65c67cf-fz4l2   0/1     Terminating         0          2m31s
```

因为将 maxUnavailable 为 0，maxSurge 为 1，所以先新增了一个 Pod。
不过在新增成功后，并没有马上停止旧的，而是等了一段时间才停止旧的。
等待的时间必须大于 minReadySeconds 定义的时间。

## 总结

minReadySeconds 可用定义新建的 Pod 经过多少秒后才被视为可用。

## 附录
