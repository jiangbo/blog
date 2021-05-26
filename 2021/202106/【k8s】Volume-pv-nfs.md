# 【k8s】Volume-pv-nfs

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-RC1

## 目标

使用 nfs 类型的 pv 时，可以不要求存储在节点中，可以存储到远程的 nfs 服务器。

## 示例

### pv.yaml

```yaml
apiVersion: v1
kind: PersistentVolume
metadata:
  name: pv-nfs
spec:
  capacity:
    storage: 5Gi
  accessModes:
    - ReadWriteMany
  nfs:
    path: /opt/nfs
    server: 192.168.56.101
```

accessModes 的可选值为：
1. ReadWriteOnce(RWO) -- 只能单个节点进行读写
2. ReadOnlyMany(ROM) -- 多个节点只读
3. ReadWriteMany(RWM) -- 多个节点读写

### 查看

```
[root@master ~]# kubectl get pv
NAME       CAPACITY   ACCESS MODES   RECLAIM POLICY   STATUS      CLAIM   STORAGECLASS   REASON   AGE
pv-local   5Gi        RWO            Retain           Available                                   3m41s
pv-nfs     5Gi        RWX            Retain           Available                                   6s
```

### 查看详情

```
[root@master ~]# kubectl describe pv pv-nfs
Name:            pv-nfs
Labels:          <none>
Annotations:     <none>
Finalizers:      [kubernetes.io/pv-protection]
StorageClass:
Status:          Available
Claim:
Reclaim Policy:  Retain
Access Modes:    RWX
VolumeMode:      Filesystem
Capacity:        5Gi
Node Affinity:   <none>
Message:
Source:
    Type:      NFS (an NFS mount that lasts the lifetime of a pod)
    Server:    192.168.56.101
    Path:      /opt/nfs
    ReadOnly:  false
Events:        <none>
[root@master ~]#
```

## 总结

nfs 类型的 pv 可以供多个节点进行读写。

## 附录
