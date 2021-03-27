# 【k8s】Container-readinessProbe

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

直接创建一个 Pod，查看其 metadata 字段的值。

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
```

### 查看属性

```
[root@master ~]# kubectl describe pod
Name:         spring-k8s
Namespace:    default
Priority:     0
Node:         node2/192.168.56.103
Start Time:   Sat, 27 Mar 2021 22:16:39 +0800
Labels:       <none>
Annotations:  <none>
Status:       Running
IP:           10.244.2.162
IPs:
  IP:  10.244.2.162
```

### get 命令查看

```
[root@master ~]# kubectl get pod spring-k8s -o yaml
apiVersion: v1
kind: Pod
metadata:
  annotations:
    kubectl.kubernetes.io/last-applied-configuration: |
      {"apiVersion":"v1","kind":"Pod","metadata":{"annotations":{},"name":"spring-k8s","namespace":"default"},"spec":{"containers":[{"image":"jiangbo920827/spring-k8s:liveness","name":"spring-k8s","ports":[{"containerPort":8080}]}]}}
  creationTimestamp: "2021-03-27T14:16:39Z"
  name: spring-k8s
  namespace: default
  resourceVersion: "474956"
  uid: 97762610-f00a-4390-9706-7ef36058ce5b
```

其中的 annotations 在之前已经学习过，creationTimestamp 表示创建时间，没有加时区。
name 时我们自定义的 Pod 的名称，namespace 为命名空间，之后学习。
resourceVersion 表示内部使用的资源版本，uid 系统生成的资源的唯一值。
其中 name 和 uid 的区别是：name 在同一命名空间下，同种资源唯一，而 uid 全局唯一。

## 总结

介绍了 metadata 的几个字段的含义以及作用。

## 附录
