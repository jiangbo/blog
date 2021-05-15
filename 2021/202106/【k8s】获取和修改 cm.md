# 【k8s】获取和修改 cm

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-M3

## 目标

学习获取和修改 cm 中数据的方式。

## 示例

### 获取 cm 中的全部数据

```
[root@master ~]# kubectl describe cm configmap4
Name:         configmap4
Namespace:    default
Labels:       <none>
Annotations:  <none>

Data
====
test.json:
----
{
  "name": "jiangbo",
  "age": 44
}

test.properties:
----
name=jiangbo
age=44

Events:  <none>
[root@master ~]# kubectl get cm configmap4 -o jsonpath='{.data.test\.json}'
{
  "name": "jiangbo",
  "age": 44
}
[root@master ~]# kubectl get cm configmap4 -o jsonpath='{.data.test\.properties}'
name=jiangbo
age=44
[root@master ~]#

```

### 获取 cm 中的单个数据

```
[root@master ~]# kubectl get cm configmap4 -o jsonpath='{.data.test\.json}' | jq -r .name
jiangbo
[root@master ~]# source <(kubectl get cm configmap4 -o jsonpath='{.data.test\.properties}');echo $name
jiangbo
```

### 修改 cm 中的数据 1

```
[root@master ~]# sed -i 's/age=44/age=4444/g' config/test.properties
[root@master ~]# kubectl create cm configmap4 --from-file=config -o yaml --dry-run=client | kubectl apply -f -
configmap/configmap4 configured
[root@master ~]# kubectl describe cm configmap4
Name:         configmap4
Namespace:    default
Labels:       <none>
Annotations:  <none>

Data
====
test.json:
----
{
  "name": "jiangbo",
  "age": 44
}

test.properties:
----
name=jiangbo
age=4444

Events:  <none>

```

### 修改 cm 中的数据 2

```
[root@master ~]# kubectl get cm configmap4 -o yaml |sed 's/age=44/age=4444/g' | kubectl apply -f -
configmap/configmap4 configured
[root@master ~]# kubectl describe cm configmap4
Name:         configmap4
Namespace:    default
Labels:       <none>
Annotations:  <none>

Data
====
test.json:
----
{
  "name": "jiangbo",
  "age": 44
}

test.properties:
----
name=jiangbo
age=4444

Events: <none>
```

## 总结

介绍了获取和修改 cm 的方式。

## 附录
