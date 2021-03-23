# 【Kubernetes】镜像拉取策略-Always

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M2

## 目标

将镜像拉取策略设置成 Always 的情况下，每次启动 Pod 都会拉取镜像。

## 镜像拉取策略

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
      imagePullPolicy: Always
      resources:
        limits:
          memory: "128Mi"
          cpu: "500m"
      command: ["sleep", "3600"]
```

### 查看

```
Events:
  Type    Reason     Age    From               Message
  ----    ------     ----   ----               -------
  Normal  Scheduled  4m38s  default-scheduler  Successfully assigned default/busybox to node2
  Normal  Pulling    4m37s  kubelet            Pulling image "busybox:stable"
  Normal  Pulled     4m20s  kubelet            Successfully pulled image "busybox:stable" in 17.00572703s
  Normal  Created    4m20s  kubelet            Created container busybox
  Normal  Started    4m20s  kubelet            Started container busybox
```

可以看到事件中，出现拉取镜像的事件。

## 总结

通过将镜像的拉取策略设置成 Always，来验证 k8s 对镜像的拉取策略。

## 附录
