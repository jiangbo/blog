# 【k8s】修改 k8s service 端口范围

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.1

## 目标

在 k8s 中使用 NodePort 的时候，随机分配的端口范围默认在 30000-32767 之间。
为了方便我们直接访问地址，不需要加端口，可以扩大端口范围，缺点是可能占用其它程序会使用的端口。
下面的配置是基于 kubeadm 安装的集群，其它方式可能会有不一致的地方。

## 示例

### 查看 api server 的配置

在 /etc/kubernetes/manifests 目录下找到 api server 的配置。

```yaml
apiVersion: v1
kind: Pod
metadata:
  annotations:
    kubeadm.kubernetes.io/kube-apiserver.advertise-address.endpoint: 192.168.56.101:6443
  creationTimestamp: null
  labels:
    component: kube-apiserver
    tier: control-plane
  name: kube-apiserver
  namespace: kube-system
spec:
  containers:
  - command:
    - kube-apiserver
    - --advertise-address=192.168.56.101
    - --allow-privileged=true
    - --authorization-mode=Node,RBAC
    - --client-ca-file=/etc/kubernetes/pki/ca.crt
    - --enable-admission-plugins=NodeRestriction
    - --enable-bootstrap-token-auth=true
    - --etcd-cafile=/etc/kubernetes/pki/etcd/ca.crt
    - --etcd-certfile=/etc/kubernetes/pki/apiserver-etcd-client.crt
    - --etcd-keyfile=/etc/kubernetes/pki/apiserver-etcd-client.key
    - --etcd-servers=https://127.0.0.1:2379
    - --insecure-port=0
    - --kubelet-client-certificate=/etc/kubernetes/pki/apiserver-kubelet-client.crt
    - --kubelet-client-key=/etc/kubernetes/pki/apiserver-kubelet-client.key
    - --kubelet-preferred-address-types=InternalIP,ExternalIP,Hostname
    - --proxy-client-cert-file=/etc/kubernetes/pki/front-proxy-client.crt
    - --proxy-client-key-file=/etc/kubernetes/pki/front-proxy-client.key
    - --requestheader-allowed-names=front-proxy-client
    - --requestheader-client-ca-file=/etc/kubernetes/pki/front-proxy-ca.crt
    - --requestheader-extra-headers-prefix=X-Remote-Extra-
    - --requestheader-group-headers=X-Remote-Group
    - --requestheader-username-headers=X-Remote-User
    - --secure-port=6443
    - --service-account-issuer=https://kubernetes.default.svc.cluster.local
    - --service-account-key-file=/etc/kubernetes/pki/sa.pub
    - --service-account-signing-key-file=/etc/kubernetes/pki/sa.key
    - --service-cluster-ip-range=10.96.0.0/12
    - --tls-cert-file=/etc/kubernetes/pki/apiserver.crt
    - --tls-private-key-file=/etc/kubernetes/pki/apiserver.key
    - --service-node-port-range=80-32767
    image: registry.aliyuncs.com/google_containers/kube-apiserver:v1.20.6
    imagePullPolicy: IfNotPresent
    livenessProbe:
      failureThreshold: 8
      httpGet:
        host: 192.168.56.101
        path: /livez
        port: 6443
        scheme: HTTPS
      initialDelaySeconds: 10
      periodSeconds: 10
      timeoutSeconds: 15
    name: kube-apiserver
    readinessProbe:
      failureThreshold: 3
      httpGet:
        host: 192.168.56.101
        path: /readyz
        port: 6443
        scheme: HTTPS
      periodSeconds: 1
      timeoutSeconds: 15
    startupProbe:
      failureThreshold: 24
      httpGet:
        host: 192.168.56.101
        path: /livez
        port: 6443
        scheme: HTTPS
      initialDelaySeconds: 10
      periodSeconds: 10
      timeoutSeconds: 15
    volumeMounts:
    - mountPath: /etc/ssl/certs
      name: ca-certs
      readOnly: true
    - mountPath: /etc/pki
      name: etc-pki
      readOnly: true
    - mountPath: /etc/kubernetes/pki
      name: k8s-certs
      readOnly: true
  hostNetwork: true
  priorityClassName: system-node-critical
  volumes:
  - hostPath:
      path: /etc/ssl/certs
      type: DirectoryOrCreate
    name: ca-certs
  - hostPath:
      path: /etc/pki
      type: DirectoryOrCreate
    name: etc-pki
  - hostPath:
      path: /etc/kubernetes/pki
      type: DirectoryOrCreate
    name: k8s-certs
status: {}
```

如果没有发现 service-node-port-range 配置，可以手动加一个，然后填写端口范围，在这里，将端口的范围扩大到从 80 开始。

### 配置 80 和 443 端口

在之前，配置了一个 Nginx Ingress，并且开放了 NodePort 端口，在这里，将其配置为默认的端口。

```
[root@master ~]# kubectl get svc -A | grep ingress
kube-system   ingress-nginx-controller             NodePort       10.111.181.202   <none>                                               80:80/TCP,443:443/TCP    67d
kube-system   ingress-nginx-controller-admission   ClusterIP      10.97.90.55      <none>                                               443/TCP                  67d
```

现在通过 http 或者 https 访问，可以不加端口了，直接走默认端口。

## 总结

介绍了 k8s 怎么扩大 service 的 NodePort 端口范围。

## 附录
