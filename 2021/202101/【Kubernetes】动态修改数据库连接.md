# 【Kubernetes】动态修改数据库连接

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

在之前[集群访问本地服务][1]中，打成镜像之前，我们还查看了 Service 的 IP 地址，然后修改配置。
如果 Service 的 IP 地址还不清楚，或者每个环境的 IP 都不一致，那就可以使用环境变量动态修改。

## 修改 Service IP

因为 Service 的 IP 分配了就不能再修改，所以我们直接删除，重新建立 Service。

### svc.yaml

```yaml
apiVersion: v1
kind: Service
metadata:
  name: svc-pg
spec:
  ports:
    - port: 5432

```

### ep.yaml

```yaml
apiVersion: v1
kind: Endpoints
metadata:
  name: svc-pg
subsets:
  - addresses:
      - ip: 192.168.56.103
    ports:
      - port: 5432
```

### 删除重建

删除 Service 的时候，默认把 Endpoints 也一起删除了，所以需要重新创建。

```
[root@master kubernetes]# kubectl delete -f svc.yaml
service "svc-pg" deleted
[root@master kubernetes]# kubectl apply -f svc.yaml
service/svc-pg created
[root@master kubernetes]# kubectl apply -f ep.yaml
endpoints/svc-pg created

```


### Service 前后对比

```
[root@master kubernetes]# kubectl get service svc-pg
NAME         TYPE           CLUSTER-IP       EXTERNAL-IP     PORT(S)          AGE
svc-pg       NodePort       10.109.50.111    <none>          5432:32206/TCP   15h

[root@master kubernetes]# kubectl get service svc-pg
NAME     TYPE        CLUSTER-IP      EXTERNAL-IP   PORT(S)    AGE
svc-pg   ClusterIP   10.108.222.50   <none>        5432/TCP   20s

```

可以看到 IP 地址已经变化了。

## 修改前访问测试

### 访问 hostname

```
[root@master kubernetes]# curl 10.244.1.105/hostname
rc-demo-r6nv8 external[root@master kubernetes]#
```

### 访问数据库

```
[root@master kubernetes]# curl 10.244.1.105/users | jq
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   119    0   119    0     0      3      0 --:--:--  0:00:30 --:--:--    29
{
  "timestamp": "2021-01-31T06:51:59.519+00:00",
  "status": 500,
  "error": "Internal Server Error",
  "message": "",
  "path": "/users"
}
```

已经访问不到数据库了，因为 Service 的 IP 已经变化。

## 修改数据库连接

### rc.yaml

```yaml
apiVersion: v1
kind: ReplicationController
metadata:
  name: rc-demo
spec:
  replicas: 3
  template:
    metadata:
      labels:
        app: myapp
    spec:
      containers:
        - name: pod-demo
          image: jiangbo920827/spring-demo:external
          ports:
            - containerPort: 80
          env:
            - name: SERVER_PORT
              value: "80"
            - name: SPRING_DATASOURCE_URL
              value: jdbc:postgresql://10.108.222.50:5432/postgres

```

### 修改 rc

```
[root@master kubernetes]# kubectl apply -f rc.yaml
replicationcontroller/rc-demo configured
```

### 重启 Pod

```
[root@master kubernetes]# kubectl delete pod -l app=myapp
pod "rc-demo-b7r9f" deleted
pod "rc-demo-b9mqr" deleted
pod "rc-demo-r6nv8" deleted
```

### 查看重启后状态

```
[root@master kubernetes]# kubectl get pod -o wide
NAME            READY   STATUS    RESTARTS   AGE     IP             NODE    NOMINATED NODE   READINESS GATES
rc-demo-8ld7l   1/1     Running   0          6m43s   10.244.1.107   node1   <none>           <none>
rc-demo-9fhdz   1/1     Running   0          6m43s   10.244.2.54    node2   <none>           <none>
rc-demo-f7dcd   1/1     Running   0          6m43s   10.244.2.55    node2   <none>           <none>
```

## 验证

### 访问 hostname

```
[root@master kubernetes]# curl 10.244.2.55/hostname
rc-demo-f7dcd external[root@master kubernetes]#
```

### 访问数据库值

```
[root@master kubernetes]# curl 10.244.2.55/users | jq
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100    29    0    29    0     0    190      0 --:--:-- --:--:-- --:--:--   192
[
  {
    "name": "jiangbo",
    "age": 44
  }
]
```

### 查看环境变量

通过环境变量列表，我们看到了我们注入的数据库的连接信息。但同时，我们也看了额外的 Service 信息。
原来 k8s 早就想到了这点，已经把建立的 Service 的信息放到环境变量了。

```
[root@master kubernetes]# kubectl exec rc-demo-f7dcd -- printenv | grep SPRING
SPRING_DATASOURCE_URL=jdbc:postgresql://10.108.222.50:5432/postgres
[root@master kubernetes]# kubectl exec rc-demo-f7dcd -- printenv
PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/lib/jvm/java-1.8-openjdk/jre/bin:/usr/lib/jvm/java-1.8-openjdk/bin
HOSTNAME=rc-demo-f7dcd
SPRING_DATASOURCE_URL=jdbc:postgresql://10.108.222.50:5432/postgres
SERVER_PORT=80
KUBERNETES_PORT_443_TCP=tcp://10.96.0.1:443
KUBERNETES_PORT_443_TCP_PORT=443
SVC_DEMO_PORT_8080_TCP_PORT=8080
SVC_PG_PORT_5432_TCP=tcp://10.108.222.50:5432
KUBERNETES_PORT=tcp://10.96.0.1:443
SVC_PG_PORT_5432_TCP_PORT=5432
KUBERNETES_SERVICE_HOST=10.96.0.1
KUBERNETES_SERVICE_PORT_HTTPS=443
KUBERNETES_PORT_443_TCP_PROTO=tcp
SVC_PG_PORT=tcp://10.108.222.50:5432
SVC_PG_PORT_5432_TCP_PROTO=tcp
KUBERNETES_SERVICE_PORT=443
SVC_DEMO_SERVICE_HOST=10.106.217.209
SVC_DEMO_SERVICE_PORT=8080
SVC_DEMO_PORT=tcp://10.106.217.209:8080
SVC_DEMO_PORT_8080_TCP_ADDR=10.106.217.209
SVC_PG_SERVICE_HOST=10.108.222.50
SVC_PG_SERVICE_PORT=5432
SVC_DEMO_PORT_8080_TCP=tcp://10.106.217.209:8080
SVC_DEMO_PORT_8080_TCP_PROTO=tcp
SVC_PG_PORT_5432_TCP_ADDR=10.108.222.50
KUBERNETES_PORT_443_TCP_ADDR=10.96.0.1
LANG=C.UTF-8
JAVA_HOME=/usr/lib/jvm/java-1.8-openjdk/jre
JAVA_VERSION=8u212
JAVA_ALPINE_VERSION=8.212.04-r0
HOME=/root
```

## 总结

通过给 Pod 注入数据库连接的环境变量，来达到动态修改数据库访问连接的目的。
同时也发现了 k8s 自动帮我们注入了 Service 中相关的信息。

[1]: https://www.cnblogs.com/jiangbo44/p/14351397.html

## 附录
