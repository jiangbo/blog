# 【k8s】sc-nfs

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-RC1

## 目标

创建了一个 nfs 类型的 sc。

## deploy.yaml

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nfs-client-provisioner
  labels:
    app: nfs-client-provisioner
  namespace: kube-system
spec:
  replicas: 1
  strategy:
    type: Recreate
  selector:
    matchLabels:
      app: nfs-client-provisioner
  template:
    metadata:
      labels:
        app: nfs-client-provisioner
    spec:
      serviceAccountName: nfs-client-provisioner
      containers:
        - name: nfs-client-provisioner
          image: jiangbo920827/nfs-subdir-external-provisioner:v4.0.2
          volumeMounts:
            - name: nfs-client-root
              mountPath: /persistentvolumes
          env:
            - name: PROVISIONER_NAME
              value: nfs-provisioner
            - name: NFS_SERVER
              value: 192.168.56.101
            - name: NFS_PATH
              value: /opt/nfs
      volumes:
        - name: nfs-client-root
          nfs:
            server: 192.168.56.101
            path: /opt/nfs
```

### rbac.yaml

```yaml
apiVersion: v1
kind: ServiceAccount
metadata:
  name: nfs-client-provisioner
  namespace: kube-system
---
kind: ClusterRoleBinding
apiVersion: rbac.authorization.k8s.io/v1
metadata:
  name: run-nfs-client-provisioner
subjects:
  - kind: ServiceAccount
    name: nfs-client-provisioner
    namespace: kube-system
roleRef:
  kind: ClusterRole
  name: cluster-admin
  apiGroup: rbac.authorization.k8s.io
```

### class.yaml

```yaml
apiVersion: storage.k8s.io/v1
kind: StorageClass
metadata:
  name: nfs-storage
provisioner: nfs-provisioner
parameters:
  archiveOnDelete: "false"
```

### 查看

```
[root@master ~]# kubectl get sc
NAME          PROVISIONER       RECLAIMPOLICY   VOLUMEBINDINGMODE   ALLOWVOLUMEEXPANSION   AGE
nfs-storage   nfs-provisioner   Delete          Immediate           false                  5h24m
[root@master ~]# kubectl describe sc nfs-storage
Name:            nfs-storage
IsDefaultClass:  No
Annotations:     kubectl.kubernetes.io/last-applied-configuration={"apiVersion":"storage.k8s.io/v1","kind":"StorageClass","metadata":{"annotations":{},"name":"nfs-storage"},"parameters":{"archiveOnDelete":"false"},"provisioner":"nfs-provisioner"}

Provisioner:           nfs-provisioner
Parameters:            archiveOnDelete=false
AllowVolumeExpansion:  <unset>
MountOptions:          <none>
ReclaimPolicy:         Delete
VolumeBindingMode:     Immediate
Events:                <none>
```

## 总结

创建一个 nfs 类型的 sc。

## 附录
