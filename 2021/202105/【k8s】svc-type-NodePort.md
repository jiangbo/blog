# 【k8s】svc-type-NodePort

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

k8s 支持的服务类型有四种，分别是：
1. ClusterIP
2. NodePort
3. ExternalName
4. LoadBalancer

这里介绍 NodePort 类型，可以在每个节点上打开端口，供外部访问。
如果没有指定暴露的端口，会随机指定一个，如果指定了端口（如果合法的话），会使用指定的。

## 示例

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
  type: NodePort
  selector:
    app: spring-k8s
  ports:
    - port: 80
      targetPort: 8080
```

### 查看

```
[root@master ~]# kubectl describe svc spring-k8s
Name:                     spring-k8s
Namespace:                default
Labels:                   <none>
Annotations:              <none>
Selector:                 app=spring-k8s
Type:                     NodePort
IP Families:              <none>
IP:                       10.104.121.41
IPs:                      10.104.121.41
Port:                     <unset>  80/TCP
TargetPort:               8080/TCP
NodePort:                 <unset>  30159/TCP
Endpoints:                10.244.2.239:8080
Session Affinity:         None
External Traffic Policy:  Cluster
Events:                   <none>
[root@master ~]# curl http://192.168.56.103:30159/hostname;echo
spring-k8s-79f74b55d7-d527c
```

## 总结

NodePort 类型的服务表示 在每个 node 上打开一个端口供外部访问。

## 附录
