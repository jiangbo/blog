# 【k8s】deploy-progressDeadlineSeconds

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

deploy 升级过程中的最大时间由 progressDeadlineSeconds 来定义。

## 示例

### Deployment.yaml

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: busybox
spec:
  progressDeadlineSeconds: 3
  selector:
    matchLabels:
      app: busybox
  replicas: 18
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
[root@master ~]# kubectl get pod
NAME                       READY   STATUS              RESTARTS   AGE
busybox-6bd65c67cf-2lq2x   0/1     ContainerCreating   0          35s
busybox-6bd65c67cf-44j46   1/1     Running             0          35s
busybox-6bd65c67cf-4hchv   0/1     ContainerCreating   0          35s
busybox-6bd65c67cf-bll7f   1/1     Running             0          35s
busybox-6bd65c67cf-hm7n7   0/1     ContainerCreating   0          35s
busybox-6bd65c67cf-jd7w9   0/1     ContainerCreating   0          35s
busybox-6bd65c67cf-jf5s9   1/1     Running             0          35s
busybox-6bd65c67cf-m47zs   1/1     Running             0          35s
busybox-6bd65c67cf-m9cl4   1/1     Running             0          35s
busybox-6bd65c67cf-nm8wt   1/1     Running             0          35s
busybox-6bd65c67cf-qdsmq   1/1     Running             0          35s
busybox-6bd65c67cf-rnpgl   0/1     ContainerCreating   0          35s
busybox-6bd65c67cf-s4nnx   0/1     ContainerCreating   0          35s
busybox-6bd65c67cf-sp87l   1/1     Running             0          35s
busybox-6bd65c67cf-trbrv   0/1     ContainerCreating   0          35s
busybox-6bd65c67cf-vjct8   0/1     ContainerCreating   0          35s
busybox-6bd65c67cf-vspqb   0/1     ContainerCreating   0          35s
busybox-6bd65c67cf-zljwk   0/1     ContainerCreating   0          35s
```

### 查看状态

```
[root@master ~]# kubectl rollout status deployment busybox
Waiting for deployment "busybox" rollout to finish: 4 of 18 updated replicas are available...
Waiting for deployment "busybox" rollout to finish: 5 of 18 updated replicas are available...
Waiting for deployment "busybox" rollout to finish: 6 of 18 updated replicas are available...
Waiting for deployment "busybox" rollout to finish: 7 of 18 updated replicas are available...
Waiting for deployment "busybox" rollout to finish: 8 of 18 updated replicas are available...
error: deployment "busybox" exceeded its progress deadline
```

可以看到超过最大时间没有升级完成，就会变成超时状态。

## 总结

progressDeadlineSeconds 定义 deploy 升级的最大时间。

## 附录
