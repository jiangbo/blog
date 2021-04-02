# 【k8s】DaemonSet

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

DaemonSet 简写为 ds。新增该种资源，会在满足条件的所有 Node 上启动一个 Pod。
节点的数量变化时，它也会同步的增加或者减少。该种资源和 Deploy 类似，不过不会新建 rs，
并且会自动添加一些容忍度。这个适合一些系统组件，比如网络插件，日志收集程序，和监控程序。

## 示例

### DaemonSet.yaml

```yaml
apiVersion: apps/v1
kind: DaemonSet
metadata:
  name: spring-k8s
spec:
  selector:
    matchLabels:
      app: spring-k8s
  template:
    metadata:
      labels:
        app: spring-k8s
    spec:
      containers:
        - name: spring-k8s
          image: jiangbo920827/spring-k8s:liveness
          ports:
            - containerPort: 8080
```

### 查看

```
[root@master ~]# kubectl describe ds spring-k8s
Name:           spring-k8s
Selector:       app=spring-k8s
Node-Selector:  <none>
Labels:         <none>
Annotations:    deprecated.daemonset.template.generation: 1
Desired Number of Nodes Scheduled: 2
Current Number of Nodes Scheduled: 2
Number of Nodes Scheduled with Up-to-date Pods: 2
Number of Nodes Scheduled with Available Pods: 2
Number of Nodes Misscheduled: 0
Pods Status:  2 Running / 0 Waiting / 0 Succeeded / 0 Failed
Pod Template:
  Labels:  app=spring-k8s
  Containers:
   spring-k8s:
    Image:        jiangbo920827/spring-k8s:liveness
    Port:         8080/TCP
    Host Port:    0/TCP
    Environment:  <none>
    Mounts:       <none>
  Volumes:        <none>
Events:
  Type    Reason            Age   From                  Message
  ----    ------            ----  ----                  -------
  Normal  SuccessfulCreate  13m   daemonset-controller  Created pod: spring-k8s-4ps4f
  Normal  SuccessfulCreate  13m   daemonset-controller  Created pod: spring-k8s-57hfk
```

## 总结

DaemonSet 会自动在每个节点上创建一个且仅一个 Pod。

## 附录
