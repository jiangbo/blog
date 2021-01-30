# 【Kubernetes】副本控制器

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

前面我们使用 rc 自动创建了一个 Pod 集群，这里再简单理解副本控制器的原理。

## 创建 rc

### rc.yaml

```yaml
apiVersion: v1
kind: ReplicationController
metadata:
  name: rc-demo
spec:
  replicas: 3
  selector:
    app: myapp
  template:
    metadata:
      labels:
        app: myapp
    spec:
      containers:
        - name: pod-demo
          image: jiangbo920827/spring-demo:actuator
          ports:
            - containerPort: 8080

```

### 查看 Pod

```
[root@master kubernetes]# kubectl get po
NAME            READY   STATUS    RESTARTS   AGE
rc-demo-9hjh6   1/1     Running   0          30s
rc-demo-ddg8d   1/1     Running   0          30s
rc-demo-qxsf4   1/1     Running   0          30s
```

### 查看 rc

```
[root@master kubernetes]# kubectl get rc
NAME      DESIRED   CURRENT   READY   AGE
rc-demo   3         3         3       44s
```

## 创建 Pod

rc 是根据 yaml 中的 template 字段来创建 Pod。有时候，我们也称这个为 PodTemplate，即 Pod 模板。
经过与之前我们创建 Pod 的 yaml 相比，就缺少了类型，版本，名称字段。

## Pod 数量

还有一个问题是，rc 怎么知道创建了多少个 Pod，还需要创建几个呢？这个是通过之前的[标签与标签选择器][1]来实现的。

### 查看 Pod 的标签

可以看到，rc 生成的每个 Pod 都有我们定义在 Pod 模板中的标签。

```
NAME            READY   STATUS    RESTARTS   AGE   LABELS
rc-demo-9hjh6   1/1     Running   1          14h   app=myapp
rc-demo-ddg8d   1/1     Running   1          14h   app=myapp
rc-demo-qxsf4   1/1     Running   1          14h   app=myapp
```

### 查看标签选择器

Selector 字段表示这个是一个标签选择器，选择的标签是 app=myapp，和我们 Pod 的标签对应。
通过标签，rc 就能知道当前有几个 Pod 在运行，还需要创建几个。

```
[root@master kubernetes]# kubectl describe -f rc.yaml
Name:         rc-demo
Namespace:    default
Selector:     app=myapp
Labels:       app=myapp
Annotations:  <none>
Replicas:     3 current / 3 desired
Pods Status:  3 Running / 0 Waiting / 0 Succeeded / 0 Failed
Pod Template:
  Labels:  app=myapp
  Containers:
   pod-demo:
    Image:        jiangbo920827/spring-demo:actuator
    Port:         8080/TCP
    Host Port:    0/TCP
    Environment:  <none>
    Mounts:       <none>
  Volumes:        <none>
Events:
  Type    Reason            Age   From                    Message
  ----    ------            ----  ----                    -------
  Normal  SuccessfulCreate  14h   replication-controller  Created pod: rc-demo-qxsf4
  Normal  SuccessfulCreate  14h   replication-controller  Created pod: rc-demo-ddg8d
  Normal  SuccessfulCreate  14h   replication-controller  Created pod: rc-demo-9hjh6
```

### 验证

既然是通过标签选择器来统计的 Pod 数量，那么修改其中一个 Pod 的标签，应该会继续创建满足条件的标签。

```
[root@master kubernetes]# kubectl get pod --show-labels
NAME            READY   STATUS    RESTARTS   AGE   LABELS
rc-demo-9hjh6   1/1     Running   1          14h   app=myapp
rc-demo-ddg8d   1/1     Running   1          14h   app=myapp
rc-demo-qxsf4   1/1     Running   1          14h   app=myapp
[root@master kubernetes]# kubectl label pod rc-demo-9hjh6 app=yourapp --overwrite
pod/rc-demo-9hjh6 labeled
[root@master kubernetes]# kubectl get pod --show-labels
NAME            READY   STATUS    RESTARTS   AGE   LABELS
rc-demo-9hjh6   1/1     Running   1          14h   app=yourapp
rc-demo-ddg8d   1/1     Running   1          14h   app=myapp
rc-demo-h4vvc   1/1     Running   0          9s    app=myapp
rc-demo-qxsf4   1/1     Running   1          14h   app=myapp
```

## 总结

介绍了副本控制器控制 Pod 数量的原理。

[1]: https://www.cnblogs.com/jiangbo44/p/14347287.html

## 附录
