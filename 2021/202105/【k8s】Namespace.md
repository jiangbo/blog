# 【k8s】Namespace

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

k8s 使用 namespace 将工作负载进行隔离，可以实现不同用户使用不同的空间，或者不同组使用不同的空间。
在不指定命名空间的情况下，默认指的是 default 命名空间下的工作负载。namespace 可以简写为 ns。

## 示例

### 查看已有命名空间

```
[root@master ~]# kubectl get namespaces
NAME                   STATUS   AGE
default                Active   96d
kube-node-lease        Active   96d
kube-public            Active   96d
kube-system            Active   96d
kubernetes-dashboard   Active   46d
```

### 新建 namespace

```
[root@master ~]# kubectl create namespace test
namespace/test created
[root@master ~]# kubectl get namespaces
NAME                   STATUS   AGE
default                Active   96d
kube-node-lease        Active   96d
kube-public            Active   96d
kube-system            Active   96d
kubernetes-dashboard   Active   46d
test                   Active   4s
[root@master ~]# kubectl describe namespaces test
Name:         test
Labels:       <none>
Annotations:  <none>
Status:       Active

No resource quota.

No LimitRange resource.
```

### 在 namespace 下创建资源

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
  namespace: test
spec:
  containers:
    - name: spring-k8s
      image: jiangbo920827/spring-k8s:liveness
      ports:
        - containerPort: 8080
```

### 查看

```
[root@master ~]# kubectl get pod -n test -o wide
NAME         READY   STATUS    RESTARTS   AGE   IP             NODE    NOMINATED NODE   READINESS GATES
spring-k8s   1/1     Running   0          59s   10.244.1.222   node1   <none>           <none>
```

### 删除 namespace

```
[root@master ~]# kubectl delete namespaces test
namespace "test" deleted
[root@master ~]# kubectl get pod -n test -o wide
No resources found in test namespace.
[root@master ~]#
```

删除 namespace，资源也会一起删除。

## 总结

介绍了 namespace 的概念以及使用。

## 附录
