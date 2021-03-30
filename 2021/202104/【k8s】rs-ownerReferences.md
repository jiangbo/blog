# 【k8s】rs-ownerReferences

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

ownerReferences 字段表示了该资源所属的资源的，表示由谁控制。

## 示例

### rc.yaml

```yaml
apiVersion: apps/v1
kind: ReplicaSet
metadata:
  name: spring-k8s
spec:
  selector:
    matchExpressions:
      - key: app
        operator: Exists
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

### 查看

```
[root@master manifests]# kubectl get pod -o yaml
...
    ownerReferences:
    - apiVersion: apps/v1
      blockOwnerDeletion: true
      controller: true
      kind: ReplicaSet
      name: spring-k8s
      uid: 90812bb6-d0e1-455a-b8e3-84af0c2a3d4f
    resourceVersion: "552132"
    uid: ff4e2d11-3524-4647-929c-539715e10e94
...
```

可以看到这里列出了 Pod 被 rs 控制的信息。

### ControlledBy

```
^C[root@master docker]# kubectl describe pod spring-k8s-7s88z
Name:         spring-k8s-7s88z
Namespace:    default
Priority:     0
Node:         node2/192.168.56.103
Start Time:   Tue, 30 Mar 2021 21:11:50 +0800
Labels:       app=spring-k8s
Annotations:  <none>
Status:       Running
IP:           10.244.2.232
IPs:
  IP:           10.244.2.232
Controlled By:  ReplicaSet/spring-k8s
```

Controlled By 表示正在被哪个资源控制。

## 总结

ownerReferences 引用了该种资源的所属资源。

## 附录
