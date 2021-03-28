# 【k8s】Pod-status

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

Pod 的状态字段都是一些只读的字段，可以给我们提供额外的信息。

## 示例

### Pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
spec:
  containers:
    - name: spring-k8s
      image: jiangbo920827/spring-k8s:liveness
      ports:
        - containerPort: 8080
```

### 查看

```
status:
  conditions:
  - lastProbeTime: null
    lastTransitionTime: "2021-03-28T08:13:24Z"
    status: "True"
    type: Initialized
  - lastProbeTime: null
    lastTransitionTime: "2021-03-28T08:13:26Z"
    status: "True"
    type: Ready
  - lastProbeTime: null
    lastTransitionTime: "2021-03-28T08:13:26Z"
    status: "True"
    type: ContainersReady
  - lastProbeTime: null
    lastTransitionTime: "2021-03-28T08:13:24Z"
    status: "True"
    type: PodScheduled
  containerStatuses:
  - containerID: docker://215ff1c529fa6a942550fddf82298383afeeb3af88213114aedd74381e7560c6
    image: jiangbo920827/spring-k8s:liveness
    imageID: docker://sha256:27e1956a7558e66cc463d09c86bcda059fd6534d520a9ab68fb8567048f786f2
    lastState: {}
    name: spring-k8s
    ready: true
    restartCount: 0
    started: true
    state:
      running:
        startedAt: "2021-03-28T08:13:25Z"
  hostIP: 192.168.56.103
  phase: Running
  podIP: 10.244.2.182
  podIPs:
  - ip: 10.244.2.182
  qosClass: BestEffort
  startTime: "2021-03-28T08:13:24Z"
```

conditions 和 containerStatuses 字段后面学习；
hostIP 表示 Pod 调度的宿主机的 IP 地址；
phase 表示 Pod 的阶段，目前是 Running；
podIP 和 podIPs 表示 Pod 的 IP 地址;
qosClass 后面学习;
startTime 表示启动时间。


## 总结

了解了 Pod 的 status 字段里的一些字段的含义。

## 附录
