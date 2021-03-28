# 【k8s】Pod-qosClass

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

qosClass 表示服务质量类型（Quality of Service），这个字段是根据请求的内存和 CPU 来进行确定的。
其中包含三种类型：Guaranteed，Burstable 和 BestEffort。
其中这三种策略在由于资源不足而驱逐 Pod 时，有不同的优先级。

可以简单理解先驱逐 BestEffort，再 Burstable，最后是 Guaranteed。

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

介绍了 qosClass 是什么，以及有什么作用，没有配置资源限制的情况下，默认为 BestEffort。

## 附录
