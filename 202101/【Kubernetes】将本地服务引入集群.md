# 【Kubernetes】将本地服务引入集群

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

在本地，我们通过 Docker 启动 PG 数据库，使用 Spring Boot 项目已能正确访问。
使用前面学习的 Endpoints 端点，将 Docker 建立的 PG 数据库引入到 k8s 集群中。

## 创建空的 Service

### svc.yaml

```yaml
apiVersion: v1
kind: Service
metadata:
  name: svc-pg
spec:
  type: NodePort
  ports:
    - port: 5432
```

### 查看 svc

```
[root@master kubernetes]# kubectl describe -f svc.yaml
Name:                     svc-pg
Namespace:                default
Labels:                   <none>
Annotations:              <none>
Selector:                 <none>
Type:                     NodePort
IP Families:              <none>
IP:                       10.109.50.111
IPs:                      10.109.50.111
Port:                     <unset>  5432/TCP
TargetPort:               5432/TCP
NodePort:                 <unset>  32206/TCP
Endpoints:                <none>
Session Affinity:         None
External Traffic Policy:  Cluster
Events:                   <none>
```

可以看到 NodePort 端口是 32206。

## 创建 Endpoints

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

### 查看 ep 状态

```
[root@master kubernetes]# kubectl describe -f ep.yaml
Name:         svc-pg
Namespace:    default
Labels:       <none>
Annotations:  <none>
Subsets:
  Addresses:          192.168.56.103
  NotReadyAddresses:  <none>
  Ports:
    Name     Port  Protocol
    ----     ----  --------
    <unset>  5432  TCP

Events:  <none>
```

### 查看 svc 状态

```
[root@master kubernetes]# kubectl describe -f svc.yaml
Name:                     svc-pg
Namespace:                default
Labels:                   <none>
Annotations:              <none>
Selector:                 <none>
Type:                     NodePort
IP Families:              <none>
IP:                       10.109.50.111
IPs:                      10.109.50.111
Port:                     <unset>  5432/TCP
TargetPort:               5432/TCP
NodePort:                 <unset>  32206/TCP
Endpoints:                192.168.56.103:5432
Session Affinity:         None
External Traffic Policy:  Cluster
Events:                   <none>
```

与之前相比，Endpoints 有数据了。

## 本地连接

### 修改配置

因为是 NodePort 暴露的，所以 IP 地址可以选择 101，102 或者 103 都行。

```yaml
management:
  endpoint:
    shutdown:
      enabled: true
  endpoints:
    web:
      exposure:
        include: "*"
        
spring:
  datasource:
    url: jdbc:postgresql://192.168.56.101:32206/postgres
    username: postgres
    password: 123456
```

### 访问测试

```
PS D:\workspace\sts\spring-demo> curl localhost:8080/users
[{"name":"jiangbo","age":44}]
```

## 总结

介绍了通过创建外部服务的方式暴露 PG 数据库，并使用 NodePort 暴露给本地服务使用。

## 附录
