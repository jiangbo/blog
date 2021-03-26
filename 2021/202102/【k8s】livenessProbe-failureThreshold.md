# 【k8s】livenessProbe-failureThreshold

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

failureThreshold 表示探针的最大失败次数，如果达到了最大的失败次数，
在存活性探针的情况，容器将重新启动。

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

### 查看存活探针日志

```
2021-03-26 16:06:42 - GET "/actuator/health/liveness", parameters={}
2021-03-26 16:06:42 - Mapped to Actuator web endpoint 'health-path'
2021-03-26 16:06:42 - Using 'application/vnd.spring-boot.actuator.v3+json', given [*/*] and supported [application/vnd.spring-boot.actuator.v3+json, application/vnd.spring-boot.actuator.v2+json, application/json]
2021-03-26 16:06:42 - Writing [org.springframework.boot.actuate.health.CompositeHealth@731b43a6]
2021-03-26 16:06:42 - Completed 200 OK
```

### 查看存活探针

```
[root@master ~]# curl 10.244.1.179:8080/actuator/health/liveness|jq
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100    15    0    15    0     0   1590      0 --:--:-- --:--:-- --:--:--  1666
{
  "status": "UP"
}
```

### 停止探针

```
[root@master ~]# curl 10.244.1.179:8080/liveness?name=BROKEN
BROKEN[root@master ~]#
```

### 查看重启事件

```
Events:
  Type     Reason     Age                 From               Message
  ----     ------     ----                ----               -------
  Normal   Scheduled  118s                default-scheduler  Successfully assigned default/spring-k8s to node1
  Normal   Pulled     17s (x2 over 117s)  kubelet            Container image "jiangbo920827/spring-k8s:liveness" already present on machine
  Normal   Created    17s (x2 over 117s)  kubelet            Created container spring-k8s
  Normal   Started    17s (x2 over 117s)  kubelet            Started container spring-k8s
  Warning  Unhealthy  17s (x3 over 37s)   kubelet            Liveness probe failed: HTTP probe failed with statuscode: 503
  Normal   Killing    17s                 kubelet            Container spring-k8s failed liveness probe, will be restarted
```

可以看到重启了，并且有三次存活探针的失败事件。

## 总结

通过定义 failureThreshold 来设定失败的最大次数，默认为 3 次。

## 附录
