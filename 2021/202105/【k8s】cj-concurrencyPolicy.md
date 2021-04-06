# 【k8s】cj-concurrencyPolicy

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

concurrencyPolicy 可以设置并发策略，合法的值包括以下三种，默认为 Allow：

1. Allow 允许并行运行
2. Forbid 禁止并行运行
3. Replace 结束已经在运行的，重新启动一个新的。

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
              command: ["/bin/sh", "-c", "sleep 150;date"]
          restartPolicy: Never
  schedule: "* * * * *"
```

### 查看

```
[root@master ~]# kubectl get cj,job,pod
NAME                    SCHEDULE    SUSPEND   ACTIVE   LAST SCHEDULE   AGE
cronjob.batch/busybox   * * * * *   False     2        30s             112s

NAME                           COMPLETIONS   DURATION   AGE
job.batch/busybox-1617723360   0/1           81s        81s
job.batch/busybox-1617723420   0/1           21s        21s

NAME                           READY   STATUS    RESTARTS   AGE
pod/busybox-1617723360-wplhx   1/1     Running   0          81s
pod/busybox-1617723420-7jts2   1/1     Running   0          21s
```

可以看到两个任务在同时运行。

## 总结

concurrencyPolicy 字段可以设置并行运行的策略。

## 附录
