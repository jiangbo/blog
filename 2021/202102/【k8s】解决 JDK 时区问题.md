# 【k8s】解决 JDK 时区问题

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

在查看 Spring Boot 项目输出的时候，看到时间不正确，比当前时间少 8 个小时，
这是由于时区不对导致的，可以通过环境变量的方式，设置正确的时区。

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
      image: jiangbo920827/spring-k8s:trace
      env:
        - name: TZ
          value: Asia/Shanghai
      livenessProbe:
        httpGet:
          path: /actuator/health/liveness
          port: http
        periodSeconds: 5
      ports:
        - name: http
          containerPort: 8080
```

### 查看

```
2021-03-26 21:25:49 - Writing [org.springframework.boot.actuate.health.CompositeHealth@259a39fd]
2021-03-26 21:25:49 - Applying default cacheSeconds=-1
2021-03-26 21:25:49 - No view rendering, null ModelAndView returned.
2021-03-26 21:25:49 - Completed 200 OK, headers={masked}
2021-03-26 21:25:54 - GET "/actuator/health/liveness", parameters={}, headers={masked} in DispatcherServlet 'dispatcherServlet'
2021-03-26 21:25:54 - Mapped to Actuator web endpoint 'health-path'
2021-03-26 21:25:54 - Arguments: [org.apache.catalina.connector.RequestFacade@55c857ec, null]
2021-03-26 21:25:54 - Using 'application/vnd.spring-boot.actuator.v3+json', given [*/*] and supported [application/vnd.spring-boot.actuator.v3+json, application/vnd.spring-boot.actuator.v2+json, application/json]
2021-03-26 21:25:54 - Writing [org.springframework.boot.actuate.health.CompositeHealth@6d4bf29b]
2021-03-26 21:25:54 - Applying default cacheSeconds=-1
2021-03-26 21:25:54 - No view rendering, null ModelAndView returned.
2021-03-26 21:25:54 - Completed 200 OK, headers={masked}
```

再次查看日志，可以看到时间已经和当前时间一致了。

## 总结

通过设置时区的方式，来解决 JDK 镜像时区不正确的问题。

## 附录
