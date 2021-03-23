# 【Kubernetes】env 注入字段值

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M2

## 目标

通过 env 设置环境变量，将 k8s 的字段信息写入环境变量。

## 注入字段信息

### 支持的字段

1. metadata.name
2. metadata.namespace
3. metadata.labels['<KEY>']
4. metadata.annotations['<KEY>']
5. spec.nodeName
6. spec.serviceAccountName
7. status.hostIP
8. status.podIP
9. status.podIPs

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
        - name: K8S_NODE_NAME
          valueFrom:
            fieldRef:
              fieldPath: spec.nodeName
      resources:
        limits:
          memory: "128Mi"
          cpu: "500m"
      command: ["sleep", "3600"]
```

### K8S_NODE_NAME

```
[root@master ~]# kubectl exec busybox -- printenv | grep K8S
K8S_NODE_NAME=node2
```

## 总结

通过使用 env 和 fieldRef，将 k8s 的字段变成环境变量注入到了容器中。

## 附录
