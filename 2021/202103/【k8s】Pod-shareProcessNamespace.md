# 【k8s】Pod-shareProcessNamespace

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

shareProcessNamespace 表示 Pod 中的容器需要共享命名空间。

## 示例

### Pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
spec:
  shareProcessNamespace: true
  containers:
    - name: spring-k8s
      image: jiangbo920827/spring-k8s:liveness
      ports:
        - containerPort: 8080
    - name: busybox
      image: busybox:stable
      command: ["/bin/sh", "-c", "sleep 3600"]
  terminationGracePeriodSeconds: 5
```

### 查看

```
[root@master ~]# kubectl exec spring-k8s -- ps
Defaulting container name to spring-k8s.
Use 'kubectl describe pod/spring-k8s -n default' to see all of the containers in this pod.
PID   USER     TIME  COMMAND
    1 root      0:00 /pause
    8 root      0:11 java org.springframework.boot.loader.JarLauncher
   24 root      0:00 sleep 3600
  120 root      0:00 ps
```

可以看到 Java 启动命令和 busybox 启动命令都可见，并且 PID 都不为 1。

### 查看容器信息

之前学习过，默认情况，容器只监听 PID 为 1 的进程，在 shareProcessNamespace 的情况下，
即使容器的 PID 不为 1，在退出时还是会自动重启。

```
NAME         READY   STATUS    RESTARTS   AGE
spring-k8s   2/2     Running   1          12m
```

## 总结

shareProcessNamespace 可以在 Pod 中共享命名空间。

## 附录
