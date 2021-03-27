# 【k8s】Container-stdin

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

容器的 stdin 字段表示是否需要给容器分配一个标准输入。

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
      stdin: true
      command: ["/bin/sh"]
```

### 查看 Pod

```
NAME      READY   STATUS    RESTARTS   AGE
busybox   1/1     Running   0          4m9s
```

### 进入容器

```
[root@master kubernetes]# kubectl attach --stdin busybox
Defaulting container name to busybox.
Use 'kubectl describe pod/busybox -n default' to see all of the containers in this pod.
If you don't see a command prompt, try pressing enter.
ls
bin
dev
etc
home
proc
root
sys
tmp
usr
var
```

## 总结

容器的 stdin 字段可以给容器分配一个标准输入。

## 附录
