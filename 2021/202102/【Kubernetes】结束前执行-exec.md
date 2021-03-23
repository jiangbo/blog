# 【Kubernetes】结束前执行-exec

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M2

## 目标

在容器结束前，执行一个命令。

## 执行命令

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
      lifecycle:
        preStop:
          exec:
            command: ["/bin/sh", "-c", "echo godbye ${HOSTNAME}! > /name.txt"]
      resources:
        limits:
          memory: "128Mi"
          cpu: "500m"
      command: ["sleep", "3600"]
```

### 查看

需要先使用 `kubectl delete pod busybox` 命令来触发删除 Pod 的操作。

```
[root@master ~]# kubectl exec busybox -- cat /name.txt
godbye busybox!
```

## 总结

通过参与容器的生命周期，在容器结束前，在容器中执行了一个命令。

## 附录
