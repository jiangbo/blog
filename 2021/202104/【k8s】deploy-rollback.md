# 【k8s】deploy-rollback

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

deploy 除了可以方便地进行上线操作之外，还可以方便地执行回滚操作。

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
`kubectl set image deployment/busybox busybox=busybox:1.32.0 --record`

执行两次升级

### 查看更新历史

```
[root@master ~]# kubectl rollout history deployment busybox
deployment.apps/busybox
REVISION  CHANGE-CAUSE
1         <none>
2         kubectl set image deployment/busybox busybox=busybox:1.31.0 --record=true
3         kubectl set image deployment/busybox busybox=busybox:1.32.0 --record=true

```

可以看到有三次记录，分别创建时，第一次升级和第二次升级。

### 回滚到上一次

```
[root@master ~]# kubectl rollout undo deployment busybox
deployment.apps/busybox rolled back
```

### 回滚到指定版本

```
[root@master ~]# kubectl rollout history deployment busybox
deployment.apps/busybox
REVISION  CHANGE-CAUSE
1         <none>
3         kubectl set image deployment/busybox busybox=busybox:1.32.0 --record=true
4         kubectl set image deployment/busybox busybox=busybox:1.31.0 --record=true

[root@master ~]# kubectl rollout undo deployment busybox --to
--token         --token=        --to-revision   --to-revision=
[root@master ~]# kubectl rollout undo deployment busybox --to-revision=1
deployment.apps/busybox rolled back
```

## 总结

deploy 除了可以执行更新操作，还可以方便地执行回滚操作。可以回滚到上一次，也可以指定回滚到的版本。

## 附录
