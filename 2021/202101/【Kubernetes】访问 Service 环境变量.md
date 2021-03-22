# 【Kubernetes】访问 Service 环境变量

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

在上一节，我们看到：k8s 已经自动把 Service 的信息放到了环境变量里。我们通过访问已有的变量来修改数据库的连接信息。

## Service 的环境变量

### 查看变量

```
[root@master kubernetes]# kubectl exec rc-demo-f7dcd -- printenv | grep SVC_PG
SVC_PG_PORT_5432_TCP=tcp://10.108.222.50:5432
SVC_PG_PORT_5432_TCP_PORT=5432
SVC_PG_PORT=tcp://10.108.222.50:5432
SVC_PG_PORT_5432_TCP_PROTO=tcp
SVC_PG_SERVICE_HOST=10.108.222.50
SVC_PG_SERVICE_PORT=5432
SVC_PG_PORT_5432_TCP_ADDR=10.108.222.50
```

Service 变量的命名都是以 Service 的名称大写加上下划线组成的。

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
              value: jdbc:postgresql://${SVC_PG_SERVICE_HOST}:${SVC_PG_SERVICE_PORT}/postgres

```

然后修改 rc，并重启 Pod。

### 查看重启后状态

```
[root@master kubernetes]# kubectl get pod -o wide
NAME            READY   STATUS    RESTARTS   AGE   IP            NODE    NOMINATED NODE   READINESS GATES
rc-demo-6tcdv   1/1     Running   0          80s   10.244.2.56   node2   <none>           <none>
rc-demo-f9xzs   1/1     Running   0          80s   10.244.2.57   node2   <none>           <none>
rc-demo-tq4kb   1/1     Running   0          80s   10.244.2.58   node2   <none>           <none>
```

## 验证

### 访问接口

```
[root@master kubernetes]# curl 10.244.2.58/hostname
rc-demo-tq4kb external[root@master kubernetes]# curl 10.244.2.58/users | jq
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100    29    0    29    0     0    132      0 --:--:-- --:--:-- --:--:--   133
[
  {
    "name": "jiangbo",
    "age": 44
  }
]
```

### 查看环境变量

```
[root@master kubernetes]# kubectl exec rc-demo-tq4kb -- printenv
PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/lib/jvm/java-1.8-openjdk/jre/bin:/usr/lib/jvm/java-1.8-openjdk/bin
HOSTNAME=rc-demo-tq4kb
SERVER_PORT=80
SPRING_DATASOURCE_URL=jdbc:postgresql://${SVC_PG_SERVICE_HOST}:${SVC_PG_SERVICE_PORT}/postgres
SVC_DEMO_PORT=tcp://10.106.217.209:8080
SVC_PG_SERVICE_PORT=5432
KUBERNETES_SERVICE_HOST=10.96.0.1
KUBERNETES_SERVICE_PORT=443
KUBERNETES_PORT=tcp://10.96.0.1:443
KUBERNETES_PORT_443_TCP=tcp://10.96.0.1:443
KUBERNETES_PORT_443_TCP_PROTO=tcp
KUBERNETES_PORT_443_TCP_PORT=443
KUBERNETES_PORT_443_TCP_ADDR=10.96.0.1
SVC_DEMO_PORT_8080_TCP_PROTO=tcp
SVC_PG_PORT_5432_TCP_PORT=5432
SVC_DEMO_PORT_8080_TCP=tcp://10.106.217.209:8080
SVC_PG_SERVICE_HOST=10.108.222.50
SVC_PG_PORT_5432_TCP_PROTO=tcp
SVC_PG_PORT=tcp://10.108.222.50:5432
SVC_PG_PORT_5432_TCP=tcp://10.108.222.50:5432
SVC_PG_PORT_5432_TCP_ADDR=10.108.222.50
KUBERNETES_SERVICE_PORT_HTTPS=443
SVC_DEMO_SERVICE_HOST=10.106.217.209
SVC_DEMO_SERVICE_PORT=8080
SVC_DEMO_PORT_8080_TCP_PORT=8080
SVC_DEMO_PORT_8080_TCP_ADDR=10.106.217.209
LANG=C.UTF-8
JAVA_HOME=/usr/lib/jvm/java-1.8-openjdk/jre
JAVA_VERSION=8u212
JAVA_ALPINE_VERSION=8.212.04-r0
HOME=/root
```

## 总结

因为 Service 自动注入了相关的信息，并且命名遵循一定的规律。所以如果我们知道任何服务的名称，就可以访问到该服务。
这个和微服务的概念不谋而合，所以 k8s 很适合微服务，和微服务中的服务注册与发现很相似。

## 附录
