# 【Kubernetes】容器重启策略-Never

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

创建一个 Pod 将其容器的重启策略设置为：Never，查看容器停止时，Pod 的行为。

## pod.yaml

直接创建 Pod 时，如果 restartPolicy 不写，默认为：Always。

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: pod-demo

spec:
  restartPolicy: Never
  containers:
    - name: pod-demo
      image: jiangbo920827/spring-demo:actuator
      ports:
        - containerPort: 8080

```

## 查看 Pod

```sh
[root@master pod]# kubectl get -f pod.yaml -o wide
NAME       READY   STATUS    RESTARTS   AGE   IP            NODE    NOMINATED NODE   READINESS GATES
pod-demo   1/1     Running   0          14s   10.244.1.19   node1   <none>           <none>

```

可以看到 Pod 正常运行，RESTARTS（重启次数）字段为 0。

## 停止容器

### 正常停止

```sh
[root@master pod]# curl -X POST 10.244.1.19:8080/actuator/shutdown
{"message":"Shutting down, bye..."}[root@master pod]#
[root@master pod]# kubectl get -f pod.yaml -o wide
NAME       READY   STATUS      RESTARTS   AGE   IP            NODE    NOMINATED NODE   READINESS GATES
pod-demo   0/1     Completed   0          66s   10.244.1.19   node1   <none>           <none>
```

状态变成了 Completed，READY 字段也变成了 0/1，也没有再次重启。

### 非正常停止

删除容器，再次创建，并且非正常停止。

```sh
[root@master pod]# kubectl delete -f pod.yaml
pod "pod-demo" deleted
[root@master pod]# kubectl apply -f pod.yaml
pod/pod-demo created
[root@master pod]# kubectl get -f pod.yaml -o wide
NAME       READY   STATUS    RESTARTS   AGE   IP            NODE    NOMINATED NODE   READINESS GATES
pod-demo   1/1     Running   0          4s    10.244.1.20   node1   <none>           <none>
[root@master pod]# kubectl exec pod-demo -- kill 1
[root@master pod]# kubectl get -f pod.yaml -o wide
NAME       READY   STATUS   RESTARTS   AGE   IP            NODE    NOMINATED NODE   READINESS GATES
pod-demo   0/1     Error    0          34s   10.244.1.20   node1   <none>           <none>

```

状态变成了 Error，READY 字段也变成了 0/1，也没有再次重启。

## 总结

介绍了容器的重启策略-Never，在创建单个 Pod 的情况下，都不会重启。
容器的重启有一个回退策略，并以指数级增加。如果在 10 分钟没有错误，则重置重启计时。
详细说明见附录摘抄的官网说明。

## 附录

### Container restart policy

```markdown
The `spec` of a Pod has a `restartPolicy` field with possible values Always, OnFailure, and Never. The default value is Always.

The `restartPolicy` applies to all containers in the Pod. `restartPolicy` only refers to restarts of the containers by the kubelet on the same node. After containers in a Pod exit, the kubelet restarts them with an exponential back-off delay (10s, 20s, 40s, …), that is capped at five minutes. Once a container has executed for 10 minutes without any problems, the kubelet resets the restart backoff timer for that container.
```