# 【k8s】Pod-enableServiceLinks

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

enableServiceLinks 表示是否将 Service 的相关信息注入到 Pod 的环境变量中，默认是 true。

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
      ports:
        - containerPort: 8080
```

### 查看环境变量

```
[root@master ~]# kubectl exec spring-k8s -- printenv|grep PG
SVC_PG_SERVICE_HOST=10.110.153.250
SVC_PG_PORT_5432_TCP=tcp://10.110.153.250:5432
SVC_PG_PORT_5432_TCP_PROTO=tcp
SVC_PG_PORT_5432_TCP_ADDR=10.110.153.250
SVC_PG_PORT=tcp://10.110.153.250:5432
SVC_PG_SERVICE_PORT=5432
SVC_PG_PORT_5432_TCP_PORT=5432
```

### 删除环境变量 yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
spec:
  enableServiceLinks: false
  containers:
    - name: spring-k8s
      image: jiangbo920827/spring-k8s:liveness
      ports:
        - containerPort: 8080
```

再次通过命令 `kubectl exec spring-k8s -- printenv|grep PG` 查看，已经没有相关的环境变量了。

## 总结

enableServiceLinks 可以设置 Service 环境变量是否注入 Pod 中，但是 k8s 自己默认的环境变量除外。

## 附录
