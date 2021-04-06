# 【k8s】cj-failedJobsHistoryLimit

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

failedJobsHistoryLimit 设置失败的任务历史记录数，默认值为 1，表示只保留一次失败的记录。

## 示例

### CronJob.yaml

```yaml
apiVersion: batch/v1beta1
kind: CronJob
metadata:
  name: busybox
spec:
  failedJobsHistoryLimit: 2
  jobTemplate:
    spec:
      template:
        spec:
          containers:
            - name: busybox
              image: busybox:1.30.0
              command: ["/bin/sh", "-c", "sleep 10;date;exit 1"]
          restartPolicy: Never
  schedule: "* * * * *"
```

### 查看

```
[root@master ~]# kubectl get cj,job,pod
NAME                    SCHEDULE    SUSPEND   ACTIVE   LAST SCHEDULE   AGE
cronjob.batch/busybox   * * * * *   False     0        55s             3m18s

NAME                           COMPLETIONS   DURATION   AGE
job.batch/busybox-1617724260   0/1           112s       112s
job.batch/busybox-1617724320   0/1           51s        51s

NAME                           READY   STATUS   RESTARTS   AGE
pod/busybox-1617724260-j5vh6   0/1     Error    0          112s
pod/busybox-1617724320-z2l7k   0/1     Error    0          51s
```

可以看到失败的 Job 保存了两个。

## 总结

failedJobsHistoryLimit 设置失败的任务历史记录数。

## 附录
