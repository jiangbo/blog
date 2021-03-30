# 【k8s】Deployment

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

之前学习过 rc 和 rs，其中 rs 比 rs 多出了集合类型的选择器。
而 Deployment 是做为一种更高级的抽象，Deployment 管理 rs，多出了升级相关的功能。
Deployment 可以简写为 deploy。

## 示例

### Deployment.yaml

```yaml
apiVersion: apps/v1
kind: Deployment
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

### 查看

```
[root@master ~]# kubectl get pod,rs,deploy
NAME                              READY   STATUS    RESTARTS   AGE
pod/spring-k8s-79f74b55d7-njw4b   1/1     Running   0          15s

NAME                                    DESIRED   CURRENT   READY   AGE
replicaset.apps/spring-k8s-79f74b55d7   1         1         1       15s

NAME                         READY   UP-TO-DATE   AVAILABLE   AGE
deployment.apps/spring-k8s   1/1     1            1           15s
[root@master ~]#

```

Deployment 创建了一个 rs 资源，而 rs 又创建 pod。根据它们的名字也可以看出之前的关系。
Deployment 可以管理多个 rs，rs 可以管理多个 Pod。
Deployment 所显示的字段有：

- `NAME` 列出了集群中 Deployment 的名称。
- `READY` 显示应用程序的可用的副本数。显示的模式是“就绪个数/期望个数”。
- `UP-TO-DATE` 显示为了达到期望状态已经更新的副本数。
- `AVAILABLE` 显示应用可供用户使用的副本数。
- `AGE` 显示应用程序运行的时间。

## 总结

Deployment 作为比 rs 更高一级的抽象，增加了升级相关的功能。

## 附录
