# 【Kubernetes】激活生产配置-args

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M2

## 目标

通过前面的[【Kubernetes】Spring Boot 开发与生产配置分离][1]生成了一个开发与生产配置分离的 Spring Boot 项目，通过 args 的方式来激活生产配置。

## 激活生产配置

### Pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
spec:
  containers:
    - name: spring-k8s
      image: jiangbo920827/spring-k8s:product
      args:
        - --spring.profiles.active=product
      resources:
        limits:
          memory: 128Mi
          cpu: 500m
      ports:
        - containerPort: 8080
```

### 查看详情

```
[root@master kubernetes]# kubectl describe pod spring-k8s
Name:         spring-k8s
Namespace:    default
Priority:     0
Node:         node1/192.168.56.102
Start Time:   Mon, 22 Mar 2021 00:03:39 +0800
Labels:       <none>
Annotations:  <none>
Status:       Running
IP:           10.244.1.151
IPs:
  IP:  10.244.1.151
Containers:
  spring-k8s:
    Container ID:  docker://3a81e225ea0a8548a7fea3acf9613ac87a42081b325cc655b9937e55484e93fb
    Image:         jiangbo920827/spring-k8s:product
    Image ID:      docker-pullable://jiangbo920827/spring-k8s@sha256:da4235afda51eb06b8f7cf0b1fe3880d5b3d85843c13eade672a75112aed9428
    Port:          8080/TCP
    Host Port:     0/TCP
    Args:
      --spring.profiles.active=product
    State:          Running
      Started:      Mon, 22 Mar 2021 00:03:40 +0800
    Ready:          True
    Restart Count:  0
    Limits:
      cpu:     500m
      memory:  128Mi
    Requests:
      cpu:        500m
      memory:     128Mi
    Environment:  <none>
    Mounts:
      /var/run/secrets/kubernetes.io/serviceaccount from default-token-slbq5 (ro)
Conditions:
  Type              Status
  Initialized       True
  Ready             True
  ContainersReady   True
  PodScheduled      True
Volumes:
  default-token-slbq5:
    Type:        Secret (a volume populated by a Secret)
    SecretName:  default-token-slbq5
    Optional:    false
QoS Class:       Guaranteed
Node-Selectors:  <none>
Tolerations:     node.kubernetes.io/not-ready:NoExecute op=Exists for 300s
                 node.kubernetes.io/unreachable:NoExecute op=Exists for 300s
Events:
  Type    Reason     Age   From               Message
  ----    ------     ----  ----               -------
  Normal  Scheduled  73s   default-scheduler  Successfully assigned default/spring-k8s to node1
  Normal  Pulled     72s   kubelet            Container image "jiangbo920827/spring-k8s:product" already present on machine
  Normal  Created    72s   kubelet            Created container spring-k8s
  Normal  Started    72s   kubelet            Started container spring-k8s
```

### 查看日志

```
[root@master kubernetes]# kubectl logs -f spring-k8s

  .   ____          _            __ _ _
 /\\ / ___'_ __ _ _(_)_ __  __ _ \ \ \ \
( ( )\___ | '_ | '_| | '_ \/ _` | \ \ \ \
 \\/  ___)| |_)| | | | | || (_| |  ) ) ) )
  '  |____| .__|_| |_|_| |_\__, | / / / /
 =========|_|==============|___/=/_/_/_/
 :: Spring Boot ::             (v2.5.0-M2)

2021-03-21 15:50:05.396  INFO 1 --- [           main] j.spring.docker.SpringDemoApplication    : Starting SpringDemoApplication vsvc using Java 1.8.0_212 on spring-k8s with PID 1 (/BOOT-INF/classes started by root in /)
2021-03-21 15:50:05.398  INFO 1 --- [           main] j.spring.docker.SpringDemoApplication    : The following profiles are active: product
2021-03-21 15:50:11.802  INFO 1 --- [           main] o.s.b.w.embedded.tomcat.TomcatWebServer  : Tomcat initialized with port(s): 8080 (http)
2021-03-21 15:50:11.892  INFO 1 --- [           main] o.apache.catalina.core.StandardService   : Starting service [Tomcat]
2021-03-21 15:50:11.892  INFO 1 --- [           main] org.apache.catalina.core.StandardEngine  : Starting Servlet engine: [Apache Tomcat/9.0.43]
2021-03-21 15:50:12.107  INFO 1 --- [           main] o.a.c.c.C.[Tomcat].[localhost].[/]       : Initializing Spring embedded WebApplicationContext
2021-03-21 15:50:12.108  INFO 1 --- [           main] w.s.c.ServletWebServerApplicationContext : Root WebApplicationContext: initialization completed in 6490 ms
2021-03-21 15:50:15.617  INFO 1 --- [           main] o.s.s.concurrent.ThreadPoolTaskExecutor  : Initializing ExecutorService 'applicationTaskExecutor'
2021-03-21 15:50:17.408  INFO 1 --- [           main] o.s.b.a.e.web.EndpointLinksResolver      : Exposing 14 endpoint(s) beneath base path '/actuator'
2021-03-21 15:50:17.799  INFO 1 --- [           main] o.s.b.w.embedded.tomcat.TomcatWebServer  : Tomcat started on port(s): 8080 (http) with context path ''
2021-03-21 15:50:17.822  INFO 1 --- [           main] j.spring.docker.SpringDemoApplication    : Started SpringDemoApplication in 14.418 seconds (JVM running for 15.799)
```

### 访问测试

```
[root@master kubernetes]# curl 10.244.1.151:8080/users|jq
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100    29    0    29    0     0     23      0 --:--:--  0:00:01 --:--:--    23
[
  {
    "name": "jiangbo",
    "age": 44
  }
]
```

## 总结

新建了一个 Spring Boot 项目，使用 args 来激活生产配置信息。

[1]: https://www.cnblogs.com/jiangbo44/p/14564477.html

## 附录
