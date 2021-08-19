# 【k8s】跨 Namespace 使用 Ingress

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.1

## 目标

在 Ingress 中访问不同命名空间下的服务。

## 示例

### 使用 ExternalName Service

```yaml
kind: Service
apiVersion: v1
metadata:
  name: dashboard
spec:
  type: ExternalName
  externalName: kubernetes-dashboard.kube-system.svc.cluster.local
```

### ingress.yaml

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: nginx-ingress
  annotations:
    nginx.ingress.kubernetes.io/rewrite-target: /$2
spec:
  tls:
    - hosts:
        - www.jiang.bo
      secretName: nginx-tls

  rules:
    - host: "www.jiang.bo"
      http:
        paths:
          - path: /spring(/|$)(.*)
            pathType: Prefix
            backend:
              service:
                name: spring-k8s
                port:
                  number: 80
          - path: /dashboard(/|$)(.*)
            pathType: Prefix
            backend:
              service:
                name: dashboard
                port:
                  number: 80
```

## 总结

介绍了跨命名空间的 Ingress 的配置。

## 附录
