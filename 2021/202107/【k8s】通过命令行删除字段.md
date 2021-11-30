# 【k8s】通过命令行删除字段

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.1

## 目标

在 shell 中，通过命令将已有的字段属性删除。

## 示例

### deploy.yaml

配置了一个存活探针，接下来会将其删除。

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: busybox
spec:
  selector:
    matchLabels:
      app: busybox
  template:
    metadata:
      labels:
        app: busybox
    spec:
      containers:
        - name: busybox
          image: busybox:1.31.0
          command: ["/bin/sh", "-c", "sleep 3600"]
          livenessProbe:
            exec:
              command: ["sh", "-c", "date"]
```

### 修改前查看

```yaml
[root@master ~]# kubectl get deployments.apps busybox -o yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: busybox
  namespace: default
  resourceVersion: "1933810"
  uid: cdf95c81-469f-4d40-99e5-baf1ed2d2187
spec:
  progressDeadlineSeconds: 600
  replicas: 1
  revisionHistoryLimit: 10
  selector:
    matchLabels:
      app: busybox
  strategy:
    rollingUpdate:
      maxSurge: 25%
      maxUnavailable: 25%
    type: RollingUpdate
  template:
    metadata:
      creationTimestamp: null
      labels:
        app: busybox
    spec:
      containers:
      - command:
        - /bin/sh
        - -c
        - sleep 3600
        image: busybox:1.31.0
        imagePullPolicy: IfNotPresent
        livenessProbe:
          exec:
            command:
            - sh
            - -c
            - date
          failureThreshold: 3
          periodSeconds: 10
          successThreshold: 1
          timeoutSeconds: 1
...
```

### 使用命令删除字段

```text
[root@master ~]# kubectl patch deployment busybox  --type json   \
-p='[{"op": "remove", "path": "/spec/template/spec/containers/0/livenessProbe"}]'
deployment.apps/busybox patched
```

### 修改后查看

```yaml
...
  template:
    metadata:
      creationTimestamp: null
      labels:
        app: busybox
    spec:
      containers:
      - command:
        - /bin/sh
        - -c
        - sleep 3600
        image: busybox:1.31.0
        imagePullPolicy: IfNotPresent
        name: busybox
        resources: {}
        terminationMessagePath: /dev/termination-log
        terminationMessagePolicy: File
      dnsPolicy: ClusterFirst
      restartPolicy: Always
      schedulerName: default-scheduler
      securityContext: {}
      terminationGracePeriodSeconds: 30
...
```

## 总结

介绍了通过命令的方式，删除 k8s 资源中存在的字段。

## 附录
