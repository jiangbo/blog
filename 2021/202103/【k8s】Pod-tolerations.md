# 【k8s】Pod-tolerations

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

前面发现将 Pod 调度到 master 节点的时候，有个污点，不能被调度上去。
tolerations 即容忍度，可以容忍污点，直接调度上去。

## 示例

### Pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
spec:
  containers:
    - name: spring-k8s
      image: jiangbo920827/spring-k8s:liveness
      ports:
        - containerPort: 8080
  nodeSelector:
    kubernetes.io/hostname: master
  tolerations:
    - key: node-role.kubernetes.io/master
```

### 查看 master 污点

```
[root@master ~]# kubectl describe node master
Name:               master
Roles:              control-plane,master
Labels:             beta.kubernetes.io/arch=amd64
                    beta.kubernetes.io/os=linux
                    kubernetes.io/arch=amd64
                    kubernetes.io/hostname=master
                    kubernetes.io/os=linux
                    node-role.kubernetes.io/control-plane=
                    node-role.kubernetes.io/master=
Annotations:        flannel.alpha.coreos.com/backend-data: {"VNI":1,"VtepMAC":"7a:d5:ed:16:23:85"}
                    flannel.alpha.coreos.com/backend-type: vxlan
                    flannel.alpha.coreos.com/kube-subnet-manager: true
                    flannel.alpha.coreos.com/public-ip: 192.168.56.101
                    kubeadm.alpha.kubernetes.io/cri-socket: /var/run/dockershim.sock
                    node.alpha.kubernetes.io/ttl: 0
                    volumes.kubernetes.io/controller-managed-attach-detach: true
CreationTimestamp:  Sun, 24 Jan 2021 15:44:44 +0800
Taints:             node-role.kubernetes.io/master:NoSchedule
Unschedulable:      false
```

可以看到有个一个叫做 node-role.kubernetes.io/master 的污点，我们容忍这个污点，就可以直接调度上去。

### 查看

```
[root@master ~]# kubectl get pod -o wide
NAME         READY   STATUS    RESTARTS   AGE     IP            NODE     NOMINATED NODE   READINESS GATES
spring-k8s   1/1     Running   0          2m52s   10.244.0.28   master   <none>           <none>
```

## 总结

tolerations 可以定义容忍度，忽略已有的污点。

## 附录
