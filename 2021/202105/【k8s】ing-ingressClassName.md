# 【k8s】ing-ingressClassName

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-M3

## 目标

ingressClassName 可以指定选择的 Ingress Controller，使用名称选择，一般有多个控制器的时候使用。
之前部署 Nginx Ingress Controller 的名称是 nginx。

## 示例

### 前提

```
[root@master ~]# kubectl get deployments,pod,service
NAME                         READY   UP-TO-DATE   AVAILABLE   AGE
deployment.apps/spring-k8s   1/1     1            1           10d

NAME                              READY   STATUS    RESTARTS   AGE
pod/spring-k8s-79f74b55d7-tjh7c   1/1     Running   2          10d

NAME                 TYPE        CLUSTER-IP      EXTERNAL-IP   PORT(S)   AGE
service/kubernetes   ClusterIP   10.96.0.1       <none>        443/TCP   10d
service/spring-k8s   ClusterIP   10.105.119.22   <none>        80/TCP    10d
```

### ing.yaml

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: spring-k8s
spec:
  ingressClassName: nginx
  defaultBackend:
    service:
      name: spring-k8s
      port:
        number: 80
```

### 查看列表

```
[root@master ~]# kubectl get ing
NAME         CLASS   HOSTS   ADDRESS         PORTS   AGE
spring-k8s   nginx   *       10.105.218.18   80      47m
```

### 访问测试

```
[root@master ~]# curl spring.k8s.jiangbo:31136/hostname;echo
spring-k8s-79f74b55d7-tjh7c
```

### 不引用 nginx

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: spring-k8s
spec:
  ingressClassName: nginx1
  defaultBackend:
    service:
      name: spring-k8s
      port:
        number: 80
```

### 访问不通

```
[root@master ~]# kubectl get ing
NAME         CLASS    HOSTS   ADDRESS         PORTS   AGE
spring-k8s   nginx1   *       10.105.218.18   80      48m
[root@master ~]# curl spring.k8s.jiangbo:31136/hostname;echo
<html>
<head><title>404 Not Found</title></head>
<body>
<center><h1>404 Not Found</h1></center>
<hr><center>nginx</center>
</body>
</html>
```

## 总结

ingressClassName 可以指定选择的  Ingress Controller。

## 附录
