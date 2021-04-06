# 【k8s】cj-suspend

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

suspend 可以将定时任务挂起，已经执行的任务不会受到影响。

## 示例

### CronJob.yaml

```yaml
apiVersion: batch/v1beta1
kind: CronJob
metadata:
  name: busybox
spec:
  suspend: true
  jobTemplate:
    spec:
      template:
        spec:
          containers:
            - name: busybox
              image: busybox:1.30.0
              command: ["/bin/sh", "-c", "sleep 10;date"]
          restartPolicy: Never
  schedule: "* * * * *"
```

### 查看

```
[root@master ~]# kubectl get cj,job,pod
NAME                    SCHEDULE    SUSPEND   ACTIVE   LAST SCHEDULE   AGE
cronjob.batch/busybox   * * * * *   False     1        24s             6m29s

NAME                           COMPLETIONS   DURATION   AGE
job.batch/busybox-1617725040   1/1           11s        77s
job.batch/busybox-1617725100   1/1           12s        17s

NAME                           READY   STATUS      RESTARTS   AGE
pod/busybox-1617725040-ccqwf   0/1     Completed   0          77s
pod/busybox-1617725100-p4q2w   0/1     Completed   0          17s
[root@master ~]# kubectl get cj,job,pod
NAME                    SCHEDULE    SUSPEND   ACTIVE   LAST SCHEDULE   AGE
cronjob.batch/busybox   * * * * *   True      0        32s             6m37s

NAME                           COMPLETIONS   DURATION   AGE
job.batch/busybox-1617725040   1/1           11s        85s
job.batch/busybox-1617725100   1/1           12s        25s

NAME                           READY   STATUS      RESTARTS   AGE
pod/busybox-1617725040-ccqwf   0/1     Completed   0          85s
pod/busybox-1617725100-p4q2w   0/1     Completed   0          25s
```

SUSPEND 字段变成了 True。

## 总结

suspend 可以将定时任务挂起，已经执行的任务不会受到影响。

## 附录
