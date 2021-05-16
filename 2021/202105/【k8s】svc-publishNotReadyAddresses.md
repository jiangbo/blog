# 【k8s】svc-publishNotReadyAddresses

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

publishNotReadyAddresses 表示是否将没有就绪的 Pod 的地址关联到服务上。
默认情况是 false，只有就绪状态的 Pod 的地址才会关联到服务上。

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
          readinessProbe:
            failureThreshold: 30
            exec:
              command: ["sh", "-c", "cat /root/test.txt"]
          ports:
            - containerPort: 8080
```

添加了一个就绪探针，因为这个文件不存在，所以 Pod 一直是未就绪，直到失败。

### 查看 Pod

```
[root@master ~]# kubectl get pod
NAME                         READY   STATUS    RESTARTS   AGE
spring-k8s-d8ccf5994-zbxzg   0/1     Running   0          4m16s
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

### 查看不发布就绪

```
[root@master ~]# kubectl describe svc spring-k8s
Name:              spring-k8s
Namespace:         default
Labels:            <none>
Annotations:       <none>
Selector:          app=spring-k8s
Type:              ClusterIP
IP Families:       <none>
IP:                10.104.121.41
IPs:               10.104.121.41
Port:              <unset>  80/TCP
TargetPort:        8080/TCP
Endpoints:
Session Affinity:  None
Events:            <none>
```

### 发布未就绪 Pod

```yaml
apiVersion: v1
kind: Service
metadata:
  name: spring-k8s
spec:
  selector:
    app: spring-k8s
  publishNotReadyAddresses: true
  ports:
    - port: 80
      targetPort: 8080
```

### 查看发布未就绪 Pod

```
[root@master ~]# kubectl describe svc spring-k8s
Name:              spring-k8s
Namespace:         default
Labels:            <none>
Annotations:       <none>
Selector:          app=spring-k8s
Type:              ClusterIP
IP Families:       <none>
IP:                10.104.121.41
IPs:               10.104.121.41
Port:              <unset>  80/TCP
TargetPort:        8080/TCP
Endpoints:         10.244.2.232:8080
Session Affinity:  None
Events:            <none>
[root@master ~]# kubectl get pod
NAME                         READY   STATUS    RESTARTS   AGE
spring-k8s-d8ccf5994-zbxzg   0/1     Running   0          7m16s
```

## 总结

publishNotReadyAddresses 可以控制是否将未就绪的 Pod 发布到 Service 上，一般不建议为 true。
如果 Pod 还未就绪的话，发送到上面的请求可能得不到正常的响应。

## 附录
