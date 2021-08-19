# 【k8s】通过 Ingress https 访问 dashboard

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.1

## 目标

在 Ingress 中配置 dashboard 的 https 请求转发。

## 示例

### 删除 NodePort

之前在 Service 中配置了 NodePort，现在通过 Ingress 访问，所以直接 ClusterIP 就可以了。

```yaml
kind: Service
apiVersion: v1
metadata:
  labels:
    k8s-app: kubernetes-dashboard
  name: kubernetes-dashboard
  namespace: kube-system
spec:
  ports:
    - port: 80
      targetPort: http
  selector:
    k8s-app: kubernetes-dashboard
```

### 删除 dashboard 的 https

因为可以直接在 Ingress 上配置 https，所以去除自带的 https，并且允许不安全登录。

```yaml
            - --enable-insecure-login
            # - --auto-generate-certificates
```

### 配置 Ingress

dashboard 我放到 kube-system 命名空间下的，这里可以根据实际情况修改。

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: dashboard-ingress
  namespace: kube-system
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
          - path: /dashboard(/|$)(.*)
            pathType: Prefix
            backend:
              service:
                name: kubernetes-dashboard
                port:
                  number: 80
```

### 浏览器访问验证

![dash-https.png][1]

## 总结

介绍了使用 Ingress 的 https 访问 dashboard 的方式。

## 附录

### dashboard.yaml

```yaml
apiVersion: v1
kind: Secret
metadata:
  labels:
    k8s-app: kubernetes-dashboard
  name: kubernetes-dashboard-csrf
  namespace: kube-system
type: Opaque
data:
  csrf: ""
---
kind: Service
apiVersion: v1
metadata:
  labels:
    k8s-app: kubernetes-dashboard
  name: kubernetes-dashboard
  namespace: kube-system
spec:
  ports:
    - port: 80
      targetPort: http
  selector:
    k8s-app: kubernetes-dashboard
---
kind: ConfigMap
apiVersion: v1
metadata:
  labels:
    k8s-app: kubernetes-dashboard
  name: kubernetes-dashboard-settings
  namespace: kube-system

---
kind: Deployment
apiVersion: apps/v1
metadata:
  labels:
    k8s-app: kubernetes-dashboard
  name: kubernetes-dashboard
  namespace: kube-system
spec:
  selector:
    matchLabels:
      k8s-app: kubernetes-dashboard
  template:
    metadata:
      labels:
        k8s-app: kubernetes-dashboard
    spec:
      containers:
        - name: kubernetes-dashboard
          image: kubernetesui/dashboard:v2.3.0
          ports:
            - containerPort: 9090
              name: http
          args:
            - --enable-insecure-login
            - --enable-skip-login
          volumeMounts:
            - mountPath: /tmp
              name: tmp-volume
          livenessProbe:
            httpGet:
              path: /
              port: 9090
            initialDelaySeconds: 30
            timeoutSeconds: 30
      volumes:
        - name: tmp-volume
          emptyDir: {}
      serviceAccountName: admin

---
kind: Service
apiVersion: v1
metadata:
  labels:
    k8s-app: dashboard-metrics-scraper
  name: dashboard-metrics-scraper
  namespace: kube-system
spec:
  ports:
    - port: 8000
      targetPort: 8000
  selector:
    k8s-app: dashboard-metrics-scraper

---
kind: Deployment
apiVersion: apps/v1
metadata:
  labels:
    k8s-app: dashboard-metrics-scraper
  name: dashboard-metrics-scraper
  namespace: kube-system
spec:
  selector:
    matchLabels:
      k8s-app: dashboard-metrics-scraper
  template:
    metadata:
      labels:
        k8s-app: dashboard-metrics-scraper
      annotations:
        seccomp.security.alpha.kubernetes.io/pod: "runtime/default"
    spec:
      containers:
        - name: dashboard-metrics-scraper
          image: kubernetesui/metrics-scraper:v1.0.6
          ports:
            - containerPort: 8000
          livenessProbe:
            httpGet:
              path: /
              port: 8000
            initialDelaySeconds: 30
            timeoutSeconds: 30
          volumeMounts:
            - mountPath: /tmp
              name: tmp-volume
      serviceAccountName: admin
      volumes:
        - name: tmp-volume
          emptyDir: {}
```

[1]:images/dash-https.png
