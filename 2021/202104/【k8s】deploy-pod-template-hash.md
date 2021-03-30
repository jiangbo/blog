# 【k8s】deploy-pod-template-hash

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

pod-template-hash 标签是 deploy 自动给它生成 rs 和 pod 加上的。
标签的值是 rs 名称的后缀，一是让 rs 生成不重复，而是可以唯一识别 rs。
deploy 可以生成多个 rs，其中这个标签的值也会跟着变化。

>不要修改此标签，否则 deploy 不能正常管理 rs 和 pod。

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

### 查看 rs

```
[root@master ~]# kubectl describe rs
Name:           busybox-6bd65c67cf
Namespace:      default
Selector:       app=busybox,pod-template-hash=6bd65c67cf
Labels:         app=busybox
                pod-template-hash=6bd65c67cf
Annotations:    deployment.kubernetes.io/desired-replicas: 4
                deployment.kubernetes.io/max-replicas: 5
                deployment.kubernetes.io/revision: 1
Controlled By:  Deployment/busybox
Replicas:       4 current / 4 desired
Pods Status:    4 Running / 0 Waiting / 0 Succeeded / 0 Failed
Pod Template:
  Labels:  app=busybox
           pod-template-hash=6bd65c67cf
  Containers:
   busybox:
    Image:      busybox:1.30.0
    Port:       <none>
    Host Port:  <none>
    Command:
      /bin/sh
      -c
      sleep 3600
    Environment:  <none>
    Mounts:       <none>
  Volumes:        <none>
Events:
  Type    Reason            Age    From                   Message
  ----    ------            ----   ----                   -------
  Normal  SuccessfulCreate  7m33s  replicaset-controller  Created pod: busybox-6bd65c67cf-9hz9g
  Normal  SuccessfulCreate  7m33s  replicaset-controller  Created pod: busybox-6bd65c67cf-frbcn
  Normal  SuccessfulCreate  7m33s  replicaset-controller  Created pod: busybox-6bd65c67cf-98xb7
  Normal  SuccessfulCreate  7m33s  replicaset-controller  Created pod: busybox-6bd65c67cf-6xphf
```

pod-template-hash 标签的值和 rs 名称后缀相同。

### 查看 pod

```
[root@master ~]# kubectl describe pod busybox-6bd65c67cf-6xphf
Name:         busybox-6bd65c67cf-6xphf
Namespace:    default
Priority:     0
Node:         node1/192.168.56.102
Start Time:   Tue, 30 Mar 2021 22:05:36 +0800
Labels:       app=busybox
              pod-template-hash=6bd65c67cf
Annotations:  <none>
Status:       Running
IP:           10.244.1.248
IPs:
  IP:           10.244.1.248
Controlled By:  ReplicaSet/busybox-6bd65c67cf
Containers:
  busybox:
...
```

具有和 rs 一样的 pod-template-hash 标签。

## 总结

查看 Deployment 中的一些字段。

## 附录
