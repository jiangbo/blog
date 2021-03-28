# 【k8s】Pod-staticPod

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

staticPod 即静态 Pod。k8s 系统组件的 Pod 都是以静态 Pod 的方式启动的。

## 示例

### Pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
spec:
  shareProcessNamespace: true
  containers:
    - name: spring-k8s
      image: jiangbo920827/spring-k8s:liveness
      ports:
        - containerPort: 8080
```

### 定义

在 /etc/kubernetes/manifests 目录下，可以看到 k8s 定义的静态 Pod 的配置文件。
因为这里是 master 节点，所以有四个，分别是 etcd，kube-apiserver，kube-controller-manager
和 kube-scheduler。其中的 pod.yaml 是我们刚刚新增进去的。

```
[root@master manifests]# pwd
/etc/kubernetes/manifests
[root@master manifests]# ll
total 20
-rw------- 1 root root 2218 Jan 24 15:44 etcd.yaml
-rw------- 1 root root 3335 Mar 15 22:15 kube-apiserver.yaml
-rw------- 1 root root 2827 Mar 15 22:15 kube-controller-manager.yaml
-rw------- 1 root root 1413 Mar 15 22:15 kube-scheduler.yaml
-rw-r--r-- 1 root root  218 Mar 28 22:01 pod.yaml
```

### 查看 Pod

```
[root@master manifests]# kubectl get pod -o wide
NAME                READY   STATUS    RESTARTS   AGE    IP            NODE     NOMINATED NODE   READINESS GATES
spring-k8s-master   1/1     Running   0          5m9s   10.244.0.29   master   <none>           <none>
```

可以看到自动启动了一个 Pod，并且名字中自动加上了 master 后缀。并且当我们删除 Pod 时，它会自动重启。

## 总结

介绍了静态 Pod 的概念，并且新建了一个静态 Pod，也看到 k8s 使用静态 Pod 实现系统组件。

## 附录
