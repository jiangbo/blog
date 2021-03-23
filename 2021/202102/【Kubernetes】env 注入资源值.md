# 【Kubernetes】env 注入资源

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M2

## 目标

通过 env 设置环境变量，将 k8s 的资源信息写入环境变量。

## 注入资源信息

### 支持的资源

1. limits.cpu
2. limits.memory
3. limits.ephemeral-storage
4. requests.cpu
5. requests.memory
6. requests.ephemeral-storage

### Pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: busybox
spec:
  containers:
    - name: busybox
      image: busybox:stable
      env:
        - name: K8S_MEMORY_REQUEST
          valueFrom:
            resourceFieldRef:
              resource: requests.memory
              divisor: 1Mi
      resources:
        limits:
          memory: "128Mi"
          cpu: "500m"
      command: ["sleep", "3600"]

```

divisor 可以设置显示值的单位，默认是 1。

### K8S_MEMORY_REQUEST

```
[root@master ~]# kubectl exec busybox -- printenv | grep K8S
K8S_MEMORY_REQUEST=128
```

## 总结

通过使用 env 和 resourceFieldRef，将 k8s 的资源信息变成环境变量注入到了容器中。

## 附录
