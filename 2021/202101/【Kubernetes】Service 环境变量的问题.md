# 【Kubernetes】Service 环境变量的问题

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

在前面，我们通过 Service 提供的环境变量，只要知道了服务的名称，就可以访问到该服务，但是这样还存在一个问题。
就是如果 Pod 在 Service 之前创建，那么 Service 的信息是不会自动注入进去的。

## 问题复现

### 查看当前 Pod

```
[root@master kubernetes]# kubectl get pod -o wide
NAME            READY   STATUS    RESTARTS   AGE   IP             NODE    NOMINATED NODE   READINESS GATES
rc-demo-46dq7   1/1     Running   0          15s   10.244.1.113   node1   <none>           <none>
rc-demo-dxvlv   1/1     Running   0          15s   10.244.1.111   node1   <none>           <none>
rc-demo-j48b8   1/1     Running   0          15s   10.244.1.112   node1   <none>           <none>
```

### 查看全部环境变量

```
[root@master kubernetes]# kubectl exec rc-demo-j48b8 -- printenv
PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/lib/jvm/java-1.8-openjdk/jre/bin:/usr/lib/jvm/java-1.8-openjdk/bin
HOSTNAME=rc-demo-j48b8
SPRING_DATASOURCE_URL=jdbc:postgresql://${SVC_PG_SERVICE_HOST}:${SVC_PG_SERVICE_PORT}/postgres
SERVER_PORT=80
KUBERNETES_SERVICE_HOST=10.96.0.1
KUBERNETES_PORT_443_TCP=tcp://10.96.0.1:443
KUBERNETES_SERVICE_PORT=443
KUBERNETES_SERVICE_PORT_HTTPS=443
SVC_PG_SERVICE_PORT=5432
SVC_PG_PORT_5432_TCP=tcp://10.108.222.50:5432
SVC_PG_PORT_5432_TCP_PROTO=tcp
SVC_PG_PORT_5432_TCP_ADDR=10.108.222.50
KUBERNETES_PORT=tcp://10.96.0.1:443
KUBERNETES_PORT_443_TCP_PROTO=tcp
KUBERNETES_PORT_443_TCP_PORT=443
KUBERNETES_PORT_443_TCP_ADDR=10.96.0.1
SVC_PG_SERVICE_HOST=10.108.222.50
SVC_PG_PORT=tcp://10.108.222.50:5432
SVC_PG_PORT_5432_TCP_PORT=5432
LANG=C.UTF-8
JAVA_HOME=/usr/lib/jvm/java-1.8-openjdk/jre
JAVA_VERSION=8u212
JAVA_ALPINE_VERSION=8.212.04-r0
HOME=/root
```

## 新建 Service

### svc.yaml

```yaml
apiVersion: v1
kind: Service
metadata:
  name: svc-demo
spec:
  selector:
    app: myapp
  ports:
    - port: 5432

```

### 查看重启后状态

```
[root@master kubernetes]# kubectl apply -f svc.yaml
service/svc-demo created
[root@master kubernetes]# kubectl get service
NAME         TYPE           CLUSTER-IP       EXTERNAL-IP     PORT(S)    AGE
kubernetes   ClusterIP      10.96.0.1        <none>          443/TCP    7d
svc-demo     ClusterIP      10.104.212.163   <none>          5432/TCP   4s
svc-pg       ClusterIP      10.108.222.50    <none>          5432/TCP   69m
```

### 再次查看环境变量

可以看到 Service svc-demo 的环境变量并没有注入进去，需要重启 Pod 才可以。

```
[root@master kubernetes]# kubectl exec rc-demo-j48b8 -- printenv
PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/lib/jvm/java-1.8-openjdk/jre/bin:/usr/lib/jvm/java-1.8-openjdk/bin
HOSTNAME=rc-demo-j48b8
SPRING_DATASOURCE_URL=jdbc:postgresql://${SVC_PG_SERVICE_HOST}:${SVC_PG_SERVICE_PORT}/postgres
SERVER_PORT=80
KUBERNETES_SERVICE_HOST=10.96.0.1
KUBERNETES_PORT_443_TCP=tcp://10.96.0.1:443
KUBERNETES_SERVICE_PORT=443
KUBERNETES_SERVICE_PORT_HTTPS=443
SVC_PG_SERVICE_PORT=5432
SVC_PG_PORT_5432_TCP=tcp://10.108.222.50:5432
SVC_PG_PORT_5432_TCP_PROTO=tcp
SVC_PG_PORT_5432_TCP_ADDR=10.108.222.50
KUBERNETES_PORT=tcp://10.96.0.1:443
KUBERNETES_PORT_443_TCP_PROTO=tcp
KUBERNETES_PORT_443_TCP_PORT=443
KUBERNETES_PORT_443_TCP_ADDR=10.96.0.1
SVC_PG_SERVICE_HOST=10.108.222.50
SVC_PG_PORT=tcp://10.108.222.50:5432
SVC_PG_PORT_5432_TCP_PORT=5432
LANG=C.UTF-8
JAVA_HOME=/usr/lib/jvm/java-1.8-openjdk/jre
JAVA_VERSION=8u212
JAVA_ALPINE_VERSION=8.212.04-r0
HOME=/root
```

## 总结

虽然通过 Service 的环境变量访问服务很方便，知道名字就可以。但是会存在 Pod 比 Service 先启动，不会注入的问题。
同时修改的时候，也不会更新现有的 Pod。所以通过环境变量的方式访问服务，可靠性方面得不到保证。

## 附录
