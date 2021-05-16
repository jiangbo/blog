# 【k8s】Volume-emptyDir

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-RC1

## 目标

emptyDir 可以在同一个 Pod 中，不同容器之间共享数据。

## 示例

### Pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
spec:
  terminationGracePeriodSeconds: 5
  containers:
    - name: busybox
      image: busybox:stable
      command:
        ["/bin/sh", "-c", "echo 'busybox' > /opt/volume/empty;sleep 3600"]
      volumeMounts:
        - mountPath: /opt/volume
          name: v1
    - name: spring-k8s
      image: jiangbo920827/spring-k8s:liveness
      volumeMounts:
        - mountPath: /opt/volume
          name: v1
      ports:
        - containerPort: 8080
  volumes:
    - name: v1
      emptyDir: {}
```

### 查看

```
[root@master ~]# kubectl exec spring-k8s -c spring-k8s -- sh -c 'cat /opt/volume/empty'
busybox
```

### 指定存储介质

emptyDir 可以使用内存存储，但是节点重启后数据会丢失，不过速度非常快。

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
spec:
  terminationGracePeriodSeconds: 5
  containers:
    - name: spring-k8s
      image: jiangbo920827/spring-k8s:liveness
      volumeMounts:
        - mountPath: /opt/volume
          name: v1
      ports:
        - containerPort: 8080
  volumes:
    - name: v1
      emptyDir:
        medium: Memory
```

## 总结

emptyDir 可以在同一个 Pod 中不同容器之间共享数据。

## 附录
