# 【k8s】sc-nfs-pvc

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-RC1

## 目标

无需创建 pv 来创建 pvc。

## pvc.yaml

```yaml
kind: PersistentVolumeClaim
apiVersion: v1
metadata:
  name: pvc
spec:
  storageClassName: nfs-storage
  accessModes:
    - ReadWriteMany
  resources:
    requests:
      storage: 1Mi
```

### 查看

```
[root@master ~]# kubectl get pv
NAME                                       CAPACITY   ACCESS MODES   RECLAIM POLICY   STATUS   CLAIM         STORAGECLASS   REASON   AGE
pvc-10336cca-fbb4-4283-adf8-d0ea5263aa0e   1Mi        RWX            Delete           Bound    default/pvc   nfs-storage             28s
[root@master ~]# kubectl get pvc
NAME   STATUS   VOLUME                                     CAPACITY   ACCESS MODES   STORAGECLASS   AGE
pvc    Bound    pvc-10336cca-fbb4-4283-adf8-d0ea5263aa0e   1Mi        RWX            nfs-storage    30s
```

### 查看详细

```
[root@master ~]# kubectl describe pvc pvc
Name:          pvc
Namespace:     default
StorageClass:  nfs-storage
Status:        Bound
Volume:        pvc-10336cca-fbb4-4283-adf8-d0ea5263aa0e
Labels:        <none>
Annotations:   pv.kubernetes.io/bind-completed: yes
               pv.kubernetes.io/bound-by-controller: yes
               volume.beta.kubernetes.io/storage-provisioner: nfs-provisioner
Finalizers:    [kubernetes.io/pvc-protection]
Capacity:      1Mi
Access Modes:  RWX
VolumeMode:    Filesystem
Used By:       <none>
Events:
  Type    Reason                 Age   From                                                                                         Message
  ----    ------                 ----  ----                                                                                         -------
  Normal  ExternalProvisioning   65s   persistentvolume-controller                                                                  waiting for a volume to be created, either by external provisioner "nfs-provisioner" or manually created by system administrator
  Normal  Provisioning           64s   nfs-provisioner_nfs-client-provisioner-66d5b856b-vq2sv_f04ed455-4a71-4824-9a27-a5da811b0d68  External provisioner is provisioning volume for claim "default/pvc"
  Normal  ProvisioningSucceeded  64s   nfs-provisioner_nfs-client-provisioner-66d5b856b-vq2sv_f04ed455-4a71-4824-9a27-a5da811b0d68  Successfully provisioned volume pvc-10336cca-fbb4-4283-adf8-d0ea5263aa0e
```

## 总结

使用 nfs 的 sc 来创建了一个 pvc 和 pv。

## 附录
