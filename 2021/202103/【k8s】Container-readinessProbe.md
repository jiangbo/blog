# 【k8s】Container-readinessProbe

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

之前学习到了存活探针（livenessProbe），readinessProbe 表示就绪探针，它们的字段一样。
就绪探针表示的意思是：就绪探针成功了，才会将流量转发到容器里，否则不会有流量进来。

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
      readinessProbe:
        httpGet:
          port: my-port
          path: /actuator/health/readiness
      ports:
        - name: my-port
          containerPort: 8080
```

### 查看就绪探针日志

```
2021-03-27 08:10:26 - GET "/actuator/health/readiness", parameters={}
2021-03-27 08:10:26 - Mapped to Actuator web endpoint 'health-path'
2021-03-27 08:10:26 - Using 'application/vnd.spring-boot.actuator.v3+json', given [*/*] and supported [application/vnd.spring-boot.actuator.v3+json, application/vnd.spring-boot.actuator.v2+json, application/json]
2021-03-27 08:10:26 - Writing [org.springframework.boot.actuate.health.CompositeHealth@6741996c]
2021-03-27 08:10:26 - Completed 200 OK
2021-03-27 08:10:36 - GET "/actuator/health/readiness", parameters={}
2021-03-27 08:10:36 - Mapped to Actuator web endpoint 'health-path'
2021-03-27 08:10:36 - Using 'application/vnd.spring-boot.actuator.v3+json', given [*/*] and supported [application/vnd.spring-boot.actuator.v3+json, application/vnd.spring-boot.actuator.v2+json, application/json]
2021-03-27 08:10:36 - Writing [org.springframework.boot.actuate.health.CompositeHealth@2c86cb07]
2021-03-27 08:10:36 - Completed 200 OK
```

## 总结

如果服务的启动时间较长，可以配置就绪探针，避免服务还没有完全启动，就有请求分发到上面。

## 附录
