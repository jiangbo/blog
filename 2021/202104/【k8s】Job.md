# 【k8s】Job

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

Job 可以启动一个任务。和 deploy 不一样的是，它一般可以执行完成。

## 示例

### Job.yaml

```yaml
apiVersion: batch/v1
kind: Job
metadata:
  name: busybox
spec:
  template:
    spec:
      containers:
        - name: busybox
          image: busybox:1.30.0
          command: ["/bin/sh", "-c", "sleep 10;date"]
      restartPolicy: OnFailure
```

因为 Job 会执行完成，所以重启的策略不能是默认的 Always，将其修改为 OnFailure。

### 查看

```
[root@master ~]# kubectl get job,pod
NAME                COMPLETIONS   DURATION   AGE
job.batch/busybox   1/1           12s        103s

NAME                READY   STATUS      RESTARTS   AGE
pod/busybox-zmnqq   0/1     Completed   0          103s
```

Pod 的状态是 Completed，表示已成功结束。

### 查看详细信息

```
[root@master ~]# kubectl describe jobs.batch busybox
Name:           busybox
Namespace:      default
Selector:       controller-uid=6cc4690b-c29a-4987-a547-7594464e6353
Labels:         controller-uid=6cc4690b-c29a-4987-a547-7594464e6353
                job-name=busybox
Annotations:    <none>
Parallelism:    1
Completions:    1
Start Time:     Tue, 06 Apr 2021 21:47:59 +0800
Completed At:   Tue, 06 Apr 2021 21:48:11 +0800
Duration:       12s
Pods Statuses:  0 Running / 1 Succeeded / 0 Failed
Pod Template:
  Labels:  controller-uid=6cc4690b-c29a-4987-a547-7594464e6353
           job-name=busybox
  Containers:
   busybox:
    Image:      busybox:1.30.0
    Port:       <none>
    Host Port:  <none>
    Command:
      /bin/sh
      -c
      sleep 10;date
    Environment:  <none>
    Mounts:       <none>
  Volumes:        <none>
Events:
  Type    Reason            Age    From            Message
  ----    ------            ----   ----            -------
  Normal  SuccessfulCreate  4m40s  job-controller  Created pod: busybox-zmnqq
  Normal  Completed         4m28s  job-controller  Job completed

```

## 总结

Job 可以启动一个任务，并且可以执行完成。

## 附录
