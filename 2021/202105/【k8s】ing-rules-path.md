# 【k8s】ing-rules-host

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-M3

## 目标

rules 规则下的 path 可以设置路径，并且可以将不同的路径转换到不同的服务上，还可以截取部分路径进行转发。

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
```

### ing.yaml

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: spring-k8s
  annotations:
    nginx.ingress.kubernetes.io/rewrite-target: /$2
spec:
  rules:
    - http:
        paths:
          - path: /test1(/|$)(.*)
            pathType: Prefix
            backend:
              service:
                name: spring-k8s
                port:
                  number: 80
          - path: /test2(/|$)(.*)
            pathType: Prefix
            backend:
              service:
                name: spring-k8s
                port:
                  number: 80
```

### 查看

```
[root@master ~]# kubectl describe ing spring-k8s
Name:             spring-k8s
Namespace:        default
Address:          10.105.218.18
Default backend:  default-http-backend:80 (<error: endpoints "default-http-backend" not found>)
Rules:
  Host        Path  Backends
  ----        ----  --------
  *
              /test1(/|$)(.*)   spring-k8s:80 (10.244.2.20:8080)
              /test2(/|$)(.*)   spring-k8s:80 (10.244.2.20:8080)
Annotations:  nginx.ingress.kubernetes.io/rewrite-target: /$2
Events:
  Type    Reason  Age                   From                      Message
  ----    ------  ----                  ----                      -------
  Normal  Sync    2m18s (x15 over 80m)  nginx-ingress-controller  Scheduled for sync
```

### 访问测试

```
[root@master ~]# curl spring.k8s.jiangbo:31136/test2/hostname;echo
spring-k8s-79f74b55d7-tjh7c
[root@master ~]# curl spring.k8s.jiangbo:31136/test1/hostname;echo
spring-k8s-79f74b55d7-tjh7c
```

## 总结

path 可以指定路径对应的服务，并且可以将路径进行截取后转发。

## 附录
