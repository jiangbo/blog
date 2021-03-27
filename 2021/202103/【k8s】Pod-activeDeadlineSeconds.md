# 【k8s】Pod-activeDeadlineSeconds

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

activeDeadlineSeconds 表示 Pod 可以运行的最长时间，达到设置的该值后，Pod 会自动停止。

## 示例

### Pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
spec:
  activeDeadlineSeconds: 30
  containers:
    - name: spring-k8s
      image: jiangbo920827/spring-k8s:liveness
      ports:
        - containerPort: 8080
```

### 查看 Pod

```
[root@master ~]# kubectl get pod
NAME         READY   STATUS    RESTARTS   AGE
spring-k8s   1/1     Running   0          9s
[root@master ~]# kubectl get pod --watch
NAME         READY   STATUS    RESTARTS   AGE
spring-k8s   1/1     Running   0          14s
spring-k8s   0/1     DeadlineExceeded   0          30s
```

### describe

```
[root@master ~]# kubectl describe pod spring-k8s
Name:         spring-k8s
Namespace:    default
Priority:     0
Node:         node2/
Start Time:   Sat, 27 Mar 2021 23:09:18 +0800
Labels:       <none>
Annotations:  <none>
Status:       Failed
Reason:       DeadlineExceeded
Message:      Pod was active on the node longer than the specified deadline
IP:           10.244.2.164
IPs:
  IP:  10.244.2.164
...

Events:
  Type    Reason            Age                From               Message
  ----    ------            ----               ----               -------
  Normal  Scheduled         118s               default-scheduler  Successfully assigned default/spring-k8s to node2
  Normal  Pulled            113s               kubelet            Container image "jiangbo920827/spring-k8s:liveness" already present on machine
  Normal  Created           113s               kubelet            Created container spring-k8s
  Normal  Started           113s               kubelet            Started container spring-k8s
  Normal  Killing           89s                kubelet            Stopping container spring-k8s
  Normal  DeadlineExceeded  87s (x3 over 89s)  kubelet            Pod was active on the node longer than the specified deadline
```

## 总结

activeDeadlineSeconds 可以设置 Pod 最长的运行时间。

## 附录
