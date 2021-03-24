# 【Kubernetes】启动后执行-HTTPGET

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M2

## 目标

在容器启动后，访问一个 HTTP 的 GET 请求。

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
        postStart:
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

```
[root@master ~]# kubectl get pod busybox -o jsonpath="{.spec.containers[0].lifecycle}" | jq
{
  "postStart": {
    "httpGet": {
      "host": "www.baidu.com",
      "path": "/",
      "port": 80,
      "scheme": "HTTP"
    }
  }
}

```

### 测试错误 GET

将端口从 80 修改为 8080，再次创建 Pod，查看 event 里的信息。

```
Events:
  Type     Reason               Age                From               Message
  ----     ------               ----               ----               -------
  Normal   Scheduled            80s                default-scheduler  Successfully assigned default/busybox to node2
  Warning  FailedPostStartHook  50s                kubelet            Http lifecycle hook (/) for Container "busybox" in Pod "busybox_default(b31715ff-55bc-4988-8b18-98804dd70eb3)" failed - error: Get "http://www.baidu.com:8080//": dial tcp 14.215.177.39:8080: i/o timeout, message: ""
  Normal   Killing              50s                kubelet            FailedPostStartHook
  Normal   Pulled               19s (x2 over 80s)  kubelet            Container image "busybox:stable" already present on machine
  Normal   Created              19s (x2 over 80s)  kubelet            Created container busybox
  Normal   Started              19s (x2 over 80s)  kubelet            Started container busybox
```

可以看到有一个失败的事件，证明 GET 请求没有成功。

## 总结

通过参与容器的生命周期，在容器启动后，在容器中执行了一个 HTTP 的 GET 请求。

## 附录
