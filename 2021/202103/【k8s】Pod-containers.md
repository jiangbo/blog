# 【k8s】Pod-containers

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

在 Pod 配置容器的时候，containers 字段是一个数组，也就是说，可以在一个 Pod 中配置多个容器。

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
    - name: busybox
      image: busybox:stable
      command: ["/bin/sh", "-c", "sleep 3600"]
```

### 查看

```
[root@master ~]# kubectl get pod -o wide
NAME         READY   STATUS    RESTARTS   AGE   IP             NODE    NOMINATED NODE   READINESS GATES
spring-k8s   2/2     Running   0          9s    10.244.2.191   node2   <none>           <none>
```

### 查看容器信息

```
[root@master ~]# kubectl get pod spring-k8s -o json | jq .spec.containers
[
  {
    "image": "jiangbo920827/spring-k8s:liveness",
    "imagePullPolicy": "IfNotPresent",
    "name": "spring-k8s",
    "ports": [
      {
        "containerPort": 8080,
        "protocol": "TCP"
      }
    ],
    "resources": {},
    "terminationMessagePath": "/dev/termination-log",
    "terminationMessagePolicy": "File",
    "volumeMounts": [
      {
        "mountPath": "/var/run/secrets/kubernetes.io/serviceaccount",
        "name": "default-token-slbq5",
        "readOnly": true
      }
    ]
  },
  {
    "command": [
      "/bin/sh",
      "-c",
      "sleep 3600"
    ],
    "image": "busybox:stable",
    "imagePullPolicy": "IfNotPresent",
    "name": "busybox",
    "resources": {},
    "terminationMessagePath": "/dev/termination-log",
    "terminationMessagePolicy": "File",
    "volumeMounts": [
      {
        "mountPath": "/var/run/secrets/kubernetes.io/serviceaccount",
        "name": "default-token-slbq5",
        "readOnly": true
      }
    ]
  }
]
[root@master ~]#
```

## 总结

Pod 中，可以定义多个容器，每个容器可以有自己的功能。

## 附录
