# 【k8s】Pod-conditions

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

Pod 的 conditions 表示了 Pod 的一些条件，是一个数组。
里面包含一些 Pod 必须满足的条件，只有所有的条件为 True 时，Pod 才可以提供服务。

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
```
默认情况下，包含四个条件，它们分别为：

- `PodScheduled`：Pod 已经被调度到某节点；
- `ContainersReady`：Pod 中所有容器都已就绪；
- `Initialized`：所有的 Init 容器都已成功启动；
- `Ready`：Pod 可以为请求提供服务。


## 总结

介绍了 Pod 默认的四个条件，以及每个条件所代表的含义。

## 附录
