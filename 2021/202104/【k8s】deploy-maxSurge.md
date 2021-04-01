# 【k8s】deploy-maxSurge

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

deploy 在更新过程中，Pod 数量可以超过定义的数量，超过的最大的值就叫 maxSurge。
该值可以是一个百分比，也可以是一个具体的数字，默认情况下，该值为 25%。

## 示例

### Deployment.yaml

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: busybox
spec:
  strategy:
    rollingUpdate:
      maxSurge: 0
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
busybox-76bcb59645-n9w5f   1/1     Terminating         0          48s
busybox-59c95c7d4b-wr6bd   0/1     Pending             0          0s
busybox-59c95c7d4b-wr6bd   0/1     Pending             0          0s
busybox-59c95c7d4b-wr6bd   0/1     ContainerCreating   0          0s
busybox-59c95c7d4b-wr6bd   1/1     Running             0          2s
busybox-76bcb59645-gjg7m   1/1     Terminating         0          50s
busybox-59c95c7d4b-fmzvx   0/1     Pending             0          0s
busybox-59c95c7d4b-fmzvx   0/1     Pending             0          0s
busybox-59c95c7d4b-fmzvx   0/1     ContainerCreating   0          0s
busybox-59c95c7d4b-fmzvx   1/1     Running             0          2s
busybox-76bcb59645-pjfv7   1/1     Terminating         0          52s
busybox-59c95c7d4b-9pv8l   0/1     Pending             0          0s
busybox-59c95c7d4b-9pv8l   0/1     Pending             0          0s
busybox-59c95c7d4b-9pv8l   0/1     ContainerCreating   0          0s
busybox-59c95c7d4b-9pv8l   1/1     Running             0          1s
busybox-76bcb59645-qg4ws   1/1     Terminating         0          53s
busybox-59c95c7d4b-nx6hr   0/1     Pending             0          0s
busybox-59c95c7d4b-nx6hr   0/1     Pending             0          0s
busybox-59c95c7d4b-nx6hr   0/1     ContainerCreating   0          0s
busybox-76bcb59645-n9w5f   0/1     Terminating         0          54s
busybox-76bcb59645-gjg7m   0/1     Terminating         0          56s
busybox-76bcb59645-n9w5f   0/1     Terminating         0          57s
busybox-76bcb59645-n9w5f   0/1     Terminating         0          57s
busybox-76bcb59645-pjfv7   0/1     Terminating         0          58s
busybox-76bcb59645-gjg7m   0/1     Terminating         0          67s
busybox-76bcb59645-gjg7m   0/1     Terminating         0          67s
busybox-76bcb59645-pjfv7   0/1     Terminating         0          67s
busybox-76bcb59645-pjfv7   0/1     Terminating         0          67s
busybox-59c95c7d4b-nx6hr   1/1     Running             0          15s
busybox-76bcb59645-qg4ws   0/1     Terminating         0          68s
busybox-76bcb59645-qg4ws   0/1     Terminating         0          78s
busybox-76bcb59645-qg4ws   0/1     Terminating         0          78s
```

因为将 maxSurge，最大超出数量设置成了 0，所以无论如何都不会超过定义的数量。
都是先减少再新增，也就是说，更新过程中，只会出现缺少服务数量的情况，不会多。

## 总结

maxSurge 定义了更新过程中，超出定义的数量，可以是一个数值，也可以是百分比。

## 附录
