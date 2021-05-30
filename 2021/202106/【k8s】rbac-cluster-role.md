# 【k8s】rbac-cluster-role

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-RC1

## 目标

其中服务账号可以和角色或者集群角色进行绑定，Role 和 ClusterRole 的区别是：
Role 有命名空间的限制，而 ClusterRole 可以跨命名空间。

## 查看 ClusterRole

```
[root@master manifests]# kubectl get clusterrole
NAME                                                                   CREATED AT
admin                                                                  2021-01-24T07:44:45Z
cluster-admin                                                          2021-01-24T07:44:45Z
edit                                                                   2021-01-24T07:44:45Z
flannel                                                                2021-03-21T08:00:50Z
ingress-nginx                                                          2021-05-01T05:21:49Z
ingress-nginx-admission                                                2021-05-01T05:21:49Z
kubeadm:get-nodes                                                      2021-01-24T07:44:48Z
kubernetes-dashboard                                                   2021-03-15T14:35:02Z
system:aggregate-to-admin                                              2021-01-24T07:44:45Z
system:aggregate-to-edit                                               2021-01-24T07:44:45Z
system:aggregate-to-view                                               2021-01-24T07:44:45Z
system:auth-delegator                                                  2021-01-24T07:44:45Z
system:basic-user                                                      2021-01-24T07:44:45Z
system:certificates.k8s.io:certificatesigningrequests:nodeclient       2021-01-24T07:44:45Z
system:certificates.k8s.io:certificatesigningrequests:selfnodeclient   2021-01-24T07:44:45Z
system:certificates.k8s.io:kube-apiserver-client-approver              2021-01-24T07:44:45Z
system:certificates.k8s.io:kube-apiserver-client-kubelet-approver      2021-01-24T07:44:45Z
system:certificates.k8s.io:kubelet-serving-approver                    2021-01-24T07:44:45Z
system:certificates.k8s.io:legacy-unknown-approver                     2021-01-24T07:44:45Z
system:controller:attachdetach-controller                              2021-01-24T07:44:45Z
system:controller:certificate-controller                               2021-01-24T07:44:45Z
system:controller:clusterrole-aggregation-controller                   2021-01-24T07:44:45Z
system:controller:cronjob-controller                                   2021-01-24T07:44:45Z
system:controller:daemon-set-controller                                2021-01-24T07:44:45Z
system:controller:deployment-controller                                2021-01-24T07:44:45Z
system:controller:disruption-controller                                2021-01-24T07:44:45Z
system:controller:endpoint-controller                                  2021-01-24T07:44:45Z
system:controller:endpointslice-controller                             2021-01-24T07:44:45Z
system:controller:endpointslicemirroring-controller                    2021-01-24T07:44:45Z
system:controller:expand-controller                                    2021-01-24T07:44:45Z
system:controller:generic-garbage-collector                            2021-01-24T07:44:45Z
system:controller:horizontal-pod-autoscaler                            2021-01-24T07:44:45Z
system:controller:job-controller                                       2021-01-24T07:44:45Z
system:controller:namespace-controller                                 2021-01-24T07:44:45Z
system:controller:node-controller                                      2021-01-24T07:44:45Z
system:controller:persistent-volume-binder                             2021-01-24T07:44:45Z
system:controller:pod-garbage-collector                                2021-01-24T07:44:45Z
system:controller:pv-protection-controller                             2021-01-24T07:44:45Z
system:controller:pvc-protection-controller                            2021-01-24T07:44:45Z
system:controller:replicaset-controller                                2021-01-24T07:44:45Z
system:controller:replication-controller                               2021-01-24T07:44:45Z
system:controller:resourcequota-controller                             2021-01-24T07:44:45Z
system:controller:root-ca-cert-publisher                               2021-01-24T07:44:45Z
system:controller:route-controller                                     2021-01-24T07:44:45Z
system:controller:service-account-controller                           2021-01-24T07:44:45Z
system:controller:service-controller                                   2021-01-24T07:44:45Z
system:controller:statefulset-controller                               2021-01-24T07:44:45Z
system:controller:ttl-controller                                       2021-01-24T07:44:45Z
system:coredns                                                         2021-01-24T07:44:48Z
system:discovery                                                       2021-01-24T07:44:45Z
system:heapster                                                        2021-01-24T07:44:45Z
system:kube-aggregator                                                 2021-01-24T07:44:45Z
system:kube-controller-manager                                         2021-01-24T07:44:45Z
system:kube-dns                                                        2021-01-24T07:44:45Z
system:kube-scheduler                                                  2021-01-24T07:44:45Z
system:kubelet-api-admin                                               2021-01-24T07:44:45Z
system:monitoring                                                      2021-01-24T07:44:45Z
system:node                                                            2021-01-24T07:44:45Z
system:node-bootstrapper                                               2021-01-24T07:44:45Z
system:node-problem-detector                                           2021-01-24T07:44:45Z
system:node-proxier                                                    2021-01-24T07:44:45Z
system:persistent-volume-provisioner                                   2021-01-24T07:44:45Z
system:public-info-viewer                                              2021-01-24T07:44:45Z
system:service-account-issuer-discovery                                2021-01-24T07:44:45Z
system:volume-scheduler                                                2021-01-24T07:44:45Z
view                                                                   2021-01-24T07:44:45Z
```

其中 system: 开头的是 k8s 内部需要使用的，不要去修改它。

### 查看 ClusterRoleBinding

