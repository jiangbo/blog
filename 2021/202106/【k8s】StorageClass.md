# 【k8s】StorageClass

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.0-RC1

## 目标

在前面，如果创建 pvc 的话，必须需要一个对应的 pv 与之绑定，并且是提前准备好的。
而 StorageClass 则不比提前提供 pv，可以在需要的时候，动态申请，可以简写为 sc。

## 说明

每个 StorageClass 都包含 provisioner、parameters 和 reclaimPolicy 字段， 这些字段会在 StorageClass 需要动态分配 PersistentVolume 时会使用到。

StorageClass 对象的命名很重要，用户使用这个命名来请求生成一个特定的类。 当创建 StorageClass 对象时，管理员设置 StorageClass 对象的命名和其他参数，一旦创建了对象就不能再对其更新。

NFS 没有内部制备器，但可以使用外部制备器。 也有第三方存储供应商提供自己的外部制备器。

## 总结

介绍了存储类的概念。

## 附录
