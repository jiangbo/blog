# 【k8s】Secret

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-M3

## 目标

Secret 是和 ConfigMap 类似的配置，不过 Secret 是专门为敏感数据提供的，比如密码，SSH密钥等。
Kubernetes Secret 默认情况下存储为 base64-编码的、非加密的字符串。

## 示例

### 查看已有 Secret

```
[root@master ~]# kubectl get secrets
NAME                  TYPE                                  DATA   AGE
default-token-slbq5   kubernetes.io/service-account-token   3      111d
[root@master ~]# kubectl describe secrets default-token-slbq5
Name:         default-token-slbq5
Namespace:    default
Labels:       <none>
Annotations:  kubernetes.io/service-account.name: default
              kubernetes.io/service-account.uid: 0dcc2d45-9daf-4bef-aa42-d4a583a0d9ab

Type:  kubernetes.io/service-account-token

Data
====
ca.crt:     1066 bytes
namespace:  7 bytes
token:      eyJhbGciOiJSUzI1NiIsImtpZCI6InJyLWZ3RUtvM05xY3NKaTJOajhZNkhWMmNEV0dVYURsbDROT2pvTVJaQzAifQ.eyJpc3MiOiJrdWJlcm5ldGVzL3NlcnZpY2VhY2NvdW50Iiwia3ViZXJuZXRlcy5pby9zZXJ2aWNlYWNjb3VudC9uYW1lc3BhY2UiOiJkZWZhdWx0Iiwia3ViZXJuZXRlcy5pby9zZXJ2aWNlYWNjb3VudC9zZWNyZXQubmFtZSI6ImRlZmF1bHQtdG9rZW4tc2xicTUiLCJrdWJlcm5ldGVzLmlvL3NlcnZpY2VhY2NvdW50L3NlcnZpY2UtYWNjb3VudC5uYW1lIjoiZGVmYXVsdCIsImt1YmVybmV0ZXMuaW8vc2VydmljZWFjY291bnQvc2VydmljZS1hY2NvdW50LnVpZCI6IjBkY2MyZDQ1LTlkYWYtNGJlZi1hYTQyLWQ0YTU4M2EwZDlhYiIsInN1YiI6InN5c3RlbTpzZXJ2aWNlYWNjb3VudDpkZWZhdWx0OmRlZmF1bHQifQ.TKrCGHVgx1JHP9cMQbWpLsno7rRvnIKomdV8JEYfJbWE_9niQITlGlGwQjNzitzj5cVWGYCqp4ABmJk3REaBDMeQUyHgmwd00unbSMZczL3YevYpNl3YyxVyXjZVAA6Gj9CaKyWhjsFdLgDqlsXSpr1BRiei7dKrQo4-2c-0_I8KrXYfgV5xYG7rB9msjQFXp4VD3TvoK9gO0CEyZ0-07w18Sx36khKsU3ss5EoG8Nr0hIhU4QbWhUspygemUm19H3WRJU88bEDirShI82jjJQCWsBr-byNsfuFm4dlOsld3NVG0gSfMZtbhuAWcQfKmfjpf6q0-m2VT_e9jEmyUHg
```

### 创建 Secret

```
[root@master ~]# echo -n "jiangbo" | base64
amlhbmdibw==
[root@master ~]# echo -n "44" | base64
NDQ=
[root@master ~]#
```

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: mysecret
data:
  name: amlhbmdibw==
  age: NDQ=
```

```
[root@master ~]# kubectl describe secrets mysecret
Name:         mysecret
Namespace:    default
Labels:       <none>
Annotations:  <none>

Type:  Opaque

Data
====
age:   2 bytes
name:  7 bytes
```

### 命令行创建

```
[root@master ~]# kubectl create secret generic my-secret --from-literal=name=jiangbo --from-literal=age=44
secret/my-secret created
[root@master ~]# kubectl describe secrets my-secret
Name:         my-secret
Namespace:    default
Labels:       <none>
Annotations:  <none>

Type:  Opaque

Data
====
age:   2 bytes
name:  7 bytes
```

## 总结

Secret 用来保存敏感信息，默认情况是使用 base64 编码存储。

## 附录
