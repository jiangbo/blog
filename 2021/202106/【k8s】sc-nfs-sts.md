# 【k8s】sc-nfs-sts

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-RC1

## 目标

sts 可以使用 pvc 模板创建。

## sts.yaml

```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: mysts
spec:
  selector:
    matchLabels:
      app: mysts
  serviceName: mysts
  replicas: 3
  template:
    metadata:
      labels:
        app: mysts
    spec:
      containers:
        - name: mysts
          image: busybox:stable
          command:
            - "/bin/sh"
          args:
            - "-c"
            - "hostname > /mnt/hostname.txt;sleep 3600"
          volumeMounts:
            - name: pvc
              mountPath: /mnt
  volumeClaimTemplates:
    - metadata:
        name: pvc
      spec:
        accessModes: ["ReadWriteMany"]
        storageClassName: nfs-storage
        resources:
          requests:
            storage: 1Mi
```

### 查看

```
[root@master ~]# kubectl get pod
NAME      READY   STATUS    RESTARTS   AGE
mysts-0   1/1     Running   0          15s
mysts-1   1/1     Running   0          13s
mysts-2   1/1     Running   0          9s
[root@master ~]# kubectl get pvc
NAME          STATUS   VOLUME                                     CAPACITY   ACCESS MODES   STORAGECLASS   AGE
pvc-mysts-0   Bound    pvc-3bdabcce-10bb-49d8-b667-b8de31970806   1Mi        RWX            nfs-storage    2m58s
pvc-mysts-1   Bound    pvc-31adf6d4-cd72-4a94-830e-c215dd1f1456   1Mi        RWX            nfs-storage    16s
pvc-mysts-2   Bound    pvc-18346c10-9690-4544-9a18-470790f1d5b1   1Mi        RWX            nfs-storage    12s
[root@master ~]# kubectl get pv
NAME                                       CAPACITY   ACCESS MODES   RECLAIM POLICY   STATUS   CLAIM                 STORAGECLASS   REASON   AGE
pvc-18346c10-9690-4544-9a18-470790f1d5b1   1Mi        RWX            Delete           Bound    default/pvc-mysts-2   nfs-storage             15s
pvc-31adf6d4-cd72-4a94-830e-c215dd1f1456   1Mi        RWX            Delete           Bound    default/pvc-mysts-1   nfs-storage             19s
pvc-3bdabcce-10bb-49d8-b667-b8de31970806   1Mi        RWX            Delete           Bound    default/pvc-mysts-0   nfs-storage             3m1s
```

### 查看 nfs 目录

```
[root@master nfs]# ll
total 0
drwxrwxrwx 2 nfsnobody nfsnobody 26 May 30 22:17 default-pvc-mysts-0-pvc-3bdabcce-10bb-49d8-b667-b8de31970806
drwxrwxrwx 2 nfsnobody nfsnobody 26 May 30 22:20 default-pvc-mysts-1-pvc-31adf6d4-cd72-4a94-830e-c215dd1f1456
drwxrwxrwx 2 nfsnobody nfsnobody 26 May 30 22:20 default-pvc-mysts-2-pvc-18346c10-9690-4544-9a18-470790f1d5b1
[root@master nfs]# cat */hostname.txt
mysts-0
mysts-1
mysts-2
```

## 总结

sts 使用 pvc 模板创建存储。

## 附录
