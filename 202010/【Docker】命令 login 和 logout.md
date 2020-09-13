# 【Docker】命令 login 和 logout

参考教程：https://docs.docker.com/engine/reference/commandline/login/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 命令格式

`docker login [OPTIONS] [SERVER]`
`docker logout [SERVER]`

使用 login 和 logout 命令可以登录和登出。

## 命令选项

| 名称 | 默认值 | 描述 |
| --- | --- | --- |
| `--password , -p` |  | 密码 |
| `--password-stdin` |  | 从标准输入流接收密码 |
| `--username , -u` |  | 用户名 |

## 示例

### 登录

```sh
[root@master ~]# docker login https://registry.hub.docker.com/
Username: jiangbo920827
Password:
Login Succeeded!
```

### 登出

```sh
[root@master ~]# docker logout https://registry.hub.docker.com/
Removed login credentials for registry.hub.docker.com/
[root@master ~]#
```

## 总结

介绍了 login/logout 命令的使用，可以进行登录和登出。
