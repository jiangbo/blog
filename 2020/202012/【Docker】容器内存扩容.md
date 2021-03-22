# 【Docker】容器内存扩容

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 内存更改

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

### 修改内存

```sh
[root@master ~]# docker update -m=64m 14
14
```

### 查看修改后内存

```sh
[root@master ~]# docker stats 14
CONTAINER ID  NAME               CPU %  MEM USAGE / LIMIT  MEM %  NET I/O    BLOCK I/O   PIDS
1457a25d6c86  heuristic_jackson  0.00%  32KiB / 64MiB      0.05%  656B / 0B  0B / 0B     1
```

## 总结

介绍了 Docker 的内存扩容。