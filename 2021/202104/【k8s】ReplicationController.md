# 【k8s】ReplicationController

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

ReplicationController 副本控制器，可以控制启动的 Pod 的数量。

## 示例

### rc.yaml

```yaml
apiVersion: v1
kind: ReplicationController
metadata:
  name: spring-k8s
spec:
  template:
    metadata:
      labels:
        app: spring-k8s
    spec:
      containers:
        - name: spring-k8s
          image: jiangbo920827/spring-k8s:liveness
          ports:
            - containerPort: 8080
```

### 查看

```
[root@master manifests]# kubectl get rc,pod
NAME                               DESIRED   CURRENT   READY   AGE
replicationcontroller/spring-k8s   1         1         1       33m

NAME                   READY   STATUS    RESTARTS   AGE
pod/spring-k8s-blhqk   1/1     Running   0          33m

```

查看 ReplicationController 的时候，可以看到需要的副本数，当前的副本数，以及有几个准备完成。

### 查看 Pod

```
Name:         spring-k8s-blhqk
Namespace:    default
Priority:     0
Node:         node2/192.168.56.103
Start Time:   Sun, 28 Mar 2021 22:24:46 +0800
Labels:       app=spring-k8s
Annotations:  <none>
Status:       Running
IP:           10.244.2.193
IPs:
  IP:           10.244.2.193
Controlled By:  ReplicationController/spring-k8s
Containers:
  spring-k8s:
    Container ID:   docker://144b9ce2f0c4130246bff002e91c99809008ffa3fc1672c5f59ea97eab60f6f5
    Image:          jiangbo920827/spring-k8s:liveness
    Image ID:       docker://sha256:27e1956a7558e66cc463d09c86bcda059fd6534d520a9ab68fb8567048f786f2
    Port:           8080/TCP
    Host Port:      0/TCP
    State:          Running
      Started:      Sun, 28 Mar 2021 22:24:47 +0800
    Ready:          True
    Restart Count:  0
    Environment:    <none>
    Mounts:
      /var/run/secrets/kubernetes.io/serviceaccount from default-token-slbq5 (ro)
Conditions:
  Type              Status
  Initialized       True
  Ready             True
  ContainersReady   True
  PodScheduled      True
Volumes:
  default-token-slbq5:
    Type:        Secret (a volume populated by a Secret)
    SecretName:  default-token-slbq5
    Optional:    false
QoS Class:       BestEffort
Node-Selectors:  <none>
Tolerations:     node.kubernetes.io/not-ready:NoExecute op=Exists for 300s
                 node.kubernetes.io/unreachable:NoExecute op=Exists for 300s
Events:
  Type    Reason     Age   From               Message
  ----    ------     ----  ----               -------
  Normal  Scheduled  33m   default-scheduler  Successfully assigned default/spring-k8s-blhqk to node2
  Normal  Pulled     33m   kubelet            Container image "jiangbo920827/spring-k8s:liveness" already present on machine
  Normal  Created    33m   kubelet            Created container spring-k8s
  Normal  Started    33m   kubelet            Started container spring-k8s
```

其中 Pod 和之前学习过的没有区别，多了一个 Controlled By:  ReplicationController/spring-k8s 的字段。
表示这个 Pod 是被 ReplicationController/spring-k8s 控制的。Pod 的名称是 rc 名称加上一个随机串组成的。

### 查看 rc

```
[root@master manifests]# kubectl describe rc
Name:         spring-k8s
Namespace:    default
Selector:     app=spring-k8s
Labels:       app=spring-k8s
Annotations:  <none>
Replicas:     1 current / 1 desired
Pods Status:  1 Running / 0 Waiting / 0 Succeeded / 0 Failed
Pod Template:
  Labels:  app=spring-k8s
  Containers:
   spring-k8s:
    Image:        jiangbo920827/spring-k8s:liveness
    Port:         8080/TCP
    Host Port:    0/TCP
    Environment:  <none>
    Mounts:       <none>
  Volumes:        <none>
Events:
  Type    Reason            Age   From                    Message
  ----    ------            ----  ----                    -------
  Normal  SuccessfulCreate  39m   replication-controller  Created pod: spring-k8s-blhqk
```

其中的字段基本上都是学习过的，Selector 表示标签选择器，Labels 表示标签。Pod Template 中的字段在 Pod 中已经学习过。
Replicas 表示副本数，即启动几个 Pod。Pod Status 表示 Pod 的状态。

## 总结

介绍 ReplicationController，简写 rc，的概念，以及创建一个 rc 的实例。

## 附录
