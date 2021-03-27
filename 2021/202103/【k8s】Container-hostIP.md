# 【k8s】Container-hostIP

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

将容器中的服务暴露到宿主机的端口上时，可以指定绑定的宿主机 IP。

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
      ports:
        - containerPort: 8080
          hostPort: 8081
          hostIP: 127.0.0.1
```

### 容器 IP 访问

```
[root@master ~]# curl 10.244.1.200:8080/hostname;echo
spring-k8s
```

### 宿主机 IP 访问

```
[root@master ~]# curl 192.168.56.102:8081/hostname;echo
curl: (7) Failed connect to 192.168.56.102:8081; Connection refused
```
可以看到通过宿主机的 IP 不能访问了。

### localhost 访问

在 192.168.56.102 主机上，通过 localhost 和 127.0.0.1 访问。

```
[root@node1 ~]# curl 127.0.0.1:8081/hostname;echo
spring-k8s
[root@node1 ~]# curl localhost:8081/hostname;echo
spring-k8s
[root@node1 ~]#
```

## 总结

将容器服务暴露到宿主机上时，可以指定需要绑定的 IP，这样可以限制通过其它 IP 访问。

## 附录
