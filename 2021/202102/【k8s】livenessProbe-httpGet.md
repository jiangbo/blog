# 【k8s】livenessProbe-httpGet

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

livenessProbe 是一个存活性探针，可以通过多种方式定义存活性探针。
下面通过 httpGet 的方式定义一个存活性探针，Spring Boot 2.3 之后内置了存活性探针。

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
      image: jiangbo920827/spring-k8s:product
      livenessProbe:
        httpGet:
          path: /actuator/health/liveness
          port: 8080
      ports:
        - containerPort: 8080
```

### 查看

```
Events:
  Type     Reason     Age    From               Message
  ----     ------     ----   ----               -------
  Normal   Scheduled  2m27s  default-scheduler  Successfully assigned default/spring-k8s to node1
  Normal   Pulled     2m27s  kubelet            Container image "jiangbo920827/spring-k8s:product" already present on machine
  Normal   Created    2m27s  kubelet            Created container spring-k8s
  Normal   Started    2m27s  kubelet            Started container spring-k8s
  Warning  Unhealthy  2m23s  kubelet            Liveness probe failed: Get "http://10.244.1.160:8080/actuator/health/liveness": dial tcp 10.244.1.160:8080: connect: connection refused
```

通过查看 Pod 的事件信息，我们看到服务已经正常启动了，并且没有重启。
但是有一次存活性探针的检测失败信息，这是因为第一次检测时，服务还没有启动，所以失败了。
后面将配置延时检测来避免这样的问题。

## 总结

通过定义 httpGet 的方式，来实现了一个存活性探针。

## 附录
