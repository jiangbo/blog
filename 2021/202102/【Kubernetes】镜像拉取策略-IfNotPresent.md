# 【Kubernetes】镜像拉取策略-IfNotPresent

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M2

## 目标

将镜像拉取策略设置成 IfNotPresent 的情况下，只有镜像不存在的情况下，才会去拉取镜像。
如果未指定，默认值为 IfNotPresent。

> 如果镜像的 tag 为 latest 或者省略，未指定拉取策略的情况下，每次都会拉取。

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
      imagePullPolicy: IfNotPresent
      resources:
        limits:
          memory: "128Mi"
          cpu: "500m"
      command: ["sleep", "3600"]
```

### 查看

```
Events:
  Type    Reason     Age   From               Message
  ----    ------     ----  ----               -------
  Normal  Scheduled  4s    default-scheduler  Successfully assigned default/busybox to node2
  Normal  Pulled     3s    kubelet            Container image "busybox:stable" already present on machine
  Normal  Created    3s    kubelet            Created container busybox
  Normal  Started    3s    kubelet            Started container busybox
```

可以看到事件中，显示出了镜像已经在本地存在，没有进行拉取。

## 总结

通过将镜像的拉取策略设置成 IfNotPresent，来验证 k8s 对镜像的拉取策略。

## 附录
