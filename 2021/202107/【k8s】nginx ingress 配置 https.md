# 【k8s】nginx ingress 配置 https

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.1

## 目标

在 nginx ingress 中配置 https 访问。

## ingress.yaml

其中的 secretName 参考之前的创建 secret 方法。

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: spring-k8s
spec:
  tls:
    - hosts:
        - www.jiang.bo
      secretName: nginx-tls
  rules:
    - host: "www.jiang.bo"
      http:
        paths:
          - path: /hostname
            pathType: Prefix
            backend:
              service:
                name: spring-k8s
                port:
                  number: 80
```

## Pod 信息

```text
[root@master ~]# kubectl get pod -o wide
NAME                          READY   STATUS    RESTARTS   AGE   IP             NODE    NOMINATED NODE   READINESS GATES
spring-k8s-79f74b55d7-862q6   1/1     Running   6          66d   10.244.2.240   node2   <none>           <none>
[root@master ~]# curl 10.244.2.240:8080/hostname;echo
spring-k8s-79f74b55d7-862q6
```

### 浏览器访问

![ingress-https.png][1]

## 总结

介绍了 nginx ingress 配置 https 的步骤，并且访问后端应用。

## 附录

[1]:images/ingress-https.png
