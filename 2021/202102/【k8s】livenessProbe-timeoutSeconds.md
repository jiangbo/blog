# 【k8s】livenessProbe-timeoutSeconds

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

timeoutSeconds 执行探测的超时的秒数，默认值 1，最小值 1。

## 示例

### Pod.yaml

定义一个探针需要两秒的命令。

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
        exec:
          command: ["sh", "-c", "sleep 2;date >> date.log"]
        periodSeconds: 5
      command: ["/bin/sh", "-c", "sleep 3;tail -f date.log"]
```

### 查看失败事件

```
Events:
  Type     Reason     Age                From               Message
  ----     ------     ----               ----               -------
  Normal   Scheduled  23s                default-scheduler  Successfully assigned default/busybox to node1
  Normal   Pulled     19s (x2 over 23s)  kubelet            Container image "busybox:stable" already present on machine
  Normal   Created    19s (x2 over 23s)  kubelet            Created container busybox
  Normal   Started    19s (x2 over 23s)  kubelet            Started container busybox
  Warning  Unhealthy  16s                kubelet            Liveness probe failed:
  Warning  BackOff    12s (x2 over 15s)  kubelet            Back-off restarting failed container
```

## 总结

通过定义 timeoutSeconds 探针需要的时间，如果探针需要时间较长，可以增加超时时间。

## 附录
