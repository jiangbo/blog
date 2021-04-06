# 【k8s】CronJob

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

CronJob 可以设置一个定时的 Job。

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

### 查看

```
[root@master ~]# kubectl get cj,job,pod
NAME                    SCHEDULE    SUSPEND   ACTIVE   LAST SCHEDULE   AGE
cronjob.batch/busybox   * * * * *   False     0        54s             3m6s

NAME                           COMPLETIONS   DURATION   AGE
job.batch/busybox-1617721680   1/1           11s        2m53s
job.batch/busybox-1617721740   1/1           12s        113s
job.batch/busybox-1617721800   1/1           12s        52s

NAME                           READY   STATUS      RESTARTS   AGE
pod/busybox-1617721680-qg8zw   0/1     Completed   0          2m53s
pod/busybox-1617721740-6g6dz   0/1     Completed   0          113s
pod/busybox-1617721800-6zdn2   0/1     Completed   0          52s
```

### 查看详情

```
[root@master ~]# kubectl describe cronjobs.batch busybox
Name:                          busybox
Namespace:                     default
Labels:                        <none>
Annotations:                   <none>
Schedule:                      * * * * *
Concurrency Policy:            Allow
Suspend:                       False
Successful Job History Limit:  3
Failed Job History Limit:      1
Starting Deadline Seconds:     <unset>
Selector:                      <unset>
Parallelism:                   <unset>
Completions:                   <unset>
Pod Template:
  Labels:  <none>
  Containers:
   busybox:
    Image:      busybox:1.30.0
    Port:       <none>
    Host Port:  <none>
    Command:
      /bin/sh
      -c
      sleep 10;date
    Environment:     <none>
    Mounts:          <none>
  Volumes:           <none>
Last Schedule Time:  Tue, 06 Apr 2021 23:12:00 +0800
Active Jobs:         busybox-1617721920
Events:
  Type    Reason            Age    From                Message
  ----    ------            ----   ----                -------
  Normal  SuccessfulCreate  4m13s  cronjob-controller  Created job busybox-1617721680
  Normal  SawCompletedJob   3m53s  cronjob-controller  Saw completed job: busybox-1617721680, status: Complete
  Normal  SuccessfulCreate  3m13s  cronjob-controller  Created job busybox-1617721740
  Normal  SawCompletedJob   2m53s  cronjob-controller  Saw completed job: busybox-1617721740, status: Complete
  Normal  SuccessfulCreate  2m12s  cronjob-controller  Created job busybox-1617721800
  Normal  SawCompletedJob   112s   cronjob-controller  Saw completed job: busybox-1617721800, status: Complete
  Normal  SuccessfulCreate  72s    cronjob-controller  Created job busybox-1617721860
  Normal  SawCompletedJob   52s    cronjob-controller  Saw completed job: busybox-1617721860, status: Complete
  Normal  SuccessfulDelete  52s    cronjob-controller  Deleted job busybox-1617721680
  Normal  SuccessfulCreate  12s    cronjob-controller  Created job busybox-1617721920
```

## 总结

CronJob 可以设置一个定时的 Job。

## 附录
