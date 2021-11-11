# 使用 openssl 生成 CA 证书

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.1
3. Centos 7

## 目标

使用 openssl 命令，生成 CA 证书。

## 查看软件版本

```text
[root@master k8s]# openssl version
OpenSSL 1.0.2k-fips  26 Jan 2017
```

### 生成 CA 私钥

```text
[root@master ssl]#  openssl genrsa -out ca.key
Generating RSA private key, 2048 bit long modulus
......................+++
............................................................+++
e is 65537 (0x10001)
[root@master ssl]#
```

### 查看 CA 私钥

```text
[root@master ssl]# cat ca.key
-----BEGIN RSA PRIVATE KEY-----
MIIEpQIBAAKCAQEA4n3obroNeycZPr78I8xcJTw/IPFvwxP456S1pc5PyoF74oqs
q7gjQg2dFHiJXExW4Or8Ap05PfbreWANxCFIwtK0qGl2NmlpldHvwN8qflEsEcxm
/KD5uPBDbUOqsd0Wx1/wlxEm44MR2QtsQr2bu4WeKm27adk43OJ2Dwj3nPVtZe0n
u6ekY3RVG++vi1bZztPurkiMx95rjrccA8Md2xlONIY4cFZJdY0EJJiu1KH9Rgyu
jnfxLYSoDFRWLDnL7tTBqbiDSxzUwh1Mg00yKpKMdTNhBi/scjYA+ftJOzbeHjjs
AGBJ/b4efHclpUlKoYWgcRSDw+OnPfrhDMzPkwIDAQABAoIBAQCnwvkKIxdfM/u4
BUC/x1wtXG2qGgtdZIrGEbNWh/WEGtrHV05Et1SuVWUXR5z+17C2Unet/ATtR9rt
AV/Q+ZJqdCXKZKth7Dx/84txwhs4iyloWY2i7B1shdJ7uVZS/l8j+IU9cD/s3yW2
LYbX/JfAk10vMi5SXcqHmdo2kr7N63uDdvFcQddZLVMaVK8IThMMpUt+MpXXBi7n
DxIRDe+QRhknVkaGOhLCoTHJmlyxthsnnNftbFd4Cn1ufAMMJYvlqa2F7XEjgxIG
PvUDqsVPX0vsOM/tt+FbuSjQQv46OBOuJ5bD/vI+aoyMknzV9w077gwY0wNLSJq2
004Nh0QhAoGBAPTfgU4VJNkEL/44aygF4TWEwjpxAaRrlG+KsG+R39YKJtKVUTXK
rs6kH39RfOwQ/KBL5vFoqy3kpmYD/PFbj+Irg9veLjilc//zXWBsfKd4ovaUWx6T
MQK4Nktb7jPeSNPKDFqT/rMb4ho11Rdt2g8MntoFsYfBvyctNc7loXHxAoGBAOzI
lRi2RID8opglooeG0ynJ4qmClqaUKH86j7A0T/e7/cxGcuh/X+6PnNJOeYfLKtHL
uSgEa91p5xgYsVViK/zz55TUlwsBMDQkuKNNyIPu9wUQ03hkDRlJYdASvMoHBn1Y
r9tLPjJNAtTFHzaocHygk24F12p9g/WEvjtJ3lXDAoGBANhdQFBw6E0EoS9cRHpp
r5NtSXz/6vaPw+lGvxzl5MSn1aPOIaKJAyo85EPICbxASieMyXNXC6GSXeDX90B/
qcOqtWipiWet8bAfJcuUMBR9yGhdr6F1ZyHDCN5PTUrxRqIz7fR+vdcedQq5tB5u
hvUzLozv6OxOKguArAGy7pLRAoGAJeogZaqZ4YWSi4ZLNiRSn7hiAP6WHOpQab74
zyAxRdvmUdVeBPiQZeS9GbfTIflYS+uxJEVA+F1C0cryalej74cmp+A6p1pzW4Yd
wphr6SRUjY0aL4blygzECstVS4xAb8XJsvWjae6G64b87oWeLXaQ5OcujTC39pJg
26GCZn0CgYEA1dun4+NXk8ntLkm5Y60hji02AYb2as53dj9Mi3IaEMoCvj1z16Hd
4IszCke6GbJvP4gLek1cfBGeCbngRHjAUqeMtkZhCqF8BROyaL0WBBLz0Qmqwt6L
Yy71Hj7VoGQabEuG+sotsxyIpH3Iwdi2+0Pofb9a+yx936U7yjrsJa0=
-----END RSA PRIVATE KEY-----
```

