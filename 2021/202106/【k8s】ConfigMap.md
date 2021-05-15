# 【k8s】ConfigMap

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-M3

## 目标

ConfigMap 是 k8s 用来解耦应用和配置信息的，可以简写为 cm。

## 示例

### ConfigMap.yaml

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: cm1
data:
  name: jiangbo
  age: "44"
```

### 查看

```
[root@master ~]# kubectl get cm cm1
NAME   DATA   AGE
cm1    2      3s
[root@master ~]# kubectl describe cm cm1
Name:         cm1
Namespace:    default
Labels:       <none>
Annotations:  <none>

Data
====
age:
----
44
name:
----
jiangbo
Events:  <none>
```

### 修改

如果还包含有生成 cm 的 yaml，则可以直接修改文件，然后使用 `kubectl apply` 命令。

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: cm1
data:
  name: jiangbo
  age: "4444"
```

```
[root@master ~]# kubectl apply -f cm1.yaml
configmap/cm1 configured
[root@master ~]# kubectl describe cm cm1
Name:         cm1
Namespace:    default
Labels:       <none>
Annotations:  <none>

Data
====
age:
----
4444
name:
----
jiangbo
Events:  <none>
```

## 总结

ConfigMap 可以生成配置信息。

## 附录
