# 【k8s】ing-rules-host
## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-M3

## 目标

rules 规则下的 host 可以设置多个主机，如果不设置，默认为所以，显示成 * 号。
host 中也可以包含通配符。

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
[root@master ~]# cat /etc/hosts
127.0.0.1   localhost localhost.localdomain localhost4 localhost4.localdomain4
::1         localhost localhost.localdomain localhost6 localhost6.localdomain6
192.168.56.101 master spring.k8s.jiangbo master.k8s.jiangbo
192.168.56.102 node1
192.168.56.103 node2
```

### ing.yaml

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: spring-k8s
spec:
  rules:
    - host: "spring.k8s.jiangbo"
      http:
        paths:
          - path: /
            pathType: Prefix
            backend:
              service:
                name: spring-k8s
                port:
                  number: 80
    - host: "master.k8s.jiangbo"
      http:
        paths:
          - path: /
            pathType: Prefix
            backend:
              service:
                name: spring-k8s
                port:
                  number: 80
```

### 查看列表

```
[root@master ~]# kubectl get ing
NAME         CLASS    HOSTS                                   ADDRESS         PORTS   AGE
spring-k8s   <none>   spring.k8s.jiangbo,master.k8s.jiangbo   10.105.218.18   80      58m
```

### 访问测试

```
[root@master ~]# curl spring.k8s.jiangbo:31136/hostname;echo
spring-k8s-79f74b55d7-tjh7c
[root@master ~]# curl master.k8s.jiangbo:31136/hostname;echo
spring-k8s-79f74b55d7-tjh7c
```

## 总结

host 可以指定可以访问的域名，可以限制通过某些域名访问。

## 附录