### 生成 CA 证书

```text
[root@master ssl]# openssl req -x509 -new -key ca.key -subj "/CN=JiangBo/C=CN/ST=ChongQing/L=ChongQing/O=jiangbo" -days 100000 -out ca.crt
[root@master ssl]# ll
total 8
-rw-r--r-- 1 root root 1285 Jun 14 01:32 ca.crt
-rw-r--r-- 1 root root 1679 Jun 14 01:27 ca.key
```

### 查看 CA 证书

```text
[root@master ssl]# cat ca.crt
-----BEGIN CERTIFICATE-----
MIIDhzCCAm+gAwIBAgIJAPhOv6nUJwCxMA0GCSqGSIb3DQEBCwUAMFkxEDAOBgNV
BAMMB0ppYW5nQm8xCzAJBgNVBAYTAkNOMRIwEAYDVQQIDAlDaG9uZ1FpbmcxEjAQ
BgNVBAcMCUNob25nUWluZzEQMA4GA1UECgwHamlhbmdibzAgFw0yMTA2MTMxNzMy
MjVaGA8yMjk1MDMyOTE3MzIyNVowWTEQMA4GA1UEAwwHSmlhbmdCbzELMAkGA1UE
BhMCQ04xEjAQBgNVBAgMCUNob25nUWluZzESMBAGA1UEBwwJQ2hvbmdRaW5nMRAw
DgYDVQQKDAdqaWFuZ2JvMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA
4n3obroNeycZPr78I8xcJTw/IPFvwxP456S1pc5PyoF74oqsq7gjQg2dFHiJXExW
4Or8Ap05PfbreWANxCFIwtK0qGl2NmlpldHvwN8qflEsEcxm/KD5uPBDbUOqsd0W
x1/wlxEm44MR2QtsQr2bu4WeKm27adk43OJ2Dwj3nPVtZe0nu6ekY3RVG++vi1bZ
ztPurkiMx95rjrccA8Md2xlONIY4cFZJdY0EJJiu1KH9RgyujnfxLYSoDFRWLDnL
7tTBqbiDSxzUwh1Mg00yKpKMdTNhBi/scjYA+ftJOzbeHjjsAGBJ/b4efHclpUlK
oYWgcRSDw+OnPfrhDMzPkwIDAQABo1AwTjAdBgNVHQ4EFgQUrC22x2uF0WoWepbP
r/i/f+tBAckwHwYDVR0jBBgwFoAUrC22x2uF0WoWepbPr/i/f+tBAckwDAYDVR0T
BAUwAwEB/zANBgkqhkiG9w0BAQsFAAOCAQEAJdxHrajTzLaFsLh+1rJniexKKtT+
GFuzd6B99KvhMoGmwzKKCnFob1ExOmQnC5DTaP6Px8p8TmdzRwWDRp/QRQ1Xe2+v
RbJMDrL02o4olZk50PvkfgTNQ/LaFBthkrjHKQkmhvVz4JocKpJ9tL/4qO0PGbFc
vJO4SgLxXm0+1hG6sQcm8vLgjwxrKLWQmcWzFL0i9Y+N8q3xKEQrMxXndgk+h2yM
8DKrCAfbY2SkmRqz1OMJw9XHMuPhTA34jko2esa38EF7NPY7JwBi8uBbbwjqJ8k7
jXDaf5/C6HwRkND6abHtLVlIxLO9ndTniA23LC1/xuyak/uQIxJFP62rFw==
-----END CERTIFICATE-----
```

### 查看 CA 证书详情

