# 【Kubernetes】Pod 注入环境变量

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

在 Spring Boot 中，环境变量的值的优先级比配置文件中的高，所以我们给 Pod 添加环境变量，
来改变 Spring Boot 值，这里以修改端口号为例，从 8080 修改到 80。

## 增加环境变量

### rc.yaml

```yaml
apiVersion: v1
kind: ReplicationController
metadata:
  name: rc-demo
spec:
  replicas: 3
  template:
    metadata:
      labels:
        app: myapp
    spec:
      containers:
        - name: pod-demo
          image: jiangbo920827/spring-demo:external
          ports:
            - containerPort: 80
          env:
            - name: SERVER_PORT
              value: "80"

```

### 查看 Pod 状态

```
[root@master kubernetes]# kubectl get pods -o wide
NAME            READY   STATUS    RESTARTS   AGE   IP             NODE    NOMINATED NODE   READINESS GATES
rc-demo-b7r9f   1/1     Running   0          76s   10.244.1.106   node1   <none>           <none>
rc-demo-b9mqr   1/1     Running   0          76s   10.244.1.104   node1   <none>           <none>
rc-demo-r6nv8   1/1     Running   0          76s   10.244.1.105   node1   <none>           <none>
```

## 验证

### 访问 hostname

```
[root@master kubernetes]# curl 10.244.1.105/hostname
rc-demo-r6nv8 external[root@master kubernetes]#
```

### 访问数据库值

```
[root@master kubernetes]# curl 10.244.1.105/users
[{"name":"jiangbo","age":44}][root@master kubernetes]#
```

### 查看环境变量

```
[root@master kubernetes]# kubectl exec rc-demo-r6nv8 -- printenv | grep SERVER
SERVER_PORT=80
```

## 总结

通过给 Pod 注入环境变量，修改 Spring Boot 项目中的配置，来达到动态配置的目的。

## 附录
