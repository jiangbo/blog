# 【k8s】Volume-pv

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-RC1

## 目标

在定义数据卷的时候，我们发现想要定义个数据卷，必须要精确知道该种类型需要哪些配置，
在完全配置正确后，才可以正确使用。为了封装不同类型的数据卷，提供统一的操作方式，
有了 PersistentVolume，也叫 pv 的概念。


## 示例

### pv.yaml

```yaml
apiVersion: v1
kind: PersistentVolume
metadata:
  name: pv-hostpath
spec:
  capacity:
    storage: 5Gi
  accessModes:
    - ReadWriteOnce
  hostPath:
    path: /opt/hostpath
```
创建了一个 hostpath 类型的 pv，不过该 pv 只适合在单节点的 k8s，如果是集群，最好使用 local 代替。

### 查看

```
[root@master ~]# kubectl get pv
NAME          CAPACITY   ACCESS MODES   RECLAIM POLICY   STATUS      CLAIM   STORAGECLASS   REASON   AGE
pv-hostpath   5Gi        RWO            Retain           Available                                   2m59s
```

### 查看详情

```
[root@master ~]# kubectl describe pv pv-hostpath
Name:            pv-hostpath
Labels:          <none>
Annotations:     <none>
Finalizers:      [kubernetes.io/pv-protection]
StorageClass:
Status:          Available
Claim:
Reclaim Policy:  Retain
Access Modes:    RWO
VolumeMode:      Filesystem
Capacity:        5Gi
Node Affinity:   <none>
Message:
Source:
    Type:          HostPath (bare host directory volume)
    Path:          /opt/hostpath
    HostPathType:
Events:            <none>
```

## 总结

pv 可以存储的一些细节进行屏蔽，统一由管理员或者运维来提供，开发可以不关心存储的细节。

## 附录
