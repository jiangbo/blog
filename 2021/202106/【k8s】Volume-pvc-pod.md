# 【k8s】Volume-pvc-pod

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-RC1

## 目标

在 Pod 中使用 pvc。

## 示例

### 前提

```
[root@master nfs]# kubectl get pv,pvc
NAME                  CAPACITY   ACCESS MODES   RECLAIM POLICY   STATUS   CLAIM         STORAGECLASS   REASON   AGE
persistentvolume/pv   2Gi        RWX            Retain           Bound    default/pvc                           5m45s

NAME                        STATUS   VOLUME   CAPACITY   ACCESS MODES   STORAGECLASS   AGE
persistentvolumeclaim/pvc   Bound    pv       2Gi        RWX                           5m33s
```

### pod.yaml

```yaml
kind: Pod
apiVersion: v1
metadata:
  name: jiangbo
spec:
  containers:
    - name: jiangbo
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
NAME      READY   STATUS      RESTARTS   AGE
jiangbo   0/1     Completed   0          90s
```

### 查看 nfs

```
[root@master nfs]# ll
total 4
-rw-r--r-- 1 nfsnobody nfsnobody 8 May 30 20:42 name.txt
[root@master nfs]# cat name.txt
jiangbo
[root@master nfs]#
```

## 总结

学习了在 Pod 中怎么使用 pvc。

## 附录
