# 【k8s】Container-ports-name

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

在指定容器的端口时，可以指定一个名称，其它地方使用这个端口时，可以直接通过名称引用。
protocol 在指定端口时，可以定义协议，默认情况下，是 TCP 协议。

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
      livenessProbe:
        httpGet:
          path: /actuator/health/liveness
          port: my-port
      ports:
        - name: my-port
          containerPort: 8080
```

### 查看存活探针

```
2021-03-27 07:20:02 - GET "/actuator/health/liveness", parameters={}
2021-03-27 07:20:02 - Mapped to Actuator web endpoint 'health-path'
2021-03-27 07:20:02 - Using 'application/vnd.spring-boot.actuator.v3+json', given [*/*] and supported [application/vnd.spring-boot.actuator.v3+json, application/vnd.spring-boot.actuator.v2+json, application/json]
2021-03-27 07:20:02 - Writing [org.springframework.boot.actuate.health.CompositeHealth@9f4b768]
2021-03-27 07:20:02 - Completed 200 OK
2021-03-27 07:20:12 - GET "/actuator/health/liveness", parameters={}
2021-03-27 07:20:12 - Mapped to Actuator web endpoint 'health-path'
2021-03-27 07:20:12 - Using 'application/vnd.spring-boot.actuator.v3+json', given [*/*] and supported [application/vnd.spring-boot.actuator.v3+json, application/vnd.spring-boot.actuator.v2+json, application/json]
2021-03-27 07:20:12 - Writing [org.springframework.boot.actuate.health.CompositeHealth@6741996c]
2021-03-27 07:20:12 - Completed 200 OK
```

## 总结

在定义容器的端口时，可以指定一个名称，以便其它地方引用。

## 附录
