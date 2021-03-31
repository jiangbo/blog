# 【k8s】deploy-revisionHistoryLimit

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

deploy 升级最大记录数由 revisionHistoryLimit 定义，默认值为 10。

## 示例

### Deployment.yaml

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: busybox
spec:
  revisionHistoryLimit: 1
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

然后将版本升级到 1.31.0，再升级到 1.32.0。

### 查看

```
[root@master ~]# kubectl get rs
NAME                 DESIRED   CURRENT   READY   AGE
busybox-59c95c7d4b   0         0         0       2m47s
busybox-76bcb59645   8         8         8       58s
[root@master ~]# kubectl rollout history deployment busybox
deployment.apps/busybox
REVISION  CHANGE-CAUSE
2         <none>
3         <none>
```

可以看到有两个 rs，一个当前使用的，一个是配置保留的。
并且配置是最大保留一个，所以只有一个历史记录，另一个被丢弃了。

## 总结

revisionHistoryLimit 可以定义保留的升级记录数。

## 附录
