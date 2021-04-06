# 【k8s】job-completions

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

completions 可以设置 Job 成功执行任务的数量。

## 示例

### Job.yaml

```yaml
apiVersion: batch/v1
kind: Job
metadata:
  name: busybox
spec:
  completions: 4
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
job.batch/busybox   4/4           50s        57s

NAME                READY   STATUS      RESTARTS   AGE
pod/busybox-8d84t   0/1     Completed   0          33s
pod/busybox-dqfgs   0/1     Completed   0          22s
pod/busybox-gwgnn   0/1     Completed   0          45s
pod/busybox-x49cc   0/1     Completed   0          57s
```

## 总结

completions 可以设置 Job 成功执行任务的数量。

## 附录
