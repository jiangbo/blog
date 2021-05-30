# 【k8s】automountServiceAccountToken

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-RC1

## 目标

automountServiceAccountToken 表示是否将服务账号默认挂载到 Pod 中，默认是 true。

### pod.yaml

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

### 查看

```
[root@master ~]# kubectl describe pod spring-k8s
Name:         spring-k8s
Namespace:    default
Priority:     0
...
Volumes:
  default-token-slbq5:
    Type:        Secret (a volume populated by a Secret)
    SecretName:  default-token-slbq5
    Optional:    false
QoS Class:       BestEffort
...

[root@master ~]# kubectl exec spring-k8s -- ls -l /var/run/secrets/kubernetes.io/serviceaccount
total 0
lrwxrwxrwx    1 root     root            13 May 30 15:24 ca.crt -> ..data/ca.crt
lrwxrwxrwx    1 root     root            16 May 30 15:24 namespace -> ..data/namespace
lrwxrwxrwx    1 root     root            12 May 30 15:24 token -> ..data/token

```

### automountServiceAccountToken

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
spec:
  automountServiceAccountToken: false
  containers:
    - name: spring-k8s
      image: jiangbo920827/spring-k8s:liveness
      ports:
        - containerPort: 8080
```

服务账号并不会自动挂载到 Pod 中。

## 总结

将默认挂载到 Pod 中的服务账号移除。

## 附录
