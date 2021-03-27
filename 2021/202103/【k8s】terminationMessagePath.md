# 【k8s】terminationMessagePath

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

terminationMessagePath 表示容器的异常终止消息的路径，默认在 /dev/termination-log 下。
当容器退出时，可以通过容器的状态看到退出信息。

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
      command: ["/bin/sh"]
      args: ["-c", "sleep 10 && echo Sleep expired > /dev/termination-log"]
```

可以看到，在睡眠 10 秒后，将 `Sleep expired` 输出到了异常退出的文件中。

### 查看退出信息

```
Containers:
  busybox:
    Container ID:  docker://c1b7011b548235b140f42e5b6558a0912cefe57199a0fd1589697ac0aa434d85
    Image:         busybox:stable
    Image ID:      docker-pullable://busybox@sha256:ce2360d5189a033012fbad1635e037be86f23b65cfd676b436d0931af390a2ac
    Port:          <none>
    Host Port:     <none>
    Command:
      /bin/sh
    Args:
      -c
      sleep 10 && echo Sleep expired > /dev/termination-log
    State:       Waiting
      Reason:    CrashLoopBackOff
    Last State:  Terminated
      Reason:    Completed
      Message:   Sleep expired

      Exit Code:    0
      Started:      Sat, 27 Mar 2021 16:32:28 +0800
      Finished:     Sat, 27 Mar 2021 16:32:38 +0800
    Ready:          False
    Restart Count:  2
    Environment:    <none>
    Mounts:
      /var/run/secrets/kubernetes.io/serviceaccount from default-token-slbq5 (ro)
```

### 只查看异常信息

```
[root@master ~]# kubectl get pod busybox -o go-template="{{range .status.containerStatuses}}{{.lastState.terminated.message}}{{end}}"
Sleep expired
```

### 自定义异常信息文件

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: busybox
spec:
  containers:
    - name: busybox
      image: busybox:stable
      command: ["/bin/sh"]
      terminationMessagePath: /root/test.log
      args: ["-c", "sleep 10 && echo Sleep expired > /root/test.log"]
```

## 总结

查看异常退出信息，以及自定义退出的信息和文件位置。

## 附录
