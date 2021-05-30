# 【k8s】Volume-pvc

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-RC1

## 目标

在建立好 pv 之后，如果要使用存储，则需要建立对应的 pvc 去和 pv 进行绑定。
pvc 全称是 PersistentVolumeClaim，表示使用 pv 的声明。

## 示例

### 建立 pv

```
[root@master ~]# kubectl get pv
NAME   CAPACITY   ACCESS MODES   RECLAIM POLICY   STATUS      CLAIM   STORAGECLASS   REASON   AGE
pv1    1Gi        RWX            Retain           Available                                   25s
pv2    2Gi        RWX            Retain           Available                                   14s
pv5    5Gi        RWX            Retain           Available                                   5s
```

### pvc.yaml

```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: pvc2
spec:
  resources:
    requests:
      storage: 2Gi
  accessModes:
    - ReadWriteMany
```

### 查看

```
[root@master ~]# kubectl get pv
NAME   CAPACITY   ACCESS MODES   RECLAIM POLICY   STATUS      CLAIM          STORAGECLASS   REASON   AGE
pv1    1Gi        RWX            Retain           Available                                          3m56s
pv2    2Gi        RWX            Retain           Bound       default/pvc2                           3m45s
pv5    5Gi        RWX            Retain           Available                                          3m36s
[root@master ~]# kubectl get pvc
NAME   STATUS   VOLUME   CAPACITY   ACCESS MODES   STORAGECLASS   AGE
pvc2   Bound    pv2      2Gi        RWX                           3s
```

### 查看详情

```
[root@master ~]# kubectl describe pvc pvc2
Name:          pvc2
Namespace:     default
StorageClass:
Status:        Bound
Volume:        pv2
Labels:        <none>
Annotations:   pv.kubernetes.io/bind-completed: yes
               pv.kubernetes.io/bound-by-controller: yes
Finalizers:    [kubernetes.io/pvc-protection]
Capacity:      2Gi
Access Modes:  RWX
VolumeMode:    Filesystem
Used By:       <none>
Events:        <none>
```

## 总结

pvc 可以和 pv 进行绑定，然后提供存储。

## 附录
