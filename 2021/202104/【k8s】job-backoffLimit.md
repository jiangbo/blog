# 【k8s】job-backoffLimit

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

backoffLimit 表示回退限制，可以指定重试几次后将 Job 标记为失败。

## 示例

### Job.yaml

```yaml
apiVersion: batch/v1
kind: Job
metadata:
  name: busybox
spec:
  backoffLimit: 2
  template:
    spec:
      containers:
        - name: busybox
          image: busybox:1.30.0
          command: ["/bin/sh", "-c", "sleep 10;date;exit 1"]
      restartPolicy: Never
```

### 查看

```
[root@master ~]# kubectl get job,pod
NAME                COMPLETIONS   DURATION   AGE
job.batch/busybox   0/1           4m34s      4m34s

NAME                READY   STATUS   RESTARTS   AGE
pod/busybox-dhrkt   0/1     Error    0          4m34s
pod/busybox-kcx46   0/1     Error    0          4m
pod/busybox-tlk48   0/1     Error    0          4m21s
```

可以看到重试了两次，但是还是失败了，后面就没有再次重试了。

### 查看详细信息

```
[root@master ~]# kubectl describe job busybox
Name:           busybox
Namespace:      default
Selector:       controller-uid=461bdd7d-4510-4c55-ac45-211b5621219f
Labels:         controller-uid=461bdd7d-4510-4c55-ac45-211b5621219f
                job-name=busybox
Annotations:    <none>
Parallelism:    1
Completions:    1
Start Time:     Tue, 06 Apr 2021 22:09:53 +0800
Pods Statuses:  0 Running / 0 Succeeded / 3 Failed
Pod Template:
  Labels:  controller-uid=461bdd7d-4510-4c55-ac45-211b5621219f
           job-name=busybox
  Containers:
   busybox:
    Image:      busybox:1.30.0
    Port:       <none>
    Host Port:  <none>
    Command:
      /bin/sh
      -c
      sleep 10;date;exit 1
    Environment:  <none>
    Mounts:       <none>
  Volumes:        <none>
Events:
  Type     Reason                Age    From            Message
  ----     ------                ----   ----            -------
  Normal   SuccessfulCreate      5m26s  job-controller  Created pod: busybox-dhrkt
  Normal   SuccessfulCreate      5m13s  job-controller  Created pod: busybox-tlk48
  Normal   SuccessfulCreate      4m52s  job-controller  Created pod: busybox-kcx46
  Warning  BackoffLimitExceeded  4m32s  job-controller  Job has reached the specified backoff limit
```

## 总结

backoffLimit 表示回退限制，可以指定重试几次后将 Job 标记为失败。

## 附录
