# 【k8s】Pod-hostAliases

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

hostAliases 可以在 /etc/host 文件中设置别名。

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
  hostAliases:
    - hostnames:
        - node2
      ip: 192.168.56.103
```

### 查看 host

```
[root@master ~]# kubectl exec -it spring-k8s -- sh
/ # ls
BOOT-INF  bin       etc       lib       mnt       org       root      sbin      sys       usr
META-INF  dev       home      media     opt       proc      run       srv       tmp       var
/ # cat /etc/hosts
# Kubernetes-managed hosts file.
127.0.0.1       localhost
::1     localhost ip6-localhost ip6-loopback
fe00::0 ip6-localnet
fe00::0 ip6-mcastprefix
fe00::1 ip6-allnodes
fe00::2 ip6-allrouters
10.244.2.169    spring-k8s

# Entries added by HostAliases.
192.168.56.103  node2
/ # ping node2
PING node2 (192.168.56.103): 56 data bytes
64 bytes from 192.168.56.103: seq=0 ttl=64 time=0.084 ms
64 bytes from 192.168.56.103: seq=1 ttl=64 time=0.071 ms
64 bytes from 192.168.56.103: seq=2 ttl=64 time=0.090 ms
^C
--- node2 ping statistics ---
3 packets transmitted, 3 packets received, 0% packet loss
round-trip min/avg/max = 0.071/0.081/0.090 ms
/ #
```

## 总结

hostAliases 可以在 /etc/host 文件中设置别名，并且可以通过别名访问对应的 IP。

## 附录
