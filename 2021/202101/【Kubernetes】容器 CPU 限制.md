# 【Kubernetes】容器 CPU 限制

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

创建一个 Pod 限制其使用不同的 CPU 量，查看 Pod 的状态。

## 限制最小 CPU

一颗 CPU 等于 1000m。

### pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: pod-demo

spec:
  containers:
    - name: pod-demo
      image: jiangbo920827/spring-demo:actuator
      ports:
        - containerPort: 8080
      resources:
        requests:
          cpu: 50m

```

### 查看 CUP 请求限制

`kubectl get -f pod.yaml -o json | jq .spec.containers[].resources`

```json
{
  "requests": {
    "cpu": "50m"
  }
}
```

## 限制最大 CPU

### pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: pod-demo

spec:
  containers:
    - name: pod-demo
      image: jiangbo920827/spring-demo:actuator
      ports:
        - containerPort: 8080
      resources:
        limits:
          cpu: 50m

```

### 查看最大限制

`kubectl get -f pod.yaml -o json | jq .spec.containers[].resources`
只限制最大的话，请求量也会直接和最大相等。

```json
{
  "limits": {
    "cpu": "50m"
  },
  "requests": {
    "cpu": "50m"
  }
}
```

## CPU 不足

### pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: pod-demo

spec:
  containers:
    - name: pod-demo
      image: jiangbo920827/spring-demo:actuator
      ports:
        - containerPort: 8080
      resources:
        limits:
          cpu: 1m

```

### 查看结果

将 CPU 修改得足够小，也并不会让容器直接失败。只是会导致容器运行缓慢，和内存的硬性资源不一样，CPU 属于弹性资源。

```text
[root@master pod]# kubectl get -f pod.yaml -o wide
NAME       READY   STATUS    RESTARTS   AGE   IP            NODE    NOMINATED NODE   READINESS GATES
pod-demo   1/1     Running   0          18s   10.244.1.61   node1   <none>           <none>
```

## 总结

介绍了容器的 CPU 资源限制，可以限制容器请求量，也可以限制最大量。CPU 资源属于弹性资源，不会造成容器失败，但是会缓慢。

## 附录
