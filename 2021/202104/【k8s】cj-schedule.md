# 【k8s】cj-schedule

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

schedule 字段可以设置定时任务执行的时间。

```
# ┌───────────── 分钟 (0 - 59)
# │ ┌───────────── 小时 (0 - 23)
# │ │ ┌───────────── 月的某天 (1 - 31)
# │ │ │ ┌───────────── 月份 (1 - 12)
# │ │ │ │ ┌───────────── 周的某天 (0 - 6) （周日到周一；在某些系统上，7 也是星期日）
# │ │ │ │ │                                   
# │ │ │ │ │
# │ │ │ │ │
# * * * * *
```

## 示例

### CronJob.yaml

```yaml
apiVersion: batch/v1beta1
kind: CronJob
metadata:
  name: busybox
spec:
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

上面的 cron 表示每分钟执行一次。

### 查看

```
[root@master ~]# kubectl get cj,job,pod
NAME                    SCHEDULE    SUSPEND   ACTIVE   LAST SCHEDULE   AGE
cronjob.batch/busybox   * * * * *   False     1        5s              51s

NAME                           COMPLETIONS   DURATION   AGE
job.batch/busybox-1617722460   0/1           0s         0s

NAME                           READY   STATUS              RESTARTS   AGE
pod/busybox-1617722460-sjvfr   0/1     ContainerCreating   0          0s
```

## 总结

schedule 字段可以设置定时任务执行的时间。

## 附录
