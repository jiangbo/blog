# Windows 和 Centos 导入 CA 证书

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.1
3. Centos 7

## 目标

在 Windows 和 Centos 上导入 CA 证书，服务器证书则默认变成可信的。

## Windows 安装证书

1. 将 ca.crt 放到 windows 中时，双击安装。
2. 双击后，在最下面能看到安装证书的按钮，然后点击。
3. 存储位置选择本地计算机，然后点击下一页。
4. 选择“将所有证书都放入下列存储”，然后点击浏览。
5. 选择“受信任的根证书颁发机构”，然后确定。
6. 下一页点击完成，应该能看到成功导入证书的弹出框。

### Centos 7 安装根证书

```sh
cp ca.crt /etc/pki/ca-trust/source/anchors
update-ca-trust
```

## 总结

介绍了在 Windows 和 Centos 上导入 CA 证书的步骤。

## 附录
