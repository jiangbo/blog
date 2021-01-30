# 【Kubernetes】自动创建 Pod 集群

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

之前介绍了手动创建多副本的集群，现在将这个过程进行自动化，需要引入 k8s 中的另一个资源对象。
即副本控制器-replicationcontroller，简写 rc，Pod 的简写是 po。

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

template 字段下面的字段，应该很熟悉，就是我们之前学习的 Pod 的相关字段。
前面的三个字段是 k8s 资源的标准声明，和 Pod 一致。
就只有 spec 中的 replicas 和 selector 需要进行说明。

### replicas

副本数，相当于服务的个数，相当于之前集群中的机器的数量，这里设定的三个。

### selector

标签选择器，其中的冒号相当于之前学习的[标签选择器][1]里的相等。

## 查看状态

### 查看 Pod

```
[root@master kubernetes]# kubectl get po
NAME            READY   STATUS    RESTARTS   AGE
rc-demo-9hjh6   1/1     Running   0          30s
rc-demo-ddg8d   1/1     Running   0          30s
rc-demo-qxsf4   1/1     Running   0          30s
```

可以看到根据定义的文档，已准确生成了三个副本。
副本的名称是 rc 的名称加上随机字母和数字。

### 查看 rc

```
[root@master kubernetes]# kubectl get rc
NAME      DESIRED   CURRENT   READY   AGE
rc-demo   3         3         3       44s
```

可以看到 rc 记录了我们定义的副本数量，并且启动 Pod 来达到我们定义的数量。

### 查看 rc 详情

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
  Normal  SuccessfulCreate  13m   replication-controller  Created pod: rc-demo-qxsf4
  Normal  SuccessfulCreate  13m   replication-controller  Created pod: rc-demo-ddg8d
  Normal  SuccessfulCreate  13m   replication-controller  Created pod: rc-demo-9hjh6
```

## 总结

介绍了自动创建 Pod 集群，使用的是 k8s 的 rc，即副本控制器。正如其名，可以准确控制生成的副本数量。

[1]: https://www.cnblogs.com/jiangbo44/p/14347287.html

## 附录
