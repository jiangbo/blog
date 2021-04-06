# 【k8s】cj-successfulJobsHistoryLimit

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

successfulJobsHistoryLimit 设置成功的任务历史记录数，默认值为 3。

## 示例

### CronJob.yaml

```yaml
apiVersion: batch/v1beta1
kind: CronJob
metadata:
  name: busybox
spec:
  successfulJobsHistoryLimit: 1
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
cronjob.batch/busybox   * * * * *   False     0        32s             3m37s

NAME                           COMPLETIONS   DURATION   AGE
job.batch/busybox-1617724920   1/1           12s        25s

NAME                           READY   STATUS      RESTARTS   AGE
pod/busybox-1617724920-zrsbv   0/1     Completed   0          25s
```

## 总结

successfulJobsHistoryLimit 设置成功的任务历史记录数。

## 附录
