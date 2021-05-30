# 【k8s】Ingress

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-M3

## 目标

Service 已经可以提供外部访问的方式和负载均衡了，Ingress 也可以提供类似的功能。
两者最大的区别是 Service 工作在第四层 TCP/IP，而 Ingress 工作在第七层 HTTP。
Ingress 可以提供负载均衡、SSL 终结和基于名称的虚拟托管，可以简写为 ing。
在使用 Ingress 之前，一定要保证有 Ingress Controller。

## 示例

默认情况下，k8s 集群中不存在 Ingress 资源。

### 增加域名映射

```
[root@master ~]# cat /etc/hosts
127.0.0.1   localhost localhost.localdomain localhost4 localhost4.localdomain4
::1         localhost localhost.localdomain localhost6 localhost6.localdomain6
192.168.56.101 master spring.k8s.jiangbo
192.168.56.102 node1
192.168.56.103 node2
```

增加一个域名的映射，就可以通过域名访问到 k8s 集群的控制平面的 IP 地址。

### Deploy.yaml

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: spring-k8s
spec:
  selector:
    matchLabels:
      app: spring-k8s
  template:
    metadata:
      labels:
        app: spring-k8s
    spec:
      containers:
        - name: spring-k8s
          image: jiangbo920827/spring-k8s:liveness
          ports:
            - containerPort: 8080
```

### Service.yaml

```yaml
apiVersion: v1
kind: Service
metadata:
  name: spring-k8s
spec:
  selector:
    app: spring-k8s
  ports:
    - port: 80
      targetPort: 8080
```

### Ingress.yaml

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: spring-k8s
spec:
  rules:
    - host: spring.k8s.jiangbo
      http:
        paths:
          - pathType: Prefix
            path: "/"
            backend:
              service:
                name: spring-k8s
                port:
                  number: 80
```

### 验证

```
[root@master ~]# kubectl get svc -n ingress-nginx
NAME                                 TYPE        CLUSTER-IP      EXTERNAL-IP   PORT(S)                      AGE
ingress-nginx-controller             NodePort    10.105.218.18   <none>        80:31136/TCP,443:32152/TCP   10d
ingress-nginx-controller-admission   ClusterIP   10.106.32.133   <none>        443/TCP                      10d
[root@master ~]# kubectl get pod
NAME                          READY   STATUS    RESTARTS   AGE
spring-k8s-79f74b55d7-tjh7c   1/1     Running   2          10d
[root@master ~]# curl spring.k8s.jiangbo:31136/hostname;echo
spring-k8s-79f74b55d7-tjh7c
```

首先找到 Ingress Controller 的 NodePort 端口，然后通过域名和端口访问，就能进入集群。
再通过配置的 Ingress 路由规则进行转发。

### 查看

```
[root@master ~]# kubectl describe ing spring-k8s
Name:             spring-k8s
Namespace:        default
Address:          10.105.218.18
Default backend:  default-http-backend:80 (<error: endpoints "default-http-backend" not found>)
Rules:
  Host                Path  Backends
  ----                ----  --------
  spring.k8s.jiangbo
                      /   spring-k8s:80 (10.244.2.20:8080)
Annotations:          <none>
Events:
  Type    Reason  Age                    From                      Message
  ----    ------  ----                   ----                      -------
  Normal  Sync    2m42s (x2 over 3m36s)  nginx-ingress-controller  Scheduled for sync
```

## 总结

Ingress 除了可以提供 Service 的负载均衡之外，还可以配置域名地址映射。

## 附录
