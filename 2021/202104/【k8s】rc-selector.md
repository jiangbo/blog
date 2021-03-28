# 【k8s】rc-selector

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

selector 可以定义标签选择器，只能定义键值对形式的选择器。
如果不写，默认和 Pod 定义中的标签一致。

## 示例

### rc.yaml

```yaml
apiVersion: v1
kind: ReplicationController
metadata:
  name: spring-k8s
spec:
  selector:
    app: spring-k8s
  template:
    metadata:
      labels:
        app: spring-k8s
        type: spring
    spec:
      containers:
        - name: spring-k8s
          image: jiangbo920827/spring-k8s:liveness
          ports:
            - containerPort: 8080
```

### 查看

```
[root@master manifests]# kubectl get pod --show-labels
NAME               READY   STATUS    RESTARTS   AGE   LABELS
spring-k8s-gq492   1/1     Running   0          50s   app=spring-k8s,type=spring
```

## 总结

selector 可以定义标签选择器。

## 附录
