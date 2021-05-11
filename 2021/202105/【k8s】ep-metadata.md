# 【k8s】ep-metadata

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

查看 Endpoints 元数据信息。

## 示例

### 查看描述

```
[root@master ~]# kubectl describe ep kubernetes
Name:         kubernetes
Namespace:    default
Labels:       endpointslice.kubernetes.io/skip-mirror=true
Annotations:  <none>
Subsets:
  Addresses:          192.168.56.101
  NotReadyAddresses:  <none>
  Ports:
    Name   Port  Protocol
    ----   ----  --------
    https  6443  TCP

Events:  <none>
```

### 查看详情

```
[root@master ~]# kubectl get  ep kubernetes -o yaml
apiVersion: v1
kind: Endpoints
metadata:
  creationTimestamp: "2021-01-24T07:44:46Z"
  labels:
    endpointslice.kubernetes.io/skip-mirror: "true"
  managedFields:
  - apiVersion: v1
    fieldsType: FieldsV1
    fieldsV1:
      f:metadata:
        f:labels:
          .: {}
          f:endpointslice.kubernetes.io/skip-mirror: {}
      f:subsets: {}
    manager: kube-apiserver
    operation: Update
    time: "2021-01-24T07:44:46Z"
  name: kubernetes
  namespace: default
  resourceVersion: "204"
  uid: 05165bb0-9626-4301-9af6-d48991884234
subsets:
- addresses:
  - ip: 192.168.56.101
  ports:
  - name: https
    port: 6443
    protocol: TCP
```

## 总结

查看 Endpoints 的元数据信息。

## 附录
