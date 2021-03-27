# 【k8s】Container-startupProbe

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

startupProbe 表示启动探针，和之前学习的两种探针字段一致。
启动探针在成功之前，另外的探针不会启动。这个主要用于启动事件长且不固定的容器。
在探测到成功之后，启动探针不会再进行探测了。

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
      startupProbe:
        httpGet:
          port: my-port
          path: /actuator/health/liveness
      ports:
        - name: my-port
          containerPort: 8080
```

### 查看就绪探针日志

Spring Boot 还没有启动探针的端点，将启动探针配置到存活探针的上面，看看效果。

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

如果服务的启动时间很长，可以单独配置启动探针。

## 附录