```
[root@master manifests]# kubectl get clusterrolebindings.rbac.authorization.k8s.io
NAME                                                   ROLE                                                                               AGE
cluster-admin                                          ClusterRole/cluster-admin                                                          126d
flannel                                                ClusterRole/flannel                                                                70d
ingress-nginx                                          ClusterRole/ingress-nginx                                                          29d
ingress-nginx-admission                                ClusterRole/ingress-nginx-admission                                                29d
kubeadm:get-nodes                                      ClusterRole/kubeadm:get-nodes                                                      126d
kubeadm:kubelet-bootstrap                              ClusterRole/system:node-bootstrapper                                               126d
kubeadm:node-autoapprove-bootstrap                     ClusterRole/system:certificates.k8s.io:certificatesigningrequests:nodeclient       126d
kubeadm:node-autoapprove-certificate-rotation          ClusterRole/system:certificates.k8s.io:certificatesigningrequests:selfnodeclient   126d
kubeadm:node-proxier                                   ClusterRole/system:node-proxier                                                    126d
kubernetes-dashboard                                   ClusterRole/cluster-admin                                                          76d
run-nfs-client-provisioner                             ClusterRole/cluster-admin                                                          6h36m
system:basic-user                                      ClusterRole/system:basic-user                                                      126d
system:controller:attachdetach-controller              ClusterRole/system:controller:attachdetach-controller                              126d
system:controller:certificate-controller               ClusterRole/system:controller:certificate-controller                               126d
system:controller:clusterrole-aggregation-controller   ClusterRole/system:controller:clusterrole-aggregation-controller                   126d
system:controller:cronjob-controller                   ClusterRole/system:controller:cronjob-controller                                   126d
system:controller:daemon-set-controller                ClusterRole/system:controller:daemon-set-controller                                126d
system:controller:deployment-controller                ClusterRole/system:controller:deployment-controller                                126d
system:controller:disruption-controller                ClusterRole/system:controller:disruption-controller                                126d
system:controller:endpoint-controller                  ClusterRole/system:controller:endpoint-controller                                  126d
system:controller:endpointslice-controller             ClusterRole/system:controller:endpointslice-controller                             126d
system:controller:endpointslicemirroring-controller    ClusterRole/system:controller:endpointslicemirroring-controller                    126d
system:controller:expand-controller                    ClusterRole/system:controller:expand-controller                                    126d
system:controller:generic-garbage-collector            ClusterRole/system:controller:generic-garbage-collector                            126d
system:controller:horizontal-pod-autoscaler            ClusterRole/system:controller:horizontal-pod-autoscaler                            126d
system:controller:job-controller                       ClusterRole/system:controller:job-controller                                       126d
system:controller:namespace-controller                 ClusterRole/system:controller:namespace-controller                                 126d
system:controller:node-controller                      ClusterRole/system:controller:node-controller                                      126d
system:controller:persistent-volume-binder             ClusterRole/system:controller:persistent-volume-binder                             126d
system:controller:pod-garbage-collector                ClusterRole/system:controller:pod-garbage-collector                                126d
system:controller:pv-protection-controller             ClusterRole/system:controller:pv-protection-controller                             126d
system:controller:pvc-protection-controller            ClusterRole/system:controller:pvc-protection-controller                            126d
system:controller:replicaset-controller                ClusterRole/system:controller:replicaset-controller                                126d
system:controller:replication-controller               ClusterRole/system:controller:replication-controller                               126d
system:controller:resourcequota-controller             ClusterRole/system:controller:resourcequota-controller                             126d
system:controller:root-ca-cert-publisher               ClusterRole/system:controller:root-ca-cert-publisher                               126d
system:controller:route-controller                     ClusterRole/system:controller:route-controller                                     126d
system:controller:service-account-controller           ClusterRole/system:controller:service-account-controller                           126d
system:controller:service-controller                   ClusterRole/system:controller:service-controller                                   126d
system:controller:statefulset-controller               ClusterRole/system:controller:statefulset-controller                               126d
system:controller:ttl-controller                       ClusterRole/system:controller:ttl-controller                                       126d
system:coredns                                         ClusterRole/system:coredns                                                         126d
system:discovery                                       ClusterRole/system:discovery                                                       126d
system:kube-controller-manager                         ClusterRole/system:kube-controller-manager                                         126d
system:kube-dns                                        ClusterRole/system:kube-dns                                                        126d
system:kube-scheduler                                  ClusterRole/system:kube-scheduler                                                  126d
system:monitoring                                      ClusterRole/system:monitoring                                                      126d
system:node                                            ClusterRole/system:node                                                            126d
system:node-proxier                                    ClusterRole/system:node-proxier                                                    126d
system:public-info-viewer                              ClusterRole/system:public-info-viewer                                              126d
system:service-account-issuer-discovery                ClusterRole/system:service-account-issuer-discovery                                126d
system:volume-scheduler                                ClusterRole/system:volume-scheduler                                                126d
```

### 将 sa 绑定到集群角色

```
[root@master ~]# kubectl get sa admin -n kube-system
NAME    SECRETS   AGE
admin   1         9m42s
```

```yaml
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: admin-binding
subjects:
  - kind: ServiceAccount
    name: admin
    namespace: kube-system
roleRef:
  kind: ClusterRole
  name: cluster-admin
  apiGroup: rbac.authorization.k8s.io
```

## 总结

绑定了一个集群角色到 sa 上。

## 附录
