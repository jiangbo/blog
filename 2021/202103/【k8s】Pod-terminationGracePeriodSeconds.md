# 【k8s】Pod-terminationGracePeriodSeconds

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

terminationGracePeriodSeconds 可以定义优雅关闭的宽限期，即在收到停止请求后，
有多少时间来进行资源释放或者做其它操作，如果到了最大时间还没有停止，会被强制结束。
默认值：30。

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
      command: ["/bin/sh", "-c", "sleep 3600"]
  terminationGracePeriodSeconds: 5
```

### 查看

```
[root@master ~]# kubectl get pod --watch
NAME         READY   STATUS    RESTARTS   AGE
busybox      1/1     Running   0          12s
spring-k8s   1/1     Running   0          13m
busybox      1/1     Terminating   0          18s
busybox      0/1     Terminating   0          49s
busybox      0/1     Terminating   0          50s
busybox      0/1     Terminating   0          56s
busybox      0/1     Terminating   0          56s
busybox      0/1     Pending       0          0s
busybox      0/1     Pending       0          0s
busybox      0/1     ContainerCreating   0          0s
busybox      1/1     Running             0          2s
busybox      1/1     Terminating         0          12s
busybox      0/1     Terminating         0          18s
busybox      0/1     Terminating         0          19s
busybox      0/1     Terminating         0          19s
```

第一次是默认 30 秒的结束时间，第二次设置成了 5 秒。
从 12s 开始结束，到 18s 强制终止。

## 总结

terminationGracePeriodSeconds，可以设置优雅关闭的期限，默认为 30 秒。

## 附录
