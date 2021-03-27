# 【k8s】Pod-hostNetwork

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

hostNetwork 可以直接使用宿主机的网络。

## 示例

### Pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
spec:
  containers:
    - name: spring-k8s
      image: jiangbo920827/spring-k8s:liveness
      ports:
        - containerPort: 8080
  hostNetwork: true
```

### 查看网卡

```
[root@master ~]# kubectl exec -it spring-k8s -- sh
/ # ip a
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
    inet 127.0.0.1/8 scope host lo
       valid_lft forever preferred_lft forever
    inet6 ::1/128 scope host
       valid_lft forever preferred_lft forever
2: enp0s3: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state UP qlen 1000
    link/ether 08:00:27:3c:23:ef brd ff:ff:ff:ff:ff:ff
    inet 10.0.2.15/24 brd 10.0.2.255 scope global dynamic enp0s3
       valid_lft 71200sec preferred_lft 71200sec
    inet6 fe80::855f:65aa:814e:9e83/64 scope link
       valid_lft forever preferred_lft forever
3: enp0s8: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state UP qlen 1000
    link/ether 08:00:27:52:cf:cb brd ff:ff:ff:ff:ff:ff
    inet 192.168.56.103/24 brd 192.168.56.255 scope global enp0s8
       valid_lft forever preferred_lft forever
    inet6 fe80::54f3:b87b:ad63:1f01/64 scope link
       valid_lft forever preferred_lft forever
4: docker0: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc noqueue state DOWN
    link/ether 02:42:47:8c:2a:90 brd ff:ff:ff:ff:ff:ff
    inet 172.17.0.1/16 brd 172.17.255.255 scope global docker0
       valid_lft forever preferred_lft forever
5: dummy0: <BROADCAST,NOARP> mtu 1500 qdisc noop state DOWN qlen 1000
    link/ether be:17:64:09:b0:9c brd ff:ff:ff:ff:ff:ff
6: kube-ipvs0: <BROADCAST,NOARP> mtu 1500 qdisc noop state DOWN
    link/ether 5a:7b:df:03:bc:16 brd ff:ff:ff:ff:ff:ff
    inet 10.96.0.10/32 scope global kube-ipvs0
       valid_lft forever preferred_lft forever
    inet 10.97.109.175/32 scope global kube-ipvs0
       valid_lft forever preferred_lft forever
    inet 10.98.201.56/32 scope global kube-ipvs0
       valid_lft forever preferred_lft forever
    inet 10.96.0.1/32 scope global kube-ipvs0
       valid_lft forever preferred_lft forever
    inet 10.110.153.250/32 scope global kube-ipvs0
       valid_lft forever preferred_lft forever
7: flannel.1: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1450 qdisc noqueue state UNKNOWN
    link/ether 76:12:3e:41:80:da brd ff:ff:ff:ff:ff:ff
    inet 10.244.2.0/32 brd 10.244.2.0 scope global flannel.1
       valid_lft forever preferred_lft forever
    inet6 fe80::7412:3eff:fe41:80da/64 scope link
       valid_lft forever preferred_lft forever
8: cni0: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc noqueue state DOWN qlen 1000
    link/ether ca:5d:64:78:25:af brd ff:ff:ff:ff:ff:ff
    inet 10.244.2.1/24 brd 10.244.2.255 scope global cni0
       valid_lft forever preferred_lft forever
    inet6 fe80::c85d:64ff:fe78:25af/64 scope link
       valid_lft forever preferred_lft forever
```

### 通过宿主机访问

```
[root@master ~]# curl 192.168.56.103:8080/hostname;echo
node2
```

## 总结

hostNetwork 可以直接使用宿主机的网卡。

## 附录
