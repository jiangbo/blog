# 【k8s】Pod-BestEffort 

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

Pod 中的容器没有设置内存和 CPU 限制或请求，则就是 BestEffort。

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
[root@master ~]# kubectl get pod spring-k8s -o json | jq .status.qosClass
"BestEffort"
```

## 总结

介绍了怎么设置 BestEffort 服务质量。

## 附录
