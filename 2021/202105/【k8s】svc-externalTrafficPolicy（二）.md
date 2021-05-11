# 【k8s】svc-externalTrafficPolicy（二）

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-M3

## 目标

Service 的 externalTrafficPolicy 表示外部流量策略，主要的作用是设置是否保留源 IP。
如果是 NodePort 类型的 Service 默认不会保留源 IP，可以通过 externalTrafficPolicy 来设置。

## 示例

以下的流程基于前一篇的流程继续进行。

### 创建 NodePort

`kubectl expose deployment source-ip-app --name=nodeport --port=80 --target-port=8080 --type=NodePort`

```
[root@master ~]# kubectl expose deployment source-ip-app --name=nodeport --port=80 --target-port=8080 --type=NodePort
service/nodeport exposed
```

### 获取访问地址

`NODEPORT=$(kubectl get -o jsonpath="{.spec.ports[0].nodePort}" services nodeport)`

```
[root@master ~]# NODEPORT=$(kubectl get -o jsonpath="{.spec.ports[0].nodePort}" services nodeport)
[root@master ~]# echo $NODEPORT
31435
```

`NODES=$(kubectl get nodes -o jsonpath='{ $.items[*].status.addresses[?(@.type=="InternalIP")].address }')`

```
[root@master ~]# NODES=$(kubectl get nodes -o jsonpath='{ $.items[*].status.addresses[?(@.type=="InternalIP")].address }')
[root@master ~]# echo $NODES
192.168.56.101 192.168.56.102 192.168.56.103
```

### 第一次访问测试

`for node in $NODES; do curl -s $node:$NODEPORT | grep -i client_address; done`

```
[root@master ~]# for node in $NODES; do curl -s $node:$NODEPORT | grep -i client_address; done
        client_address=::ffff:10.244.0.0
        client_address=::ffff:10.244.1.0
        client_address=::ffff:10.244.2.1
```

可以这些并不是真正的客户端 IP 地址，而是集群 IP 地址。

### 修改 Service

`kubectl patch svc nodeport -p '{"spec":{"externalTrafficPolicy":"Local"}}'`

```
[root@master ~]# kubectl patch svc nodeport -p '{"spec":{"externalTrafficPolicy":"Local"}}'
service/nodeport patched
```

### 修改后访问

`for node in $NODES; do curl --connect-timeout 1 -s $node:$NODEPORT | grep -i client_address; done`

```
[root@master ~]# for node in $NODES; do curl --connect-timeout 1 -s $node:$NODEPORT | grep -i client_address; done
        client_address=::ffff:192.168.56.101
```

可以看到已经获取正确的客户端 IP 了，但是访问的三个 Node，只有一个 Node 有返回。

## 总结

设置 service.spec.externalTrafficPolicy 的值为 Local，请求就只会被代理到本地 endpoints 而不会被转发到其它节点。这样就保留了最初的源 IP 地址。如果没有本地 endpoints，发送到这个节点的数据包将会被丢弃。

## 附录