```text
[root@master ssl]# openssl x509 -noout -text -in ca.crt
Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            f8:4e:bf:a9:d4:27:00:b1
    Signature Algorithm: sha256WithRSAEncryption
        Issuer: CN=JiangBo, C=CN, ST=ChongQing, L=ChongQing, O=jiangbo
        Validity
            Not Before: Jun 13 17:32:25 2021 GMT
            Not After : Mar 29 17:32:25 2295 GMT
        Subject: CN=JiangBo, C=CN, ST=ChongQing, L=ChongQing, O=jiangbo
        Subject Public Key Info:
            Public Key Algorithm: rsaEncryption
                Public-Key: (2048 bit)
                Modulus:
                    00:e2:7d:e8:6e:ba:0d:7b:27:19:3e:be:fc:23:cc:
                    5c:25:3c:3f:20:f1:6f:c3:13:f8:e7:a4:b5:a5:ce:
                    4f:ca:81:7b:e2:8a:ac:ab:b8:23:42:0d:9d:14:78:
                    89:5c:4c:56:e0:ea:fc:02:9d:39:3d:f6:eb:79:60:
                    0d:c4:21:48:c2:d2:b4:a8:69:76:36:69:69:95:d1:
                    ef:c0:df:2a:7e:51:2c:11:cc:66:fc:a0:f9:b8:f0:
                    43:6d:43:aa:b1:dd:16:c7:5f:f0:97:11:26:e3:83:
                    11:d9:0b:6c:42:bd:9b:bb:85:9e:2a:6d:bb:69:d9:
                    38:dc:e2:76:0f:08:f7:9c:f5:6d:65:ed:27:bb:a7:
                    a4:63:74:55:1b:ef:af:8b:56:d9:ce:d3:ee:ae:48:
                    8c:c7:de:6b:8e:b7:1c:03:c3:1d:db:19:4e:34:86:
                    38:70:56:49:75:8d:04:24:98:ae:d4:a1:fd:46:0c:
                    ae:8e:77:f1:2d:84:a8:0c:54:56:2c:39:cb:ee:d4:
                    c1:a9:b8:83:4b:1c:d4:c2:1d:4c:83:4d:32:2a:92:
                    8c:75:33:61:06:2f:ec:72:36:00:f9:fb:49:3b:36:
                    de:1e:38:ec:00:60:49:fd:be:1e:7c:77:25:a5:49:
                    4a:a1:85:a0:71:14:83:c3:e3:a7:3d:fa:e1:0c:cc:
                    cf:93
                Exponent: 65537 (0x10001)
        X509v3 extensions:
            X509v3 Subject Key Identifier:
                AC:2D:B6:C7:6B:85:D1:6A:16:7A:96:CF:AF:F8:BF:7F:EB:41:01:C9
            X509v3 Authority Key Identifier:
                keyid:AC:2D:B6:C7:6B:85:D1:6A:16:7A:96:CF:AF:F8:BF:7F:EB:41:01:C9

            X509v3 Basic Constraints:
                CA:TRUE
    Signature Algorithm: sha256WithRSAEncryption
         25:dc:47:ad:a8:d3:cc:b6:85:b0:b8:7e:d6:b2:67:89:ec:4a:
         2a:d4:fe:18:5b:b3:77:a0:7d:f4:ab:e1:32:81:a6:c3:32:8a:
         0a:71:68:6f:51:31:3a:64:27:0b:90:d3:68:fe:8f:c7:ca:7c:
         4e:67:73:47:05:83:46:9f:d0:45:0d:57:7b:6f:af:45:b2:4c:
         0e:b2:f4:da:8e:28:95:99:39:d0:fb:e4:7e:04:cd:43:f2:da:
         14:1b:61:92:b8:c7:29:09:26:86:f5:73:e0:9a:1c:2a:92:7d:
         b4:bf:f8:a8:ed:0f:19:b1:5c:bc:93:b8:4a:02:f1:5e:6d:3e:
         d6:11:ba:b1:07:26:f2:f2:e0:8f:0c:6b:28:b5:90:99:c5:b3:
         14:bd:22:f5:8f:8d:f2:ad:f1:28:44:2b:33:15:e7:76:09:3e:
         87:6c:8c:f0:32:ab:08:07:db:63:64:a4:99:1a:b3:d4:e3:09:
         c3:d5:c7:32:e3:e1:4c:0d:f8:8e:4a:36:7a:c6:b7:f0:41:7b:
         34:f6:3b:27:00:62:f2:e0:5b:6f:08:ea:27:c9:3b:8d:70:da:
         7f:9f:c2:e8:7c:11:90:d0:fa:69:b1:ed:2d:59:48:c4:b3:bd:
         9d:d4:e7:88:0d:b7:2c:2d:7f:c6:ec:9a:93:fb:90:23:12:45:
         3f:ad:ab:17
```

## 总结

介绍了使用 openssl 命令生成 CA 私钥和证书的方式。

## 附录
