# 【k8s】访问 api-server

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-RC1

## 目标

访问 api-server 的几种方式。

## 代理

```
[root@master ~]# kubectl proxy --port=8080 &
[1] 29975
[root@master ~]# Starting to serve on 127.0.0.1:8080
curl http://localhost:8080/api/
{
  "kind": "APIVersions",
  "versions": [
    "v1"
  ],
  "serverAddressByClientCIDRs": [
    {
      "clientCIDR": "0.0.0.0/0",
      "serverAddress": "192.168.56.101:6443"
    }
  ]
}
```

### 无代理

```
[root@master ~]# APISERVER=$(kubectl config view | grep server | cut -f 2- -d ":" | tr -d " ")
[root@master ~]# TOKEN=$(kubectl describe secret $(kubectl get secrets | grep default | cut -f1 -d ' ') | grep -E '^token' | cut -f2 -d':' | tr -d ' ')
[root@master ~]# curl $APISERVER/api --header "Authorization: Bearer $TOKEN" --insecure
{
  "kind": "APIVersions",
  "versions": [
    "v1"
  ],
  "serverAddressByClientCIDRs": [
    {
      "clientCIDR": "0.0.0.0/0",
      "serverAddress": "192.168.56.101:6443"
    }
  ]
}
```

### 证书

```
[root@master ~]# curl -k --cert /etc/kubernetes/pki/apiserver-kubelet-client.crt --key /etc/kubernetes/pki/apiserver-kubelet-client.key https://192.168.56.01:6443/api
{
  "kind": "APIVersions",
  "versions": [
    "v1"
  ],
  "serverAddressByClientCIDRs": [
    {
      "clientCIDR": "0.0.0.0/0",
      "serverAddress": "192.168.56.101:6443"
    }
  ]
}
```

### Pod 的 sa

```
[root@master ~]# kubectl exec spring-k8s -- sh -c 'curl -k -H "Authorization: Bearer $(cat /var/run/secrets/kubernetes.io/serviceaccount/token)" https://kubernetes.default.svc/api'
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   186  100   186    0     0   2188      0 --:--:-- --:--:-- --:--:--  2188
{
  "kind": "APIVersions",
  "versions": [
    "v1"
  ],
  "serverAddressByClientCIDRs": [
    {
      "clientCIDR": "0.0.0.0/0",
      "serverAddress": "192.168.56.101:6443"
    }
  ]
}
```

## 总结

介绍了几种访问 api-server 的方式。

## 附录
