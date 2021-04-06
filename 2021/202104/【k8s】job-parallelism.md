# 【k8s】job-parallelism

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

parallelism 可以设置 Job 执行任务的并行数。

## 示例

### Job.yaml

```yaml
apiVersion: batch/v1
kind: Job
metadata:
  name: busybox
spec:
  completions: 4
  parallelism: 2
  template:
    spec:
      containers:
        - name: busybox
          image: busybox:1.30.0
          command: ["/bin/sh", "-c", "sleep 10;date"]
      restartPolicy: Never
```

### 查看

```
[root@master ~]# kubectl get job,pod
NAME                COMPLETIONS   DURATION   AGE
job.batch/busybox   4/4           23s        83s

NAME                READY   STATUS      RESTARTS   AGE
pod/busybox-j4mzm   0/1     Completed   0          83s
pod/busybox-th4jl   0/1     Completed   0          72s
pod/busybox-xwjbr   0/1     Completed   0          83s
pod/busybox-zf7jr   0/1     Completed   0          72s
```

可以看到并行数为 2，两个 Pod 的 AGE 都是一样的，表示它们是同时启动的。

## 总结

parallelism 可以设置 Job 执行任务的并行数。

## 附录
