# 【k8s】Volume-downwardAPI

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-RC1

## 目标

downwardAPI 可以获取 Pod 的相关信息，并且以文件的方式存入到数据卷中。
引入该功能的目的是为了和 k8s 的 api 解耦。

## 示例

### 获取 Pod 信息

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
        - mountPath: /opt/volume
          name: v1
      ports:
        - containerPort: 8080
  volumes:
    - name: v1
      downwardAPI:
        items:
          - path: name
            fieldRef:
              fieldPath: metadata.name
```

### 查看名称

```
[root@master ~]# kubectl exec spring-k8s -- sh -c "cat /opt/volume/name;echo"
spring-k8s
[root@master ~]#
```

### 获取容器信息

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
        - mountPath: /opt/volume
          name: v1
      ports:
        - containerPort: 8080
  volumes:
    - name: v1
      downwardAPI:
        items:
          - path: limit
            resourceFieldRef:
              resource: limits.memory
              containerName: spring-k8s
              divisor: 1Mi
```

```
[root@master ~]# kubectl exec spring-k8s -- sh -c "cat /opt/volume/limit;echo"
2768
```

## 总结

downwardAPI 可以将 Pod 或者容器的信息写入到环境变量或者数据卷中。

## 附录
