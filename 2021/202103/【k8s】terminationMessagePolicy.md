# 【k8s】terminationMessagePolicy

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

默认情况容器退出时，退出信息会从文件中读取。可以通过 terminationMessagePolicy 来修改。
将 terminationMessagePolicy 修改为：FallbackToLogsOnError，从日志中来读取。

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
      terminationMessagePolicy: FallbackToLogsOnError
      ports:
        - containerPort: 8080
```

### 查看退出信息

```
Containers:
  spring-k8s:
    Container ID:  docker://b9d103c60261f39aa934391311017d268a32218570f3a77c6f7a83689130ac4c
    Image:         jiangbo920827/spring-k8s:liveness
    Image ID:      docker://sha256:27e1956a7558e66cc463d09c86bcda059fd6534d520a9ab68fb8567048f786f2
    Port:          8080/TCP
    Host Port:     0/TCP
    State:         Running
      Started:     Sat, 27 Mar 2021 16:50:11 +0800
    Last State:    Terminated
      Reason:      Error
      Message:
  .   ____          _            __ _ _
 /\\ / ___'_ __ _ _(_)_ __  __ _ \ \ \ \
( ( )\___ | '_ | '_| | '_ \/ _` | \ \ \ \
 \\/  ___)| |_)| | | | | || (_| |  ) ) ) )
  '  |____| .__|_| |_|_| |_\__, | / / / /
 =========|_|==============|___/=/_/_/_/
 :: Spring Boot ::             (v2.5.0-M3)

2021-03-27 08:49:55 - Starting SpringDemoApplication vliveness using Java 1.8.0_212 on spring-k8s with PID 1 (/BOOT-INF/classes started by root in /)
2021-03-27 08:49:55 - No active profile set, falling back to default profiles: default
2021-03-27 08:49:57 - Tomcat initialized with port(s): 8080 (http)
2021-03-27 08:49:57 - Starting service [Tomcat]
2021-03-27 08:49:57 - Starting Servlet engine: [Apache Tomcat/9.0.44]
2021-03-27 08:49:57 - Initializing Spring embedded WebApplicationContext
2021-03-27 08:49:57 - Root WebApplicationContext: initialization completed in 1899 ms
2021-03-27 08:49:58 - Mapping filters: filterRegistrationBean urls=[/*] order=-2147483647, characterEncodingFilter urls=[/*] order=-2147483648, formContentFilter urls=[/*] order=-9900, requestContextFilter urls=[/*] order=-105
2021-03-27 08:49:58 - Mapping servlets: dispatcherServlet urls=[/]
2021-03-27 08:49:58 - Initializing ExecutorService 'applicationTaskExecutor'
2021-03-27 08:49:58 - ControllerAdvice beans: 0 @ModelAttribute, 0 @InitBinder, 1 RequestBodyAdvice, 1 ResponseBodyAdvice
2021-03-27 08:49:58 - 5 mappings in 'requestMappingHandlerMapping'
2021-03-27 08:49:58 - Patterns [/webjars/**, /**] in 'resourceHandlerMapping'
2021-03-27 08:49:58 - ControllerAdvice beans: 0 @ExceptionHandler, 1 ResponseBodyAdvice
2021-03-27 08:49:59 - Exposing 14 endpoint(s) beneath base path '/actuator'
2021-03-27 08:49:59 - Tomcat started on port(s): 8080 (http) with context path ''
2021-03-27 08:49:59 - Started SpringDemoApplication in 4.185 seconds (JVM running for 4.717)

      Exit Code:    143
      Started:      Sat, 27 Mar 2021 16:49:54 +0800
      Finished:     Sat, 27 Mar 2021 16:50:10 +0800
    Ready:          True
    Restart Count:  1
    Environment:    <none>
    Mounts:
      /var/run/secrets/kubernetes.io/serviceaccount from default-token-slbq5 (ro)
Conditions:
  Type              Status
  Initialized       True
  Ready             True
  ContainersReady   True
  PodScheduled      True
```

## 总结

通过修改 terminationMessagePolicy，退出信息直接从控制台日志里进行读取，打印出了 Spring Boot 的日志信息。

## 附录
