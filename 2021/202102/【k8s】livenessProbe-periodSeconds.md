# 【k8s】livenessProbe-periodSeconds

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

periodSeconds 表示探针的探测周期，默认情况下是 10 秒。
下面演示将周期修改成 5 秒。

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

2021-03-25 16:17:56.209 TRACE 1 --- [nio-8080-exec-2] s.w.s.m.m.a.RequestMappingHandlerAdapter : Applying default cacheSeconds=-1
2021-03-25 16:17:56.210 TRACE 1 --- [nio-8080-exec-2] o.s.web.servlet.DispatcherServlet        : No view rendering, null ModelAndView returned.
2021-03-25 16:17:56.210 DEBUG 1 --- [nio-8080-exec-2] o.s.web.servlet.DispatcherServlet        : Completed 200 OK, headers={masked}
2021-03-25 16:18:01.199 TRACE 1 --- [nio-8080-exec-5] o.s.web.servlet.DispatcherServlet        : GET "/actuator/health/liveness", parameters={}, headers={masked} in DispatcherServlet 'dispatcherServlet'
2021-03-25 16:18:01.199 TRACE 1 --- [nio-8080-exec-5] s.b.a.e.w.s.WebMvcEndpointHandlerMapping : Mapped to Actuator web endpoint 'health-path'
2021-03-25 16:18:01.200 TRACE 1 --- [nio-8080-exec-5] o.s.web.method.HandlerMethod             : Arguments: [org.apache.catalina.connector.RequestFacade@29d96d2f, null]
2021-03-25 16:18:01.200 DEBUG 1 --- [nio-8080-exec-5] o.s.w.s.m.m.a.HttpEntityMethodProcessor  : Using 'application/vnd.spring-boot.actuator.v3+json', given [*/*] and supported [application/vnd.spring-boot.actuator.v3+json, application/vnd.spring-boot.actuator.v2+json, application/json]
2021-03-25 16:18:01.200 TRACE 1 --- [nio-8080-exec-5] o.s.w.s.m.m.a.HttpEntityMethodProcessor  : Writing [org.springframework.boot.actuate.health.CompositeHealth@7294e8dd]
2021-03-25 16:18:01.208 TRACE 1 --- [nio-8080-exec-5] s.w.s.m.m.a.RequestMappingHandlerAdapter : Applying default cacheSeconds=-1
2021-03-25 16:18:01.209 TRACE 1 --- [nio-8080-exec-5] o.s.web.servlet.DispatcherServlet        : No view rendering, null ModelAndView returned.
2021-03-25 16:18:01.209 DEBUG 1 --- [nio-8080-exec-5] o.s.web.servlet.DispatcherServlet        : Completed 200 OK, headers={masked}
```

可以看到每过 5 秒，就收到了存活探针的请求。

## 总结

通过定义 periodSeconds 来设置探针的探测间隔。

## 附录
