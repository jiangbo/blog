# 【k8s】Pod-containerStatuses

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

Pod 的 containerStatuses 代表了其中运行的容器的状态。

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
```

包括了容器的 ID，容器使用的镜像，镜像的 ID，容器的名称，重启次数，启动时间等。

## 总结

介绍了 Pod 的 containerStatuses 字段，表示了 Pod 中容器的状态。

## 附录
