# 【Kubernetes】手动创建 Pod 集群

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

现在我们已经可以创建一个 Pod 来提供服务了。但是，在生产上，一般来说最少需要三台服务器来提供服务，
避免其中一台服务出错导致整个服务不能访问。现在我们也试着创建三个 Pod 的集群。

## 创建集群

### 创建第一个 Pod

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: pod1
spec:
  containers:
    - name: pod1
      image: jiangbo920827/spring-demo:actuator
      ports:
        - containerPort: 8080
      resources:
        limits:
          memory: 200Mi
          cpu: 200m

```

### 创建第二个 Pod

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: pod2
spec:
  containers:
    - name: pod2
      image: jiangbo920827/spring-demo:actuator
      ports:
        - containerPort: 8080
      resources:
        limits:
          memory: 200Mi
          cpu: 200m

```

### 创建第三个 Pod

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: pod3
spec:
  containers:
    - name: pod3
      image: jiangbo920827/spring-demo:actuator
      ports:
        - containerPort: 8080
      resources:
        limits:
          memory: 200Mi
          cpu: 200m

```

## Pod 集群

### 查看 Pod 状态

```
[root@master pod]# kubectl get pod -o wide
NAME   READY   STATUS    RESTARTS   AGE     IP            NODE    NOMINATED NODE   READINESS GATES
pod1   1/1     Running   0          5m26s   10.244.1.70   node1   <none>           <none>
pod2   1/1     Running   0          3m36s   10.244.1.71   node1   <none>           <none>
pod3   1/1     Running   0          3m25s   10.244.2.36   node2   <none>           <none>
[root@master pod]#
```

### 访问测试

```
[root@master pod]# curl 10.244.1.70:8080/hostname
pod1 actuator[root@master pod]# curl 10.244.1.71:8080/hostname
pod2 actuator[root@master pod]# curl 10.244.2.36:8080/hostname
pod3 actuator[root@master pod]#
```

可以看到在访问 Pod1, 2, 3 的时候，都成功了，我们很容易就拥有了相当于三台服务器提供的服务。
如果在这三个 Pod 的前面加上负载均衡器，将 IP 地址配置进去，集群环境将搭建好了。
同时，还具有一定的自恢复功能，如果 Pod 中的容器因为意外停止了，还可以自动重启。

在 k8s，即 Kubernetes（因为中间省略了 8 个字母），像这种多个 Pod 的形式，叫做多副本（replica）。

## 总结

介绍了手动搭建一个多 Pod 的集群，也叫多副本。

## 附录
