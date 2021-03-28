# 【k8s】Pod-Burstable

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

如果满足下面条件，将会指定 Pod 的 QoS 类为 Burstable：

- Pod 不符合 Guaranteed QoS 类的标准。
- Pod 中至少一个容器具有内存或 CPU 请求。

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
        requests:
          cpu: 100m
```

### 查看

```
[root@master ~]# kubectl get pod spring-k8s -o json | jq .status.qosClass
"Burstable"
```

## 总结

介绍了怎么设置 Burstable 服务质量。

## 附录
