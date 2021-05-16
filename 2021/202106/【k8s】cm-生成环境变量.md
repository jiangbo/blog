# 【k8s】cm-生成环境变量

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-RC1

## 目标

将 cm 中的配置信息，设置到 Pod 的环境变量中，如果 cm 有修改，环境变量中的值并不会同步修改，除非重启。

## 示例

### 准备 cm

```
[root@master ~]# kubectl create cm cm1 --from-literal=name=jiangbo --from-literal=age=44
configmap/cm1 created
[root@master ~]# kubectl describe cm cm1
Name:         cm1
Namespace:    default
Labels:       <none>
Annotations:  <none>

Data
====
age:
----
44
name:
----
jiangbo
Events:  <none>
```

### 直接生成

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
spec:
  containers:
    - name: spring-k8s
      envFrom:
        - configMapRef:
            name: cm1
      image: jiangbo920827/spring-k8s:liveness
      ports:
        - containerPort: 8080
```

```
[root@master ~]# kubectl exec spring-k8s -- sh -c 'printenv name age'
jiangbo
44
```

### 增加统一前缀

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
spec:
  containers:
    - name: spring-k8s
      envFrom:
        - prefix: ENV_
          configMapRef:
            name: cm1
      image: jiangbo920827/spring-k8s:liveness
      ports:
        - containerPort: 8080
```

```
[root@master ~]# kubectl exec spring-k8s -- sh -c 'env | grep ENV'
ENV_name=jiangbo
ENV_age=44
```

### 自定义 KEY

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
spec:
  containers:
    - name: spring-k8s
      env:
        - name: CHINESE_NAME
          valueFrom:
            configMapKeyRef:
              name: cm1
              key: name
        - name: ENV_AGE
          valueFrom:
            configMapKeyRef:
              name: cm1
              key: age
      image: jiangbo920827/spring-k8s:liveness
      ports:
        - containerPort: 8080
```

```
[root@master ~]# kubectl exec spring-k8s -- env
PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/lib/jvm/java-1.8-openjdk/jre/bin:/usr/lib/jvm/java-1.8-openjdk/bin
HOSTNAME=spring-k8s
CHINESE_NAME=jiangbo
ENV_AGE=44
KUBERNETES_SERVICE_HOST=10.96.0.1
KUBERNETES_SERVICE_PORT=443
KUBERNETES_SERVICE_PORT_HTTPS=443
KUBERNETES_PORT=tcp://10.96.0.1:443
KUBERNETES_PORT_443_TCP=tcp://10.96.0.1:443
KUBERNETES_PORT_443_TCP_PROTO=tcp
KUBERNETES_PORT_443_TCP_PORT=443
KUBERNETES_PORT_443_TCP_ADDR=10.96.0.1
LANG=C.UTF-8
JAVA_HOME=/usr/lib/jvm/java-1.8-openjdk/jre
JAVA_VERSION=8u212
JAVA_ALPINE_VERSION=8.212.04-r0
HOME=/root
```

## 总结

使用 cm 生成环境变量，一旦生成就不可以修改。

## 附录
