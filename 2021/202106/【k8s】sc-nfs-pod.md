# 【k8s】sc-nfs-pod

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-RC1

## 目标

Pod 中使用 sc 创建的 pvc。

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

### pod.yaml

```yaml
kind: Pod
apiVersion: v1
metadata:
  name: test-pod
spec:
  containers:
    - name: test-pod
      image: busybox:stable
      command:
        - "/bin/sh"
      args:
        - "-c"
        - "echo 'jiangbo' > /mnt/name.txt"
      volumeMounts:
        - name: nfs-pvc
          mountPath: "/mnt"
  restartPolicy: "Never"
  volumes:
    - name: nfs-pvc
      persistentVolumeClaim:
        claimName: pvc
```

### 查看

```
[root@master nfs]# kubectl get pod
NAME       READY   STATUS      RESTARTS   AGE
test-pod   0/1     Completed   0          5s
[root@master nfs]# kubectl get pod,pvc,pv
NAME           READY   STATUS      RESTARTS   AGE
pod/test-pod   0/1     Completed   0          36s

NAME                        STATUS   VOLUME                                     CAPACITY   ACCESS MODES   STORAGECLASS   AGE
persistentvolumeclaim/pvc   Bound    pvc-be6a882d-5b39-43bd-86ac-1ee17b581ac8   1Mi        RWX            nfs-storage    57s

NAME                                                        CAPACITY   ACCESS MODES   RECLAIM POLICY   STATUS   CLAIM         STORAGECLASS   REASON   AGE
persistentvolume/pvc-be6a882d-5b39-43bd-86ac-1ee17b581ac8   1Mi        RWX            Delete           Bound    default/pvc   nfs-storage             57s
[root@master nfs]#
```

### 查看 nfs 目录

```
[root@master nfs]# ll
total 0
drwxrwxrwx 2 nfsnobody nfsnobody 22 May 30 22:10 default-pvc-pvc-be6a882d-5b39-43bd-86ac-1ee17b581ac8
[root@master nfs]# cd default-pvc-pvc-be6a882d-5b39-43bd-86ac-1ee17b581ac8/
[root@master default-pvc-pvc-be6a882d-5b39-43bd-86ac-1ee17b581ac8]# ls
name.txt
[root@master default-pvc-pvc-be6a882d-5b39-43bd-86ac-1ee17b581ac8]# cat name.txt
jiangbo
[root@master default-pvc-pvc-be6a882d-5b39-43bd-86ac-1ee17b581ac8]#
```

## 总结

Pod 中使用 sc 创建的 pvc。

## 附录
