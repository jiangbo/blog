# 【k8s】svc-type-ClusterIP

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

k8s 支持的服务类型有四种，LoadBalancer 一般是云厂商提供，这里不介绍。分别是：
1. ClusterIP
2. NodePort
3. ExternalName
4. LoadBalancer

这里介绍 ClusterIP 类型，这个是默认的类型，表示集群 IP，只能在集群内部访问。
在 k8s 的 node 上也是可以访问的。

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
  selector:
    app: spring-k8s
  ports:
    - port: 80
      targetPort: 8080
```

### 查看

```
[root@master ~]# curl 10.104.121.41/hostname;echo
spring-k8s-79f74b55d7-d527c
[root@master ~]# kubectl get pod
NAME                          READY   STATUS    RESTARTS   AGE
spring-k8s-79f74b55d7-d527c   1/1     Running   0          71m
[root@master ~]#
```

## 总结

ClusterIP 类型表示集群内部 IP，不能被外部访问。

## 附录
