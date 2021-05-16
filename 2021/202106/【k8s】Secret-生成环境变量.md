# 【k8s】Secret-生成环境变量

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-M3

## 目标

将 Secret 中的配置用来生成 Pod 中的环境变量，如果 Secret 有修改，环境变量不会自动更新。
在使用时，会自动使用 base64 将值解码。

## 示例

### 准备 Secret

```
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

### 创建环境变量

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
spec:
  containers:
    - name: spring-k8s
      image: jiangbo920827/spring-k8s:liveness
      envFrom:
        - secretRef:
            name: mysecret
      ports:
        - containerPort: 8080
```

```
[root@master ~]# kubectl exec spring-k8s -- printenv
...
HOSTNAME=spring-k8s
age=44
name=jiangbo
...
```

### 创建带前缀的环境变量

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
spec:
  containers:
    - name: spring-k8s
      image: jiangbo920827/spring-k8s:liveness
      envFrom:
        - secretRef:
            name: mysecret
          prefix: SECRET_
      ports:
        - containerPort: 8080
```

```
[root@master ~]# kubectl exec spring-k8s -- sh -c "printenv | grep SECRET"
SECRET_name=jiangbo
SECRET_age=44
```

### 自定义环境变量 Key

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
spec:
  containers:
    - name: spring-k8s
      env:
        - name: FULL_NAME
          valueFrom:
            secretKeyRef:
              name: mysecret
              key: name
        - name: SECRET_AGE
          valueFrom:
            secretKeyRef:
              name: mysecret
              key: age
      image: jiangbo920827/spring-k8s:liveness
      ports:
        - containerPort: 8080
```

```
[root@master ~]# kubectl exec spring-k8s -- printenv
...
HOSTNAME=spring-k8s
FULL_NAME=jiangbo
SECRET_AGE=44
...
```

## 总结

Secret 可以使用来生成 Pod 的环境变量。

## 附录
