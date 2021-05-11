# 【k8s】svc-sessionAffinityConfig

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

sessionAffinityConfig 可以配置会话亲和，目前可以配置的是 timeoutSeconds，即会话亲和的超时时间。
默认的超时时间为 10800 秒，即三个小时，最长不能超过一天。

## 示例

### Deploy.yaml

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: spring-k8s
spec:
  replicas: 4
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
  sessionAffinity: ClientIP
  sessionAffinityConfig:
    clientIP:
      timeoutSeconds: 10
  selector:
    app: spring-k8s
  ports:
    - port: 80
      targetPort: 8080
```

## 总结

sessionAffinityConfig 可以设置会话亲和性的超时时间。

## 附录
