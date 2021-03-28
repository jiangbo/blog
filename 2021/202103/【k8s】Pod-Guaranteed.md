# 【k8s】Pod-Guaranteed

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

对于 QoS 类为 Guaranteed 的 Pod：

- Pod 中的每个容器，包含初始化容器，必须指定内存请求和内存限制，并且两者要相等。
- Pod 中的每个容器，包含初始化容器，必须指定 CPU 请求和 CPU 限制，并且两者要相等。

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
      resources:
        limits:
          cpu: 100m
          memory: 100Mi
```

### 查看

```
[root@master ~]# kubectl get pod spring-k8s -o json | jq .status.qosClass
"Guaranteed"
```

## 总结

介绍了怎么设置 Guaranteed 服务质量。

## 附录
