# 【k8s】Container-workingDir

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

workingDir 可以设置进入容器的目录，没有设置的情况下，默认进入根目录。

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
      workingDir: /root/
      command: ["/bin/sh", "-c", "sleep 3600"]
```

### 查看 Pod

```
NAME      READY   STATUS    RESTARTS   AGE
busybox   1/1     Running   0          6s
```

### 进入容器

```
[root@master ~]# kubectl exec -it busybox -- sh
~ # ls
~ # pwd
/root
```

## 总结

通过 workingDir 可以设置工作目录。

## 附录
