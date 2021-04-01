# 【k8s】deploy-maxUnavailable

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

deploy 在更新过程中，Pod 数量可以低于定义的数量，低于定义的数量最大值就叫 maxUnavailable。
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
busybox-59c95c7d4b-6c2tc   0/1     Pending   0          0s
busybox-59c95c7d4b-6c2tc   0/1     Pending   0          0s
busybox-59c95c7d4b-6c2tc   0/1     ContainerCreating   0          0s
busybox-59c95c7d4b-6c2tc   1/1     Running             0          2s
busybox-6bd65c67cf-rfhd8   1/1     Terminating         0          4m53s
busybox-59c95c7d4b-vjhst   0/1     Pending             0          0s
busybox-59c95c7d4b-vjhst   0/1     Pending             0          0s
busybox-59c95c7d4b-vjhst   0/1     ContainerCreating   0          0s
busybox-59c95c7d4b-vjhst   1/1     Running             0          4s
busybox-6bd65c67cf-dt7p2   1/1     Terminating         0          4m57s
busybox-59c95c7d4b-wjxt5   0/1     Pending             0          0s
busybox-59c95c7d4b-wjxt5   0/1     Pending             0          0s
busybox-59c95c7d4b-wjxt5   0/1     ContainerCreating   0          0s
busybox-59c95c7d4b-wjxt5   1/1     Running             0          2s
busybox-6bd65c67cf-rfhd8   0/1     Terminating         0          4m59s
busybox-6bd65c67cf-jxvwc   1/1     Terminating         0          4m59s
busybox-59c95c7d4b-r9f5w   0/1     Pending             0          0s
busybox-59c95c7d4b-r9f5w   0/1     Pending             0          0s
busybox-59c95c7d4b-r9f5w   0/1     ContainerCreating   0          0s
busybox-59c95c7d4b-r9f5w   1/1     Running             0          2s
busybox-6bd65c67cf-fnwqd   1/1     Terminating         0          5m1s
busybox-6bd65c67cf-dt7p2   0/1     Terminating         0          5m3s
busybox-6bd65c67cf-jxvwc   0/1     Terminating         0          5m5s
busybox-6bd65c67cf-fnwqd   0/1     Terminating         0          5m7s
busybox-6bd65c67cf-rfhd8   0/1     Terminating         0          5m8s
busybox-6bd65c67cf-rfhd8   0/1     Terminating         0          5m8s
busybox-6bd65c67cf-dt7p2   0/1     Terminating         0          5m8s
busybox-6bd65c67cf-dt7p2   0/1     Terminating         0          5m8s
busybox-6bd65c67cf-jxvwc   0/1     Terminating         0          5m8s
busybox-6bd65c67cf-jxvwc   0/1     Terminating         0          5m9s
```

因为将 maxUnavailable，最大不可用数量设置成了 0，所以无论如何都不会低于定义的数量。
都是先增加再减少，也就是说，更新过程中，只会出现多于服务数量的情况，不会少。

## 总结

maxUnavailable 定义了更新过程中，低于定义的数量，可以是一个数值，也可以是百分比。

## 附录
