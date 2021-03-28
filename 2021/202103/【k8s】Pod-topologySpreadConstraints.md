# 【k8s】Pod-topologySpreadConstraints

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

topologySpreadConstraints 表示拓扑分布约束，可以控制 Pod 在某些节点的分布。

## 示例

### 设置地点分布

```
[root@master ~]# kubectl get nodes --show-labels
NAME     STATUS   ROLES                  AGE   VERSION   LABELS
master   Ready    control-plane,master   63d   v1.20.4   beta.kubernetes.io/arch=amd64,beta.kubernetes.io/os=linux,kubernetes.io/arch=amd64,kubernetes.io/hostname=master,kubernetes.io/os=linux,node-role.kubernetes.io/control-plane=,node-role.kubernetes.io/master=
node1    Ready    <none>                 63d   v1.20.4   beta.kubernetes.io/arch=amd64,beta.kubernetes.io/os=linux,kubernetes.io/arch=amd64,kubernetes.io/hostname=node1,kubernetes.io/os=linux
node2    Ready    <none>                 63d   v1.20.4   beta.kubernetes.io/arch=amd64,beta.kubernetes.io/os=linux,kubernetes.io/arch=amd64,kubernetes.io/hostname=node2,kubernetes.io/os=linux
[root@master ~]# kubectl label nodes node1 address=ChongQing
node/node1 labeled
[root@master ~]# kubectl label nodes node2 address=BeiJing
node/node2 labeled
[root@master ~]# kubectl get nodes --show-labels
NAME     STATUS   ROLES                  AGE   VERSION   LABELS
master   Ready    control-plane,master   63d   v1.20.4   beta.kubernetes.io/arch=amd64,beta.kubernetes.io/os=linux,kubernetes.io/arch=amd64,kubernetes.io/hostname=master,kubernetes.io/os=linux,node-role.kubernetes.io/control-plane=,node-role.kubernetes.io/master=
node1    Ready    <none>                 63d   v1.20.4   address=ChongQing,beta.kubernetes.io/arch=amd64,beta.kubernetes.io/os=linux,kubernetes.io/arch=amd64,kubernetes.io/hostname=node1,kubernetes.io/os=linux
node2    Ready    <none>                 63d   v1.20.4   address=BeiJing,beta.kubernetes.io/arch=amd64,beta.kubernetes.io/os=linux,kubernetes.io/arch=amd64,kubernetes.io/hostname=node2,kubernetes.io/os=linux
[root@master ~]#
```

### ReplicationController.yaml

```yaml
apiVersion: v1
kind: ReplicationController
metadata:
  name: spring-k8s
spec:
  replicas: 2
  template:
    metadata:
      labels:
        app: myapp
    spec:
      containers:
        - name: spring-k8s
          image: jiangbo920827/spring-k8s:liveness
          ports:
            - containerPort: 8080
```

### 查看

```
[root@master ~]# kubectl get pod -o wide
NAME               READY   STATUS    RESTARTS   AGE   IP             NODE    NOMINATED NODE   READINESS GATES
spring-k8s         1/1     Running   0          48m   10.244.2.172   node2   <none>           <none>
spring-k8s-4fkm7   1/1     Running   0          14s   10.244.2.176   node2   <none>           <none>
spring-k8s-r9c7x   1/1     Running   0          14s   10.244.2.175   node2   <none>           <none>
```

可以看到两个 Pod 分布到了同一个节点上。

### 分布约束

```yaml
apiVersion: v1
kind: ReplicationController
metadata:
  name: spring-k8s
spec:
  replicas: 2
  template:
    metadata:
      labels:
        app: myapp
    spec:
      containers:
        - name: spring-k8s
          image: jiangbo920827/spring-k8s:liveness
          ports:
            - containerPort: 8080
      topologySpreadConstraints:
        - topologyKey: address
          maxSkew: 1
          whenUnsatisfiable: DoNotSchedule
          labelSelector:
            matchLabels:
              app: myapp
```

topologyKey 表示按照什么标签进行区分；
whenUnsatisfiable 表示不满足时采取什么策略，DoNotSchedule 表示不调度，ScheduleAnyway 表示任然调度；
maxSkew 表示几个区域相差的最大值；
labelSelector 表示标签选择器。


### 查看分布约束

```
[root@master ~]# kubectl get pod -o wide
NAME               READY   STATUS    RESTARTS   AGE   IP             NODE    NOMINATED NODE   READINESS GATES
spring-k8s         1/1     Running   0          55m   10.244.2.172   node2   <none>           <none>
spring-k8s-89p7d   1/1     Running   0          20s   10.244.2.181   node2   <none>           <none>
spring-k8s-v4nqb   1/1     Running   0          20s   10.244.1.219   node1   <none>           <none>
```

## 总结

topologySpreadConstraints 可以在多个域之间平衡 Pod 数量。

## 附录
