# 【k8s】Pod-nodeSelector

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

nodeSelector 可以将 Pod 指定到某个节点运行，和 nodeName 不同的是它使用标签选择。

## 示例

### Pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
spec:
  containers:
    - name: spring-k8s
      image: jiangbo920827/spring-k8s:liveness
      ports:
        - containerPort: 8080
  nodeSelector:
    kubernetes.io/hostname: node1
```

### 查看

```
[root@master ~]# kubectl get pod -o wide
NAME         READY   STATUS    RESTARTS   AGE   IP             NODE    NOMINATED NODE   READINESS GATES
spring-k8s   1/1     Running   0          9s    10.244.1.218   node1   <none>           <none>
```

### 选择主节点

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
spec:
  containers:
    - name: spring-k8s
      image: jiangbo920827/spring-k8s:liveness
      ports:
        - containerPort: 8080
  nodeSelector:
    kubernetes.io/hostname: master
```

### 查看事件

```
Events:
  Type     Reason            Age   From               Message
  ----     ------            ----  ----               -------
  Warning  FailedScheduling  19s   default-scheduler  0/3 nodes are available: 1 node(s) had taint {node-role.kubernetes.io/master: }, that the pod didn't tolerate, 2 node(s) didn't match Pod's node affinity.
  Warning  FailedScheduling  19s   default-scheduler  0/3 nodes are available: 1 node(s) had taint {node-role.kubernetes.io/master: }, that the pod didn't tolerate, 2 node(s) didn't match Pod's node affinity.
```

可以看到调度到 master 失败了，因为 master 上有 taint，即污点，不能被调度上去。

## 总结

nodeSelector 可以根据标签选择 Pod 运行的节点。

## 附录
