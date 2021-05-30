# 【k8s】rbac-sa

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-RC1

## 目标

RBAC 是 k8s 的鉴权机制，RBAC API 声明了四种 Kubernetes 对象：
Role、ClusterRole、RoleBinding 和 ClusterRoleBinding。
除了角色和绑定之外，还有主体，可以是用户，服务账号和组，其中服务账号接触较多。
服务账号的全称是 serviceaccount，可以简写为 sa。

## 查看

```
[root@master manifests]# kubectl get serviceaccounts
NAME      SECRETS   AGE
default   1         126d
[root@master manifests]# kubectl describe serviceaccounts
Name:                default
Namespace:           default
Labels:              <none>
Annotations:         <none>
Image pull secrets:  <none>
Mountable secrets:   default-token-slbq5
Tokens:              default-token-slbq5
Events:              <none>
```

### 查看 token

```
[root@master manifests]# kubectl get secrets
NAME                  TYPE                                  DATA   AGE
default-token-slbq5   kubernetes.io/service-account-token   3      126d
[root@master manifests]# kubectl describe secrets
Name:         default-token-slbq5
Namespace:    default
Labels:       <none>
Annotations:  kubernetes.io/service-account.name: default
              kubernetes.io/service-account.uid: 0dcc2d45-9daf-4bef-aa42-d4a583a0d9ab

Type:  kubernetes.io/service-account-token

Data
...
ca.crt:     1066 bytes
namespace:  7 bytes

```

### 创建 sa

```yaml
apiVersion: v1
kind: ServiceAccount
metadata:
  name: admin
  namespace: kube-system
```

### 查看 sa

```
[root@master manifests]# kubectl describe -n kube-system sa admin
Name:                admin
Namespace:           kube-system
Labels:              <none>
Annotations:         <none>
Image pull secrets:  <none>
Mountable secrets:   admin-token-q59qh
Tokens:              admin-token-q59qh
Events:              <none>
[root@master manifests]# kubectl get secrets -n kube-system admin-token-q59qh
NAME                TYPE                                  DATA   AGE
admin-token-q59qh   kubernetes.io/service-account-token   3      2m21s
[root@master manifests]#

```

## 总结

介绍了 rbac 中的 sa 的概念和使用方式。

## 附录
