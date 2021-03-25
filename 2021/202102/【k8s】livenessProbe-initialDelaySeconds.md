# 【k8s】livenessProbe-initialDelaySeconds

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

initialDelaySeconds 表示在容器启动后，延时多少秒才开始探测。
之前出现过进行存活探测时失败，就是因为容器启动后直接进行探测，里面的服务还未启动好。
下面演示延时 30 秒进行探测。

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
        initialDelaySeconds: 30
      ports:
        - name: http
          containerPort: 8080
```

### 查看

```
2021-03-25 15:54:01.158  INFO 1 --- [           main] o.s.b.w.embedded.tomcat.TomcatWebServer  : Tomcat started on port(s): 8080 (http) with context path ''
2021-03-25 15:54:01.198  INFO 1 --- [           main] j.spring.docker.SpringDemoApplication    : Started SpringDemoApplication in 5.533 seconds (JVM running for 6.106)
2021-03-25 15:54:27.568  INFO 1 --- [nio-8080-exec-1] o.a.c.c.C.[Tomcat].[localhost].[/]       : Initializing Spring DispatcherServlet 'dispatcherServlet'
2021-03-25 15:54:27.573  INFO 1 --- [nio-8080-exec-1] o.s.web.servlet.DispatcherServlet        : Initializing Servlet 'dispatcherServlet'
2021-03-25 15:54:27.574 TRACE 1 --- [nio-8080-exec-1] o.s.web.servlet.DispatcherServlet        : Detected org.springframework.web.multipart.support.StandardServletMultipartResolver@b751ecf
2021-03-25 15:54:27.574 TRACE 1 --- [nio-8080-exec-1] o.s.web.servlet.DispatcherServlet        : Detected org.springframework.web.servlet.i18n.AcceptHeaderLocaleResolver@483c9a07
2021-03-25 15:54:27.574 TRACE 1 --- [nio-8080-exec-1] o.s.web.servlet.DispatcherServlet        : Detected org.springframework.web.servlet.theme.FixedThemeResolver@16210ed3
2021-03-25 15:54:27.575 TRACE 1 --- [nio-8080-exec-1] o.s.web.servlet.DispatcherServlet        : Detected DefaultRequestToViewNameTranslator
2021-03-25 15:54:27.575 TRACE 1 --- [nio-8080-exec-1] o.s.web.servlet.DispatcherServlet        : Detected SessionFlashMapManager
2021-03-25 15:54:27.575 DEBUG 1 --- [nio-8080-exec-1] o.s.web.servlet.DispatcherServlet        : enableLoggingRequestDetails='false': request parameters and headers will be masked to prevent unsafe logging of potentially sensitive data
2021-03-25 15:54:27.575  INFO 1 --- [nio-8080-exec-1] o.s.web.servlet.DispatcherServlet        : Completed initialization in 2 ms
2021-03-25 15:54:27.600 TRACE 1 --- [nio-8080-exec-1] o.s.web.servlet.DispatcherServlet        : GET "/actuator/health/liveness", parameters={}, headers={masked} in DispatcherServlet 'dispatcherServlet'
2021-03-25 15:54:27.613 TRACE 1 --- [nio-8080-exec-1] s.b.a.e.w.s.WebMvcEndpointHandlerMapping : Mapped to Actuator web endpoint 'health-path'
2021-03-25 15:54:27.678 TRACE 1 --- [nio-8080-exec-1] o.s.web.method.HandlerMethod             : Arguments: [org.apache.catalina.connector.RequestFacade@1937f21c, null]
```

根据日志中的时间显示，Tomcat 启动完成于 2021-03-25 15:54:01，并且花费了 5.533 秒。
在 2021-03-25 15:54:27 时，发生了第一次存活性探测，接近 30 秒。

## 总结

通过定义 initialDelaySeconds 启动延时探测，避免在容器中的服务未完全启动起来时进行探测。

## 附录
