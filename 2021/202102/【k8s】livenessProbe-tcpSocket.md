# 【k8s】livenessProbe-tcpSocket

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

livenessProbe 是一个存活性探针，可以通过多种方式定义存活性探针。
下面通过 tcpSocket 的方式定义一个存活性探针。

## 示例

### Pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: busybox
spec:
  containers:
    - name: busybox
      image: busybox:stable
      livenessProbe:
        tcpSocket:
          port: 8080
      command: ["/bin/sh", "-c", "nc -p 8080 -kle echo pong1"]
```

## 总结

通过定义 tcpSocket 的方式，来实现了一个存活性探针。

## 附录
