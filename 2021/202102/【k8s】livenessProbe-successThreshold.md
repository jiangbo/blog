# 【k8s】livenessProbe-successThreshold

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

successThreshold 表示探针的成功的阈值，在达到该次数时，表示成功。
默认值为 1，表示只要成功一次，就算成功了。

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
          port: http
      ports:
        - name: http
          containerPort: 8080
```

### 将存活探针修改为失败

因为探针的默认周期是 10 秒，并且最大失败次数默认三次，所以在将其修改为失败后，
看到两次失败的时候，将可以将其修改回来，避免容器重启。

修改的状态的 URL 为：10.244.1.181:8080/liveness?name=BROKEN，IP根据实际情况修改。

```
2021-03-27 05:05:43 - GET "/actuator/health/liveness", parameters={}
2021-03-27 05:05:43 - Mapped to Actuator web endpoint 'health-path'
2021-03-27 05:05:43 - Using 'application/vnd.spring-boot.actuator.v3+json', given [*/*] and supported [application/vnd.spring-boot.actuator.v3+json, application/vnd.spring-boot.actuator.v2+json, application/json]
2021-03-27 05:05:43 - Writing [org.springframework.boot.actuate.health.CompositeHealth@741c344a]
2021-03-27 05:05:43 - Completed 200 OK
2021-03-27 05:05:49 - GET "/liveness?name=BROKEN", parameters={masked}
2021-03-27 05:05:49 - Mapped to jiangbo.spring.docker.SpringDemoApplication#liveness(String)
2021-03-27 05:05:49 - Using 'text/plain', given [*/*] and supported [text/plain, */*, text/plain, */*, application/json, application/*+json, application/json, application/*+json]
2021-03-27 05:05:49 - Writing ["BROKEN"]
2021-03-27 05:05:49 - Completed 200 OK
```

### 查看失败的探针请求

可以看到两次失败的存活性探针的请求，返回的状态是 503，后面又修改回了可用的状态。

```
2021-03-27 05:05:53 - GET "/actuator/health/liveness", parameters={}
2021-03-27 05:05:53 - Mapped to Actuator web endpoint 'health-path'
2021-03-27 05:05:53 - Using 'application/vnd.spring-boot.actuator.v3+json', given [*/*] and supported [application/vnd.spring-boot.actuator.v3+json, application/vnd.spring-boot.actuator.v2+json, application/json]
2021-03-27 05:05:53 - Writing [org.springframework.boot.actuate.health.CompositeHealth@2622456]
2021-03-27 05:05:53 - Completed 503 SERVICE_UNAVAILABLE
2021-03-27 05:06:03 - GET "/actuator/health/liveness", parameters={}
2021-03-27 05:06:03 - Mapped to Actuator web endpoint 'health-path'
2021-03-27 05:06:03 - Using 'application/vnd.spring-boot.actuator.v3+json', given [*/*] and supported [application/vnd.spring-boot.actuator.v3+json, application/vnd.spring-boot.actuator.v2+json, application/json]
2021-03-27 05:06:03 - Writing [org.springframework.boot.actuate.health.CompositeHealth@23288d0e]
2021-03-27 05:06:03 - Completed 503 SERVICE_UNAVAILABLE
2021-03-27 05:06:05 - GET "/liveness?name=CORRECT", parameters={masked}
2021-03-27 05:06:05 - Mapped to jiangbo.spring.docker.SpringDemoApplication#liveness(String)
2021-03-27 05:06:05 - Using 'text/plain', given [*/*] and supported [text/plain, */*, text/plain, */*, application/json, application/*+json, application/json, application/*+json]
2021-03-27 05:06:05 - Writing ["CORRECT"]
2021-03-27 05:06:05 - Completed 200 OK
```

### 查看探针失败的事件

```
Events:
  Type     Reason     Age                From               Message
  ----     ------     ----               ----               -------
  Normal   Scheduled  2m40s              default-scheduler  Successfully assigned default/spring-k8s to node1
  Normal   Pulled     2m39s              kubelet            Container image "jiangbo920827/spring-k8s:liveness" already present on machine
  Normal   Created    2m39s              kubelet            Created container spring-k8s
  Normal   Started    2m39s              kubelet            Started container spring-k8s
  Warning  Unhealthy  30s (x2 over 40s)  kubelet            Liveness probe failed: HTTP probe failed with statuscode: 503
```

出现了两次 503 状态，后面变为可用后，容器并没有进行重启。
只要探针成功了一次，就会恢复正常。

## 总结

通过定义 successThreshold 来设定成功的阈值，只要成功了一次，就表示成功了。

## 附录
