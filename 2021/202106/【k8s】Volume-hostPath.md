# 【k8s】Volume-hostPath

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-RC1

## 目标

hostPath 可以将宿主机的目录挂载到容器中，不过由于宿主机目录中的内容可能不一致，而导致容器的状态不一致。

## 示例

### Pod.yaml

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
        - mountPath: /opt/volume/hosts
          name: v1
      ports:
        - containerPort: 8080
  volumes:
    - name: v1
      hostPath:
        path: /etc/hosts
```

### 查看

```
[root@master ~]# kubectl exec spring-k8s -- sh -c "cat /opt/volume/hosts;echo"
127.0.0.1   localhost localhost.localdomain localhost4 localhost4.localdomain4
::1         localhost localhost.localdomain localhost6 localhost6.localdomain6
192.168.56.101 master
192.168.56.102 node1
192.168.56.103 node2
```

## 总结

hostPath 可以将宿主机和容器的目录共享。

## 附录
