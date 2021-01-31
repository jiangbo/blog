# 【Kubernetes】手动创建外部服务

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

通过之前 Service 的探索，我们知道 Service 访问 Pod，其实是通过 Endpoints 访问。
那我们手动创建一个 Endpoints 来进行访问，就访问 www.baidu.com。

## 创建空的 Service

targetPort 不设置的话，默认和 port 一致。

```yaml
apiVersion: v1
kind: Service
metadata:
  name: svc-baidu
spec:
  ports:
    - port: 80
```

### 查看空 Service 状态

```
[root@master kubernetes]# kubectl describe -f svc.yaml
Name:              svc-baidu
Namespace:         default
Labels:            <none>
Annotations:       <none>
Selector:          <none>
Type:              ClusterIP
IP Families:       <none>
IP:                10.105.12.97
IPs:               10.105.12.97
Port:              <unset>  80/TCP
TargetPort:        80/TCP
Endpoints:         <none>
Session Affinity:  None
Events:            <none>
```

其中的 Endpoints 字段为 none，表示没有。

## 创建 Endpoints

### 查看百度的 IP

```
正在 Ping www.a.shifen.com [14.215.177.38] 具有 32 字节的数据:
来自 14.215.177.38 的回复: 字节=32 时间=28ms TTL=55
来自 14.215.177.38 的回复: 字节=32 时间=28ms TTL=55

14.215.177.38 的 Ping 统计信息:
    数据包: 已发送 = 2，已接收 = 2，丢失 = 0 (0% 丢失)，
往返行程的估计时间(以毫秒为单位):
    最短 = 28ms，最长 = 28ms，平均 = 28ms
```

### yaml

其中的 name 需要和建立的 Service 一致。

```yaml
apiVersion: v1
kind: Endpoints
metadata:
  name: svc-baidu
subsets:
  - addresses:
      - ip: 14.215.177.38
    ports:
      - port: 80
```

### 查看 Service 状态

```
[root@master kubernetes]# kubectl describe -f svc.yaml
Name:              svc-baidu
Namespace:         default
Labels:            <none>
Annotations:       <none>
Selector:          <none>
Type:              ClusterIP
IP Families:       <none>
IP:                10.105.12.97
IPs:               10.105.12.97
Port:              <unset>  80/TCP
TargetPort:        80/TCP
Endpoints:         14.215.177.38:80
Session Affinity:  None
Events:            <none>
```

## 访问测试

`curl 10.105.12.97`，不过由于网页安全原因，可能会出现其它的一些问题。如果能看到百度的一些关键字，则表示成功了。

## 总结

介绍了通过 Endpoints 和 Service 的组合，访问外部的一些网站。

## 附录
