# 【Kubernetes】使用 Pod 提供服务

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

建立一个 Pod，可以像 Docker 容器一样启动，并向外提供服务。

## 使用命令行创建 Pod

### 命令行创建

```sh
kubectl run pod-shell --image=jiangbo920827/spring-docker:1.0.1
```

### 查看状态

```sh
[root@master pod]# kubectl get pod -o wide
NAME        READY   STATUS    RESTARTS   AGE    IP           NODE    NOMINATED NODE   READINESS GATES
pod-shell   1/1     Running   0          3m2s   10.244.2.4   node2   <none>           <none>
```

### 访问

```sh
[root@master pod]# curl 10.244.2.4:8080/hostname
pod-shell 1.0.1[root@master pod]#
```

### 删除

```sh
[root@master pod]# kubectl delete po pod-shell
pod "pod-shell" deleted
```

## 文件创建 Pod

### Pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: pod-yaml
spec:
  containers:
    - name: pod-yaml
      image: jiangbo920827/spring-docker:1.0.1
      ports:
        - containerPort: 8080
```

### 创建命令

```sh
[root@master pod]# kubectl apply -f pod.yaml
pod/pod-yaml created
```

### 查看 Pod 状态

```sh
[root@master pod]# kubectl get pod -o wide
NAME       READY   STATUS    RESTARTS   AGE   IP           NODE    NOMINATED NODE   READINESS GATES
pod-yaml   1/1     Running   0          53s   10.244.1.6   node1   <none>           <none>
```

### 访问测试

```sh
[root@master pod]# curl 10.244.1.6:8080/hostname
pod-yaml 1.0.1[root@master pod]#
```

### 删除 Pod

```sh
kubectl delete -f pod.yaml
```

## 总结

介绍了创建一个最简单的 Pod，并使用之前创建的镜像来启动服务。
之后在使用 Kubernetes 的过程中，创建资源以 yaml 文件为主，命令为辅。

## 附录