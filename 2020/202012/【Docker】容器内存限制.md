# 【Docker】容器内存限制

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 内存限制

### 正常启动容器

```sh
[root@master ~]# docker run -d busybox sleep 3600
7f3df02708573bba3a03a1a637934030678881dd686cecb0413b30eec094272a
```

### 查看内存

```sh
[root@master ~]# docker stats 7f
CONTAINER ID   NAME             CPU %   MEM USAGE / LIMIT   MEM %   NET I/O      BLOCK I/O      PIDS
7f3df0270857   peaceful_beaver  0.00%   32KiB / 1.795GiB    0.00%   656B / 0B    0B / 0B        1
```

可以看到容器使用了宿主机所有的内存。

### 启动限制内存容器

```sh
[root@master ~]# docker run -d -m=32m busybox sleep 3600
1457a25d6c8660b03d678a9f43c1e5304ef53727c7ad984c9fb7fbdd4409c9aa
```

### 查看限制内存的容器

```sh
[root@master ~]# docker stats 14
CONTAINER ID   NAME                CPU %    MEM USAGE / LIMIT   MEM %   NET I/O       BLOCK I/O    PIDS
1457a25d6c86   heuristic_jackson   0.00%    32KiB / 32MiB       0.10%   656B / 0B     0B / 0B      1

```

## 总结

介绍了 Docker 的内存限制。