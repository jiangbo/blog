# 【Kubernetes】标签与标签选择器

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

之前介绍了字段选择器，字段选择器不太灵活，只能选择 Kubernetes 定义好的字段。而标签选择器更灵活，可以自己定义。

## 标签选择器

### 标签的定义

标签是一个键值对，其中键可以由两部分组合，前缀和名称。

### 定义一个Pod

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: pod-demo

spec:
  containers:
    - name: pod-demo
      image: jiangbo920827/spring-demo:actuator
      ports:
        - containerPort: 8080
      resources:
        limits:
          memory: 200Mi
          cpu: 200m

```

### 查看标签

除了之前学习的使用 get 和 describe 命令查看 Pod 的详细信息时，会列出所有的标签，还可以使用下面的方式。

`kubectl get pod --show-labels`

```
[root@master pod]# kubectl get pod --show-labels
NAME       READY   STATUS    RESTARTS   AGE   LABELS
pod-demo   1/1     Running   0          11s   <none>
```

## 新增标签

### 新增普通标签

`kubectl label pod pod-demo env=study`

```
[root@master pod]# kubectl label pod pod-demo env=study
pod/pod-demo labeled
```

### 新增带前缀的标签

`kubectl label pod pod-demo jiangbo.study/env=study`

```
[root@master pod]# kubectl label pod pod-demo jiangbo.study/env=study
pod/pod-demo labeled
[root@master pod]# kubectl get pod --show-labels
NAME       READY   STATUS    RESTARTS   AGE    LABELS
pod-demo   1/1     Running   0          114s   env=study,jiangbo.study/env=study
```

### yaml 建立

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: pod-label
  labels:
    env: test
    jiangbo.study/version: stable
spec:
  containers:
    - name: pod-label
      image: jiangbo920827/spring-demo:actuator
      ports:
        - containerPort: 8080
      resources:
        limits:
          memory: 200Mi
          cpu: 200m

```

```
[root@master pod]# kubectl get pod --show-labels
NAME        READY   STATUS    RESTARTS   AGE     LABELS
pod-demo    1/1     Running   0          7m53s   env=study,jiangbo.study/env=study
pod-label   1/1     Running   0          20s     env=test,jiangbo.study/version=stable
```

## 支持的操作符

### 存在

存在指定的标签键

```
[root@master pod]# kubectl get pod --show-labels -l env
NAME        READY   STATUS    RESTARTS   AGE     LABELS
pod-demo    1/1     Running   0          9m33s   env=study,jiangbo.study/env=study
pod-label   1/1     Running   0          2m      env=test,jiangbo.study/version=stable
```

### 等于

```
[root@master pod]# kubectl get pod --show-labels -l env=test
NAME        READY   STATUS    RESTARTS   AGE     LABELS
pod-label   1/1     Running   0          2m40s   env=test,jiangbo.study/version=stable
```

### 不等于

```
[root@master pod]# kubectl get pod --show-labels -l env!=test
NAME       READY   STATUS    RESTARTS   AGE   LABELS
pod-demo   1/1     Running   0          12m   env=study,jiangbo.study/env=study
```

### 含有

```
[root@master pod]# kubectl get pod --show-labels -l 'env in (test,pro)'
NAME        READY   STATUS    RESTARTS   AGE     LABELS
pod-label   1/1     Running   0          5m37s   env=test,jiangbo.study/version=stable
```

### 不含有

```
[root@master pod]# kubectl get pod --show-labels -l 'env notin (test,pro)'
NAME       READY   STATUS    RESTARTS   AGE   LABELS
pod-demo   1/1     Running   0          13m   env=study,jiangbo.study/env=study
[root@master pod]#
```

### 组合选择

```
[root@master pod]# kubectl get pod --show-labels -l 'jiangbo.study/env,env notin (test,pro)'
NAME       READY   STATUS    RESTARTS   AGE   LABELS
pod-demo   1/1     Running   0          15m   env=study,jiangbo.study/env=study
```

## 标签说明

自动化系统添加的标签的，应该含有系统的前缀，Kubernetes 给资源添加的标签，含有 kubernetes.io 或者 k8s.io 前缀。
没有前缀的标签，一般表示用户自己添加的，是私有的。

### 查看 Node 的标签

```
[root@master pod]# kubectl get nodes --show-labels
NAME     STATUS   ROLES                  AGE    VERSION   LABELS
master   Ready    control-plane,master   5d6h   v1.20.2   beta.kubernetes.io/arch=amd64,beta.kubernetes.io/os=linux,kubernetes.io/arch=amd64,kubernetes.io/hostname=master,kubernetes.io/os=linux,node-role.kubernetes.io/control-plane=,node-role.kubernetes.io/master=
node1    Ready    <none>                 5d6h   v1.20.2   beta.kubernetes.io/arch=amd64,beta.kubernetes.io/os=linux,kubernetes.io/arch=amd64,kubernetes.io/hostname=node1,kubernetes.io/os=linux
node2    Ready    <none>                 5d6h   v1.20.2   beta.kubernetes.io/arch=amd64,beta.kubernetes.io/os=linux,kubernetes.io/arch=amd64,kubernetes.io/hostname=node2,kubernetes.io/os=linux

```

## 总结

介绍了标签的添加，以及标签选择器的使用。相比较字段选择器，更加的灵活。

## 附录
