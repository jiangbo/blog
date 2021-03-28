# 【k8s】Pod-readinessGates

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

除了之前学习过的四种条件外，readinessGates 可以自定义 Pod 条件信息。

## 示例

### Pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
spec:
  readinessGates:
    - conditionType: my-condition
  containers:
    - name: spring-k8s
      image: jiangbo920827/spring-k8s:liveness
      ports:
        - containerPort: 8080
```

### 查看

```
[root@master ~]# kubectl get pod -o wide
NAME         READY   STATUS    RESTARTS   AGE   IP             NODE    NOMINATED NODE   READINESS GATES
spring-k8s   1/1     Running   0          8s    10.244.2.185   node2   <none>           0/1
[root@master ~]# kubectl get pod spring-k8s -o json | jq .status.conditions
[
  {
    "lastProbeTime": null,
    "lastTransitionTime": "2021-03-28T10:01:51Z",
    "status": "True",
    "type": "Initialized"
  },
  {
    "lastProbeTime": null,
    "lastTransitionTime": "2021-03-28T10:01:51Z",
    "message": "corresponding condition of pod readiness gate \"my-condition\" does not exist.",
    "reason": "ReadinessGatesNotReady",
    "status": "False",
    "type": "Ready"
  },
  {
    "lastProbeTime": null,
    "lastTransitionTime": "2021-03-28T10:01:52Z",
    "status": "True",
    "type": "ContainersReady"
  },
  {
    "lastProbeTime": null,
    "lastTransitionTime": "2021-03-28T10:01:51Z",
    "status": "True",
    "type": "PodScheduled"
  }
]
```

可以看到 Read 条件因为我们自定义的条件不满足而变成了 False。
由于状态字段不能通过 kubectl 命令进行修改，所以只能使用 API 的方式修改。

### 开放 API 端点

```
[root@master ~]# kubectl proxy --accept-hosts=".*" --address=0.0.0.0
Starting to serve on [::]:8001

```

### 更新 readinessGates

```
[root@master ~]# curl http://localhost:8001/api/v1/namespaces/default/pods/spring-k8s/status -X PATCH -H "Content-Type: application/json-patch+json" -d '[{"op": "add", "path": "/status/conditions/-", "value": {"type": "my-condition", "status": "True", "lastProbeTime": null}}]'
```

### 查看 readinessGates

```
[root@master ~]# kubectl get pod -o wide
NAME         READY   STATUS    RESTARTS   AGE     IP             NODE    NOMINATED NODE   READINESS GATES
spring-k8s   1/1     Running   0          5m37s   10.244.2.186   node2   <none>           1/1
[root@master ~]# kubectl get pod spring-k8s -o json | jq .status.conditions
[
  {
    "lastProbeTime": null,
    "lastTransitionTime": null,
    "status": "True",
    "type": "my-condition"
  },
  {
    "lastProbeTime": null,
    "lastTransitionTime": "2021-03-28T10:20:53Z",
    "status": "True",
    "type": "Initialized"
  },
  {
    "lastProbeTime": null,
    "lastTransitionTime": "2021-03-28T10:26:22Z",
    "status": "True",
    "type": "Ready"
  },
  {
    "lastProbeTime": null,
    "lastTransitionTime": "2021-03-28T10:20:55Z",
    "status": "True",
    "type": "ContainersReady"
  },
  {
    "lastProbeTime": null,
    "lastTransitionTime": "2021-03-28T10:20:53Z",
    "status": "True",
    "type": "PodScheduled"
  }
]
```

## 总结

介绍了 Pod 的 readinessGates 字段，通过条件的方式，可以自定义 Read 状态的时间。

## 附录
