# 【k8s】Volume-pv-local

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-RC1

## 目标

使用 local 类型的数据卷时，和 hostPath 类型的数据卷很类似，不过可以针对集群。
存储还是在单节点上，不过可以调整调度的节点。


## 示例

### pv.yaml

```yaml
apiVersion: v1
kind: PersistentVolume
metadata:
  name: pv-local
spec:
  capacity:
    storage: 5Gi
  nodeAffinity:
    required:
      nodeSelectorTerms:
        - matchFields:
            - key: metadata.name
              operator: In
              values: [node2]
  accessModes:
    - ReadWriteOnce
  local:
    path: /opt/local
```

目录必须要先存在

### 查看

```
[root@master ~]# kubectl get pv
NAME       CAPACITY   ACCESS MODES   RECLAIM POLICY   STATUS      CLAIM   STORAGECLASS   REASON   AGE
pv-local   5Gi        RWO            Retain           Available                                   3s
```

### 查看详情

```
[root@master ~]# kubectl describe pv pv-local
Name:              pv-local
Labels:            <none>
Annotations:       <none>
Finalizers:        [kubernetes.io/pv-protection]
StorageClass:
Status:            Available
Claim:
Reclaim Policy:    Retain
Access Modes:      RWO
VolumeMode:        Filesystem
Capacity:          5Gi
Node Affinity:
  Required Terms:
    Term 0:        <none>
Message:
Source:
    Type:  LocalVolume (a persistent volume backed by local storage on a node)
    Path:  /opt/local
Events:    <none>
```

## 总结

local 类型的 pv 可以用来用作存储，不过是和节点绑定的，如果节点挂了，存储的数据也使用不了。

## 附录
