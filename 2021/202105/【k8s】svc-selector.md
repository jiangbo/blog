# 【k8s】svc-selector

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

selector 可以定义选择器，如果这个选择器选中了 Pod，那么会自动将 Pod 的地址关联到服务上。
Pod 的 IP 地址变化了也会自动更新。如果没有定义选择器，那么就需要自己维护 ep 端点。

## 示例

### Deploy.yaml

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: spring-k8s
spec:
  selector:
    matchLabels:
      app: spring-k8s
  template:
    metadata:
      labels:
        app: spring-k8s
    spec:
      containers:
        - name: spring-k8s
          image: jiangbo920827/spring-k8s:liveness
          ports:
            - containerPort: 8080
```

### Service.yaml

```yaml
apiVersion: v1
kind: Service
metadata:
  name: spring-k8s
spec:
  selector:
    app: spring-k8s
  ports:
    - port: 80
      targetPort: 8080
```

### 访问测试

```
[root@master ~]# curl 10.104.121.41/hostname;echo
spring-k8s-d8ccf5994-zbxzg
```

## 总结

selector 可以定义选择器，将选中的 Pod 的地址自动关联到服务上。

## 附录
