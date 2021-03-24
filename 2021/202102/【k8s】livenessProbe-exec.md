# 【k8s】livenessProbe-exec

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

livenessProbe 是一个存活性探针，可以通过多种方式定义存活性探针。
下面通过 exec 的方式定义一个存活性探针，并且让其失败。

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
        exec:
          command: ["cat", "/tmp/healthy"]
      command:
        [
          "/bin/sh",
          "-c",
          "touch /tmp/healthy; sleep 30; rm -rf /tmp/healthy; sleep 600",
        ]
```

定义了一个启动命令，首先在 /tmp 目录下新建了一个文件，睡眠 30 秒后又进行了删除。
看到的现象是容器首先启动成功了，在等待 30 秒后，开始发现存活性探针失败，连续检测三次失败后，重启容器。

### 查看

```
Events:
  Type     Reason     Age               From               Message
  ----     ------     ----              ----               -------
  Normal   Scheduled  64s               default-scheduler  Successfully assigned default/busybox to node2
  Normal   Pulled     64s               kubelet            Container image "busybox:stable" already present on machine
  Normal   Created    63s               kubelet            Created container busybox
  Normal   Started    63s               kubelet            Started container busybox
  Warning  Unhealthy  6s (x3 over 26s)  kubelet            Liveness probe failed: cat: can't open '/tmp/healthy': No such file or directory
  Normal   Killing    6s                kubelet            Container busybox failed liveness probe, will be restarted
```

## 总结

通过执行命令的方式，来实现了一个存活性探针，在检测到探针失败时，容器进行了重启。

## 附录
