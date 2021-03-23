# 【Kubernetes】镜像拉取策略-Never

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M2

## 目标

将镜像拉取策略设置成 Never 的情况下，不会主动拉取镜像。

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
      imagePullPolicy: Never
      resources:
        limits:
          memory: "128Mi"
          cpu: "500m"
      command: ["sleep", "3600"]
```

### 查看

```
Events:
  Type     Reason             Age              From               Message
  ----     ------             ----             ----               -------
  Normal   Scheduled          10s              default-scheduler  Successfully assigned default/busybox to node2
  Warning  ErrImageNeverPull  9s (x2 over 9s)  kubelet            Container image "busybox" is not present with pull policy of Never
  Warning  Failed             9s (x2 over 9s)  kubelet            Error: ErrImageNeverPull
```

可以看到事件中，出现拉取镜像的事件。

## 总结

通过将镜像的拉取策略设置成 Never，来验证 k8s 对镜像的拉取策略。

## 附录
