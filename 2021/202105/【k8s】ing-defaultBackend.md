# 【k8s】ing-defaultBackend

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-M3

## 目标

defaultBackend 可以设置不满足任何规则的请求应该处理的方式，如果没有指定 Rules，则必须指定 defaultBackend。
如果 defaultBackend 没有设置，不满足任何规则的请求的处理方式，将会由 Ingress Controller 决定。

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
[root@master ~]#
```

### ing.yaml

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: spring-k8s
spec:
  defaultBackend:
    service:
      name: spring-k8s
      port:
        number: 80
```

### 查看列表

```
[root@master ~]# kubectl get ing
NAME         CLASS    HOSTS   ADDRESS         PORTS   AGE
spring-k8s   <none>   *       10.105.218.18   80      39m
```

### 访问测试

```
[root@master ~]# curl spring.k8s.jiangbo:31136/hostname;echo
spring-k8s-79f74b55d7-tjh7c
```

## 总结

defaultBackend 可以设置默认路由，如果不满足任何规则，则会走默认路由。

## 附录
