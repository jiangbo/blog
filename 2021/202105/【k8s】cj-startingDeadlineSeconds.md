# 【k8s】cj-startingDeadlineSeconds

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

startingDeadlineSeconds 表示开始的最后期限。它表示任务如果由于某种原因错过了调度时间，开始该任务的截止时间的秒数。

## 示例

### CronJob.yaml

```yaml
apiVersion: batch/v1beta1
kind: CronJob
metadata:
  name: busybox
spec:
  concurrencyPolicy: Forbid
  startingDeadlineSeconds: 120
  jobTemplate:
    spec:
      template:
        spec:
          containers:
            - name: busybox
              image: busybox:1.30.0
              command: ["/bin/sh", "-c", "sleep 90;date"]
          restartPolicy: Never
  schedule: "* * * * *"
```

### 查看

```
[root@master ~]# kubectl get cj,job,pod
NAME                    SCHEDULE    SUSPEND   ACTIVE   LAST SCHEDULE   AGE
cronjob.batch/busybox   * * * * *   False     1        53s             4m29s

NAME                           COMPLETIONS   DURATION   AGE
job.batch/busybox-1617725460   1/1           97s        3m44s
job.batch/busybox-1617725520   1/1           91s        2m3s
job.batch/busybox-1617725640   0/1           23s        23s

NAME                           READY   STATUS      RESTARTS   AGE
pod/busybox-1617725460-ddh5l   0/1     Completed   0          3m44s
pod/busybox-1617725520-9tj5w   0/1     Completed   0          2m3s
pod/busybox-1617725640-xj78s   1/1     Running     0          23s
```

## 总结

startingDeadlineSeconds 表示开始的最后期限。

## 附录
