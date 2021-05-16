# 【k8s】Volume-ConfigMap

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-RC1

## 目标

前面学习了将 ConfigMap 当作环境变量写入到 Pod 中，其实 ConfigMap 可以当作数据卷使用。

## 示例

### 创建简单键值对 cm

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: cm-simple
data:
  name: jiangbo
  age: "4444"
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
        - mountPath: /opt/cfg
          name: v1
      ports:
        - containerPort: 8080
  volumes:
    - name: v1
      configMap:
        name: cm-simple
```

### 查看

```
[root@master ~]# kubectl exec spring-k8s -- sh -c "ls -l /opt/cfg"
total 0
lrwxrwxrwx    1 root     root            10 May 16 05:39 age -> ..data/age
lrwxrwxrwx    1 root     root            11 May 16 05:39 name -> ..data/name
[root@master ~]# kubectl exec spring-k8s -- sh -c "cat  /opt/cfg/name"
jiangbo[root@master ~]# kubectl exec spring-k8s -- sh -c "cat  /opt/cfg/name;echo"
jiangbo
[root@master ~]#
```

普通键值对类型的 cm 会以文件的方式挂载到定义的目录下，其中文件名是定义的 key，而文件内容是 value。

### 修改 cm

```
[root@master ~]# kubectl get cm cm-simple -o yaml |sed 's/jiangbo/k8s/g' | kubectl apply -f -
configmap/cm-simple configured
[root@master ~]# kubectl exec spring-k8s -- sh -c "cat  /opt/cfg/name;echo"
k8s
```

如果看到文件中的内容没有变化，需要多等一会才会生效。如果修改了 cm 中的值，使用数据卷的方式，会自动更新。

## 总结

学习了使用 ConfigMap 以数据卷的方式来使用。

## 附录
