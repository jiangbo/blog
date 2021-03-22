# 【Kubernetes】容器内存限制

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

创建一个 Pod 限制其使用不同的内存量，查看 Pod 的状态。

## 限制最小内存

### pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: pod-demo

spec:
  containers:
    - name: pod-demo
      image: jiangbo920827/spring-demo:actuator
      ports:
        - containerPort: 8080
      resources:
        requests:
          memory: 100Mi

```

### 查看请求限制

可以看到内存请求量已经生效了

```text
[root@master pod]# kubectl describe -f pod.yaml 
Name:         pod-demo
Namespace:    default
Priority:     0
Node:         node1/192.168.56.102
Start Time:   Tue, 26 Jan 2021 23:44:25 +0800
Labels:       <none>
Annotations:  <none>
Status:       Running
IP:           10.244.1.35
IPs:
  IP:  10.244.1.35
Containers:
  pod-demo:
    Container ID:   docker://44eccd49e8cff129eedb1e439785c383d70a3fae74e377f216240344d4e02159
    Image:          jiangbo920827/spring-demo:actuator
    Image ID:       docker-pullable://jiangbo920827/spring-demo@sha256:fef2dd74c274e783e4cf2f270da15cadbc0766c8a0d24dad31dd0258e2eb4722
    Port:           8080/TCP
    Host Port:      0/TCP
    State:          Running
      Started:      Tue, 26 Jan 2021 23:44:26 +0800
    Ready:          True
    Restart Count:  0
    Requests:
      memory:     100Mi
    Environment:  <none>
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
QoS Class:       Burstable
Node-Selectors:  <none>
Tolerations:     node.kubernetes.io/not-ready:NoExecute op=Exists for 300s
                 node.kubernetes.io/unreachable:NoExecute op=Exists for 300s
Events:
  Type    Reason     Age   From               Message
  ----    ------     ----  ----               -------
  Normal  Scheduled  3s    default-scheduler  Successfully assigned default/pod-demo to node1
  Normal  Pulled     2s    kubelet            Container image "jiangbo920827/spring-demo:actuator" already present on machine
  Normal  Created    2s    kubelet            Created container pod-demo
  Normal  Started    2s    kubelet            Started container pod-demo
```

## 限制最大内存

### pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: pod-demo

spec:
  containers:
    - name: pod-demo
      image: jiangbo920827/spring-demo:actuator
      ports:
        - containerPort: 8080
      resources:
        requests:
          memory: 100Mi
        limits:
          memory: 200Mi

```

### 查看最大限制

```sh
    Ready:          True
    Restart Count:  0
    Limits:
      memory:  200Mi
    Requests:
      memory:     100Mi
    Environment:  <none>
    Mounts:
      /var/run/secrets/kubernetes.io/serviceaccount from default-token-slbq5 (ro)
```

### 不限制请求大小

不限制请求内存的大小，只限制最大，则请求和最大一样多。

```text
    Ready:          True
    Restart Count:  0
    Limits:
      memory:  200Mi
    Requests:
      memory:     200Mi
    Environment:  <none>
    Mounts:
      /var/run/secrets/kubernetes.io/serviceaccount from default-token-slbq5 (ro)

```

## 内存不足

### pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: pod-demo

spec:
  containers:
    - name: pod-demo
      image: jiangbo920827/spring-demo:actuator
      ports:
        - containerPort: 8080
      resources:
        limits:
          memory: 20Mi

```

### OOMKilled

可以看到容器启动失败，失败原因是：OOMKilled。

```
[root@master pod]# kubectl get pod
NAME       READY   STATUS             RESTARTS   AGE
pod-demo   0/1     CrashLoopBackOff   1          9s
[root@master pod]# kubectl describe -f pod.yaml
Name:         pod-demo
Namespace:    default
Priority:     0
Node:         node1/192.168.56.102
Start Time:   Wed, 27 Jan 2021 22:37:33 +0800
Labels:       <none>
Annotations:  <none>
Status:       Running
IP:           10.244.1.47
IPs:
  IP:  10.244.1.47
Containers:
  pod-demo:
    Container ID:   docker://479f54bdef27e9a22d51af01bf3b42d1e38be5b21674b8a40e239d5cb4d6b3b2
    Image:          jiangbo920827/spring-demo:actuator
    Image ID:       docker-pullable://jiangbo920827/spring-demo@sha256:fef2dd74c274e783e4cf2f270da15cadbc0766c8a0d24dad31dd0258e2eb4722
    Port:           8080/TCP
    Host Port:      0/TCP
    State:          Waiting
      Reason:       CrashLoopBackOff
    Last State:     Terminated
      Reason:       OOMKilled
      Exit Code:    137
      Started:      Wed, 27 Jan 2021 22:37:35 +0800
      Finished:     Wed, 27 Jan 2021 22:37:36 +0800
    Ready:          False
    Restart Count:  1
    Limits:
      memory:  20Mi
    Requests:
      memory:     20Mi
    Environment:  <none>
    Mounts:
      /var/run/secrets/kubernetes.io/serviceaccount from default-token-slbq5 (ro)
Conditions:
  Type              Status
  Initialized       True
  Ready             False
  ContainersReady   False
  PodScheduled      True
Volumes:
  default-token-slbq5:
    Type:        Secret (a volume populated by a Secret)
    SecretName:  default-token-slbq5
    Optional:    false
QoS Class:       Burstable
Node-Selectors:  <none>
Tolerations:     node.kubernetes.io/not-ready:NoExecute op=Exists for 300s
                 node.kubernetes.io/unreachable:NoExecute op=Exists for 300s
Events:
  Type     Reason     Age                From               Message
  ----     ------     ----               ----               -------
  Normal   Scheduled  18s                default-scheduler  Successfully assigned default/pod-demo to node1
  Normal   Pulled     17s (x2 over 18s)  kubelet            Container image "jiangbo920827/spring-demo:actuator" already present on machine
  Normal   Created    17s (x2 over 18s)  kubelet            Created container pod-demo
  Normal   Started    17s (x2 over 18s)  kubelet            Started container pod-demo
  Warning  BackOff    15s (x2 over 16s)  kubelet            Back-off restarting failed container

```

## 总结

介绍了容器的内存资源限制，可以限制容器请求量，也可以限制最大量。容器的内存资源限制不能动态修改，修改时会提示错误。
在内存资源不足时，容器将因为 Terminated 失败，并且不断地重启。

## 附录
