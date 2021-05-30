# 【k8s】Volume-persistentVolumeReclaimPolicy

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-RC1

## 目标

persistentVolumeReclaimPolicy 表示 pv 的重用策略，如果手动创建的 pv 默认为 Retain。
如果是动态提供的，默认为 Delete。还有一种 Recycle 已经过时了。

## 示例

### 建立 pv 和 pvc

```
[root@master ~]# kubectl get pv
NAME   CAPACITY   ACCESS MODES   RECLAIM POLICY   STATUS      CLAIM          STORAGECLASS   REASON   AGE
pv1    1Gi        RWX            Retain           Available                                          8m32s
pv2    2Gi        RWX            Retain           Bound       default/pvc2                           8m21s
pv5    5Gi        RWX            Retain           Bound       default/pvc3                           8m12s
```

### pvc.yaml

```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: pvc-test
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
pv1    1Gi        RWX            Retain           Available                                          9m28s
pv2    2Gi        RWX            Retain           Bound       default/pvc2                           9m17s
pv5    5Gi        RWX            Retain           Bound       default/pvc3                           9m8s
[root@master ~]# kubectl get pvc
NAME       STATUS    VOLUME   CAPACITY   ACCESS MODES   STORAGECLASS   AGE
pvc-test   Pending                                                     16s
pvc2       Bound     pv2      2Gi        RWX                           6m10s
pvc3       Bound     pv5      5Gi        RWX                           63s
```

虽然 pv1 还没有被绑定，但是 pvc-test 要求 2Gi 的存储，满足不了，所以进入了 Pending 状态。

### 删除在使用的 pvc

```
[root@master ~]# kubectl delete pvc pvc3
persistentvolumeclaim "pvc3" deleted
[root@master ~]# kubectl get pv
NAME   CAPACITY   ACCESS MODES   RECLAIM POLICY   STATUS      CLAIM          STORAGECLASS   REASON   AGE
pv1    1Gi        RWX            Retain           Available                                          11m
pv2    2Gi        RWX            Retain           Bound       default/pvc2                           11m
pv5    5Gi        RWX            Retain           Released    default/pvc3                           10m
[root@master ~]# kubectl get pvc
NAME       STATUS    VOLUME   CAPACITY   ACCESS MODES   STORAGECLASS   AGE
pvc-test   Pending                                                     2m3s
pvc2       Bound     pv2      2Gi        RWX                           7m57s
```

虽然将 pvc3 删除了，但是 pv5 的状态变成了 Released，也是不可以提供给 pv 使用的。
所以 pvc-test 还是 Pending 状态。原因是策略 Retain 会保留删除了 pvc 的 pv，并不会重新使用。

## 总结

pv 的 persistentVolumeReclaimPolicy 定义了 pvc 删除时，pv 的保留策略。

## 附录
