# 【k8s】ReplicaSet

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

ReplicaSet 和 rc 差不多，都是来控制 Pod 的副本数的，唯一的区别是标签选择器。

## 示例

### rc.yaml

```yaml
apiVersion: apps/v1
kind: ReplicaSet
metadata:
  name: spring-k8s
spec:
  selector:
    matchLabels:
      app: spring-k8s
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

selector 字段不是直接填写标签，而是通过 matchLabels 来定义标签，而且标签选择器不可以省略。

### 查看

```
[root@master manifests]# kubectl get rs,pod
NAME                         DESIRED   CURRENT   READY   AGE
replicaset.apps/spring-k8s   1         1         1       8s

NAME                   READY   STATUS    RESTARTS   AGE
pod/spring-k8s-5v5wb   1/1     Running   0          8s
```

### 查看 rs

```
[root@master manifests]# kubectl describe rs
Name:         spring-k8s
Namespace:    default
Selector:     app=spring-k8s
Labels:       <none>
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
  Type    Reason            Age    From                   Message
  ----    ------            ----   ----                   -------
  Normal  SuccessfulCreate  3m32s  replicaset-controller  Created pod: spring-k8s-5v5wb
```

## 总结

介绍了 ReplicaSet 的概念，以及定义一个 ReplicaSet。

## 附录
