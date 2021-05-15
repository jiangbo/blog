# 【k8s】从文件生成 cm

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-M3

## 目标

之前学习直接通过 yaml 生成和修改 cm 的方式，接下来学习从文件生成 cm。

## 示例

### 准备文件

```
[root@master ~]# ls config/
test.json  test.properties
[root@master ~]# cat config/test.json
{
  "name": "jiangbo",
  "age": 44
}
[root@master ~]# cat config/test.properties
name=jiangbo
age=44
```

### 单个文件生成

```
[root@master ~]# kubectl create configmap configmap1 --from-file=config/test.json
configmap/configmap1 created
[root@master ~]# kubectl create configmap configmap2 --from-file=config/test.properties
configmap/configmap2 created
[root@master ~]# kubectl describe configmaps configmap1
Name:         configmap1
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

Events:  <none>
[root@master ~]# kubectl describe configmaps configmap2
Name:         configmap2
Namespace:    default
Labels:       <none>
Annotations:  <none>

Data
====
test.properties:
----
name=jiangbo
age=44

Events:  <none>
[root@master ~]#
```

### 多个文件生成

```
[root@master ~]# kubectl create configmap configmap3 --from-file=config/test.json --from-file=config/test.properties
configmap/configmap3 created
[root@master ~]# kubectl create configmap configmap4 --from-file=config/
configmap/configmap4 created
[root@master ~]# kubectl describe configmaps configmap3
Name:         configmap3
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
[root@master ~]# kubectl describe configmaps configmap4
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

```

## 总结

介绍了生成 cm 的方式。

## 附录
