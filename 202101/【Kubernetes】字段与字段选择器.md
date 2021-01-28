# 【Kubernetes】字段与字段选择器

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

之前介绍了 Pod 有哪些字段，现在根据这些字段来选择不同的 Pod。

## 字段选择器

现在集群里 Pod 的状态如下，如果看过之前[容器的重启策略][1]，这些 Pod 的状态应该都是见过的。

```
[root@master pod]# kubectl get pod -o wide
NAME            READY   STATUS      RESTARTS   AGE     IP            NODE    NOMINATED NODE   READINESS GATES
pod-completed   0/1     Completed   0          95s     10.244.2.33   node2   <none>           <none>
pod-demo        1/1     Running     0          68m     10.244.1.63   node1   <none>           <none>
pod-demo1       1/1     Running     0          53m     10.244.1.64   node1   <none>           <none>
pod-error       0/1     Error       0          2m32s   10.244.1.65   node1   <none>           <none>
```

### 名称选择

名称是 metadata 大类下的一个字段，如果看过 [Pod 的组成字段][2]，应该好理解。
`kubectl get pod --field-selector metadata.name=pod-demo`

```
[root@master pod]# kubectl get pod --field-selector metadata.name=pod-demo
NAME       READY   STATUS    RESTARTS   AGE
pod-demo   1/1     Running   0          71m
```

### 状态选择

`kubectl get pod --field-selector status.phase=Failed`

```
[root@master pod]# kubectl get pod --field-selector status.phase=Failed
NAME        READY   STATUS   RESTARTS   AGE
pod-error   0/1     Error    0          21m
```

`kubectl get pod --field-selector status.phase="Succeeded"`

```
[root@master pod]# kubectl get pod --field-selector status.phase="Succeeded"
NAME            READY   STATUS      RESTARTS   AGE
pod-completed   0/1     Completed   0          23m
```

## 支持的操作符

### 等于

其中 = 和 == 都表示等于：

```
[root@master pod]# kubectl get pod --field-selector status.phase==Running
NAME        READY   STATUS    RESTARTS   AGE
pod-demo    1/1     Running   0          92m
pod-demo1   1/1     Running   0          77m
```

### 不等于

```
[root@master pod]# kubectl get pod --field-selector status.phase!=Running
NAME            READY   STATUS      RESTARTS   AGE
pod-completed   0/1     Completed   0          26m
pod-error       0/1     Error       0          27m
[root@master pod]#
```

## 组合选择

```
[root@master pod]# kubectl get pod  --field-selector=status.phase!=Running,metadata.name=pod-error
NAME        READY   STATUS   RESTARTS   AGE
pod-error   0/1     Error    0          32m
```

## 总结

介绍了字段选择器，并使用选择器选择我们需要的 Pod 进行展示。

[1]: https://www.cnblogs.com/jiangbo44/p/14332903.html
[2]: https://www.cnblogs.com/jiangbo44/p/14342283.html

## 附录
