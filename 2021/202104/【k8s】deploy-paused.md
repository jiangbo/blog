# 【k8s】deploy-paused

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

paused 表示暂停，在 deploy 升级的过程中，可以暂停升级。

## 示例

### Deployment.yaml

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: busybox
spec:
  paused: true
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

### 查看

```
[root@master ~]# kubectl get deployments.apps,rs,pod
NAME                      READY   UP-TO-DATE   AVAILABLE   AGE
deployment.apps/busybox   0/8     0            0           35s
```

可以看到只有 deploy 其它的都没有生成。

### 继续创建

```
[root@master ~]# kubectl rollout resume deployment busybox
deployment.apps/busybox resumed
[root@master ~]# kubectl get pod
NAME                       READY   STATUS              RESTARTS   AGE
busybox-6bd65c67cf-4tmcn   0/1     ContainerCreating   0          4s
busybox-6bd65c67cf-5mfxw   0/1     ContainerCreating   0          4s
busybox-6bd65c67cf-kpfqj   1/1     Running             0          4s
busybox-6bd65c67cf-kx9m9   1/1     Running             0          4s
busybox-6bd65c67cf-mtpdb   0/1     ContainerCreating   0          4s
busybox-6bd65c67cf-sl4q2   1/1     Running             0          4s
busybox-6bd65c67cf-t9h8l   1/1     Running             0          4s
busybox-6bd65c67cf-xvjx9   1/1     Running             0          4s
[root@master ~]#
```

## 总结

paused 可以暂停 deploy 的升级操作。

## 附录
