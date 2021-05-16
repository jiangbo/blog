# 【k8s】Volume-Secret

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-RC1

## 目标

Secret 和 ConfigMap 类似，也可以将其以数据卷的方式挂载到 Pod 中，以下是示例。

## 示例

### 创建简单键值对 Secret

```
[root@master ~]# kubectl create secret generic mysecret --from-literal=name=jiangbo --from-literal=age=44
secret/mysecret created
[root@master ~]# kubectl describe secrets mysecret
Name:         mysecret
Namespace:    default
Labels:       <none>
Annotations:  <none>

Type:  Opaque

Data
====
age:   2 bytes
name:  7 bytes
```

### 绑定到 Pod

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
spec:
  containers:
    - name: spring-k8s
      image: jiangbo920827/spring-k8s:liveness
      volumeMounts:
        - mountPath: /opt/secret
          name: v1
      ports:
        - containerPort: 8080
  volumes:
    - name: v1
      secret:
        secretName: mysecret
```

### 查看

```
[root@master ~]# kubectl exec spring-k8s -- sh -c "ls -l /opt/secret"
total 0
lrwxrwxrwx    1 root     root            10 May 16 06:13 age -> ..data/age
lrwxrwxrwx    1 root     root            11 May 16 06:13 name -> ..data/name
```

### 自动 base64 解码

```
[root@master ~]# kubectl exec spring-k8s -- sh -c "cat /opt/secret/name"
jiangbo[root@master ~]#
```

## 总结

学习了使用 Secret 以数据卷的方式来使用，如果 Secret 被更新了，挂载的文件也会自动更新。

## 附录
