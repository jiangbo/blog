# 【k8s】deploy-Recreate

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

deploy 可以自定义升级策略，其中升级的策略可以是 Recreate 和 RollingUpdate。
其中 RollingUpdate 是默认值，下面演示 Recreate 升级策略。

## 示例

### Deployment.yaml

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: busybox
spec:
  strategy:
    type: Recreate
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
busybox-59c95c7d4b-wzwjr   1/1     Terminating         0          81s
busybox-59c95c7d4b-6p7gq   1/1     Terminating         0          81s
busybox-59c95c7d4b-wj725   1/1     Terminating         0          81s
busybox-59c95c7d4b-wkdbn   1/1     Terminating         0          81s
busybox-59c95c7d4b-wj725   0/1     Terminating         0          87s
busybox-59c95c7d4b-wkdbn   0/1     Terminating         0          87s
busybox-59c95c7d4b-6p7gq   0/1     Terminating         0          87s
busybox-59c95c7d4b-wzwjr   0/1     Terminating         0          87s
busybox-59c95c7d4b-wkdbn   0/1     Terminating         0          88s
busybox-59c95c7d4b-wkdbn   0/1     Terminating         0          88s
busybox-59c95c7d4b-wzwjr   0/1     Terminating         0          97s
busybox-59c95c7d4b-wzwjr   0/1     Terminating         0          97s
busybox-59c95c7d4b-6p7gq   0/1     Terminating         0          97s
busybox-59c95c7d4b-6p7gq   0/1     Terminating         0          97s
busybox-59c95c7d4b-wj725   0/1     Terminating         0          98s
busybox-59c95c7d4b-wj725   0/1     Terminating         0          98s
busybox-76bcb59645-l57k6   0/1     Pending             0          0s
busybox-76bcb59645-ndw29   0/1     Pending             0          0s
busybox-76bcb59645-dfdw8   0/1     Pending             0          0s
busybox-76bcb59645-l57k6   0/1     Pending             0          0s
busybox-76bcb59645-ndw29   0/1     Pending             0          0s
busybox-76bcb59645-dfdw8   0/1     Pending             0          0s
busybox-76bcb59645-4djmv   0/1     Pending             0          0s
busybox-76bcb59645-4djmv   0/1     Pending             0          0s
busybox-76bcb59645-ndw29   0/1     ContainerCreating   0          0s
busybox-76bcb59645-4djmv   0/1     ContainerCreating   0          0s
busybox-76bcb59645-l57k6   0/1     ContainerCreating   0          0s
busybox-76bcb59645-dfdw8   0/1     ContainerCreating   0          0s
busybox-76bcb59645-4djmv   1/1     Running             0          2s
busybox-76bcb59645-ndw29   1/1     Running             0          9s
busybox-76bcb59645-dfdw8   1/1     Running             0          9s
busybox-76bcb59645-l57k6   1/1     Running             0          10s
```

可以看到旧的 Pod 被全部终止，并且新的还没有创建出来，这样会导致服务一段时间不可用。

## 总结

Recreate 的升级策略就是完全删除重建。

## 附录
