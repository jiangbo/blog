# 【Kubernetes】结束前执行-HTTPGET

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M2

## 目标

在容器结束前，访问一个 HTTP 的 GET 请求。

## 示例

### Pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: busybox
spec:
  containers:
    - name: busybox
      image: busybox:stable
      lifecycle:
        preStop:
          httpGet:
            scheme: HTTP
            port: 80
            host: www.baidu.com
      resources:
        limits:
          memory: "128Mi"
          cpu: "500m"
      command: ["sleep", "3600"]
```

### 查看

需要先使用 `kubectl delete pod busybox` 命令来触发删除 Pod 的操作。

```
[root@master ~]# kubectl get pod busybox -o jsonpath="{.spec.containers[0].lifecycle}" | jq
{
  "preStop": {
    "httpGet": {
      "host": "www.baidu.com",
      "path": "/",
      "port": 80,
      "scheme": "HTTP"
    }
  }
}
```

## 总结

通过参与容器的生命周期，在容器结束前，在容器中执行了一个 HTTP 的 GET 请求。

## 附录
