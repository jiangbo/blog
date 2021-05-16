# 【k8s】Volume-ConfigMap-file

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-RC1

## 目标

使用文件的方式创建 cm 然后进行挂载。

## 示例

### 创建 cm

```
[root@master ~]# kubectl create cm cm-file --from-file=config/
configmap/cm-file created
[root@master ~]# kubectl describe cm cm-file
Name:         cm-file
Namespace:    default
Labels:       <none>
Annotations:  <none>

Data
====
test.json:
----
{
  "name": "jiangbo",
  "age": 44
}

test.properties:
----
name=jiangbo
age=4444

Events:  <none>
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
        name: cm-file
```

### 查看

```
[root@master ~]# kubectl exec spring-k8s -- sh -c "ls -l /opt/cfg/"
total 0
lrwxrwxrwx    1 root     root            16 May 16 05:58 test.json -> ..data/test.json
lrwxrwxrwx    1 root     root            22 May 16 05:58 test.properties -> ..data/test.properties
```

### 挂载单个

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
        name: cm-file
        items:
          - key: test.properties
            path: test/1.properties
```

```
[root@master ~]# kubectl exec spring-k8s -- sh -c "ls -l /opt/cfg/test/"
total 4
-rw-r--r--    1 root     root            22 May 16 06:00 1.properties
[root@master ~]#
```

## 总结

使用文件的方式创建了 cm 的数据，然后挂载到 Pod 中，其中可以选择只挂载 cm 数据中的一部分 key。

## 附录
