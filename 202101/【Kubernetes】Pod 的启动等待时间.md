# 【Kubernetes】Pod 的启动等待时间

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

现在我们有一个副本控制器 rc，可以很方便地实现扩容与缩容操作。

## 缩容

replicas 没有写，默认为 1。
selector 没有写，默认和 Pod 的标签一致。

### 缩容 rc.yaml

```yaml
apiVersion: v1
kind: ReplicationController
metadata:
  name: rc-demo
spec:
  template:
    metadata:
      labels:
        app: myapp
    spec:
      containers:
        - name: pod-demo
          image: jiangbo920827/spring-demo:actuator
          ports:
            - containerPort: 8080

```

### 查看缩容 Pod

```
[root@master kubernetes]# kubectl get pod -o wide
NAME            READY   STATUS    RESTARTS   AGE   IP            NODE    NOMINATED NODE   READINESS GATES
rc-demo-7dnrc   1/1     Running   0          17s   10.244.1.78   node1   <none>           <none>
```

### 查看缩容 rc

```
[root@master kubernetes]# kubectl get rc
NAME      DESIRED   CURRENT   READY   AGE
rc-demo   1         1         1       15h
```

## 扩容

### 扩容 rc.yaml

```yaml
apiVersion: v1
kind: ReplicationController
metadata:
  name: rc-demo
spec:
  replicas: 3
  template:
    metadata:
      labels:
        app: myapp
    spec:
      containers:
        - name: pod-demo
          image: jiangbo920827/spring-demo:actuator
          ports:
            - containerPort: 8080

```

### 查看扩容 Pod

```
[root@master kubernetes]# kubectl get pod  -o wide
NAME            READY   STATUS    RESTARTS   AGE     IP            NODE    NOMINATED NODE   READINESS GATES
rc-demo-7dnrc   1/1     Running   0          3m55s   10.244.1.78   node1   <none>           <none>
rc-demo-rcn7s   1/1     Running   0          23s     10.244.1.79   node1   <none>           <none>
rc-demo-xww8g   1/1     Running   0          23s     10.244.1.80   node1   <none>           <none>
```

### 查看扩容 rc

```
[root@master kubernetes]# kubectl get -f rc.yaml -o wide
NAME      DESIRED   CURRENT   READY   AGE   CONTAINERS   IMAGES                               SELECTOR
rc-demo   3         3         3       15h   pod-demo     jiangbo920827/spring-demo:actuator   app=myapp
```

## 命令扩容和缩容

```
[root@master kubernetes]# kubectl get pod -o wide
NAME            READY   STATUS    RESTARTS   AGE     IP            NODE    NOMINATED NODE   READINESS GATES
rc-demo-7dnrc   1/1     Running   0          7m53s   10.244.1.78   node1   <none>           <none>
rc-demo-rcn7s   1/1     Running   0          4m21s   10.244.1.79   node1   <none>           <none>
rc-demo-xww8g   1/1     Running   0          4m21s   10.244.1.80   node1   <none>           <none>
[root@master kubernetes]# kubectl scale --replicas=1 rc rc-demo
replicationcontroller/rc-demo scaled

[root@master kubernetes]# kubectl get pod -o wide
NAME            READY   STATUS    RESTARTS   AGE     IP            NODE    NOMINATED NODE   READINESS GATES
rc-demo-7dnrc   1/1     Running   0          9m31s   10.244.1.78   node1   <none>           <none>

[root@master kubernetes]# kubectl scale --replicas=3 rc rc-demo
replicationcontroller/rc-demo scaled

[root@master kubernetes]# kubectl get pod -o wide
NAME            READY   STATUS    RESTARTS   AGE     IP            NODE    NOMINATED NODE   READINESS GATES
rc-demo-7dnrc   1/1     Running   0          9m54s   10.244.1.78   node1   <none>           <none>
rc-demo-7kl4t   1/1     Running   0          14s     10.244.2.41   node2   <none>           <none>
rc-demo-nl5pq   1/1     Running   0          14s     10.244.1.81   node1   <none>           <none>

```

通过命令，可以很方便地进行扩容和缩容操作。
但是在操作的过程中，发现 Pod 的 IP 变化了，而且所在节点服务器也有可能发生变化。
如果是在这几个 Pod 的前面增加负载均衡器，那么就不得不修改配置的 IP 地址。
实际上，在 Pod 重启后（和容器的重启区分），是新的一个 Pod，k8s 在设计上，Pod 应该是短暂的，随时可以替换的。
所以不应该依赖 Pod 所在的 IP。

## 总结

介绍了通过副本控制器很容易地进行扩容和缩容操作。同时也发现了一个问题，Pod 的 IP 地址是不固定的，给我们的访问增加了很大的难度。

## 附录
