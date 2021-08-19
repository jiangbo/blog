# 【k8s】创建 tls 类型 Secret

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.1

## 目标

之前已经了解过 Secret 了，并且创建 Opaque 类型的 Secret。除了之前创建的类型，还有 tls 的类型。

## 示例

### 准备私钥和证书

之前已经生成了服务器的证书和私钥，如下：

```text
[root@master nginx]# ll
total 16
-rw-r--r-- 1 root root 4199 Jun 13 23:11 nginx.crt
-rw-r--r-- 1 root root  989 Jun 13 23:10 nginx.csr
-rw-r--r-- 1 root root 1675 Jun 13 18:05 nginx.key
```

### 创建 Secret

```text
[root@master nginx]# kubectl create secret tls nginx-tls --key nginx.key --cert nginx.crt
secret/nginx-tls created
```

### 查看详情

```text
[root@master nginx]# kubectl describe secrets nginx-tls
Name:         nginx-tls
Namespace:    default
Labels:       <none>
Annotations:  <none>

Type:  kubernetes.io/tls

Data
====
tls.crt:  4199 bytes
tls.key:  1675 bytes
```

## 总结

介绍了创建 kubernetes.io/tls 类型的 Secret 的方式。

## 附录
