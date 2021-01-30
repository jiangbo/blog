# 【Kubernetes】副本的删除

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

在副本控制器的作用下，删除 Pod 副本。

## 普通删除

```
[root@master kubernetes]# kubectl get pod
NAME            READY   STATUS    RESTARTS   AGE
rc-demo-7dnrc   1/1     Running   0          65m
rc-demo-7kl4t   1/1     Running   0          55m
rc-demo-nl5pq   1/1     Running   0          55m
[root@master kubernetes]# kubectl delete pod rc-demo-nl5pq
pod "rc-demo-nl5pq" deleted
[root@master kubernetes]# kubectl get pod
NAME            READY   STATUS    RESTARTS   AGE
rc-demo-7dnrc   1/1     Running   0          66m
rc-demo-7kl4t   1/1     Running   0          56m
rc-demo-r7pxg   1/1     Running   0          47s
```

可以看到，普通删除副本后，副本控制器会再次新建一个 Pod 出来。所以说是删除生效了，但是又新建了一个。
可以利用这种方式，来达到重启 Pod 的目的。

## 批量删除

可以通过标签选择器批量删除 Pod，还达到重启所有 Pod 的目的。

```
[root@master kubernetes]# kubectl get pod --show-labels
NAME            READY   STATUS    RESTARTS   AGE    LABELS
rc-demo-7dnrc   1/1     Running   0          105m   app=myapp
rc-demo-7kl4t   1/1     Running   0          96m    app=myapp
rc-demo-r7pxg   1/1     Running   0          40m    app=myapp
[root@master kubernetes]# kubectl delete pod -l app=myapp
pod "rc-demo-7dnrc" deleted
pod "rc-demo-7kl4t" deleted
pod "rc-demo-r7pxg" deleted
[root@master kubernetes]# kubectl get pod --show-labels
NAME            READY   STATUS    RESTARTS   AGE   LABELS
rc-demo-d7lzb   1/1     Running   0          60s   app=myapp
rc-demo-n6c88   1/1     Running   0          60s   app=myapp
rc-demo-pzj8w   1/1     Running   0          60s   app=myapp
```

## 删除副本控制器

### 不删除 Pod

```
[root@master kubernetes]# kubectl get pod --show-labels
NAME            READY   STATUS    RESTARTS   AGE   LABELS
rc-demo-cqmb7   1/1     Running   0          10s   app=myapp
rc-demo-pc87m   1/1     Running   0          10s   app=myapp
rc-demo-q59cg   1/1     Running   0          10s   app=myapp
[root@master kubernetes]# kubectl delete rc rc-demo --cascade=orphan
replicationcontroller "rc-demo" deleted
[root@master kubernetes]# kubectl get pod --show-labels
NAME            READY   STATUS    RESTARTS   AGE   LABELS
rc-demo-cqmb7   1/1     Running   0          19s   app=myapp
rc-demo-pc87m   1/1     Running   0          19s   app=myapp
rc-demo-q59cg   1/1     Running   0          19s   app=myapp
```

### 删除 Pod

```
[root@master kubernetes]# kubectl get pod --show-labels
NAME            READY   STATUS    RESTARTS   AGE   LABELS
rc-demo-cqmb7   1/1     Running   0          65s   app=myapp
rc-demo-pc87m   1/1     Running   0          65s   app=myapp
rc-demo-q59cg   1/1     Running   0          65s   app=myapp
[root@master kubernetes]# kubectl delete rc rc-demo
replicationcontroller "rc-demo" deleted
[root@master kubernetes]# kubectl get all
NAME                 TYPE        CLUSTER-IP   EXTERNAL-IP   PORT(S)   AGE
service/kubernetes   ClusterIP   10.96.0.1    <none>        443/TCP   6d
```

## 总结

介绍了副本控制器和 Pod 的删除方式，以及重启的代替方案。

## 附录
