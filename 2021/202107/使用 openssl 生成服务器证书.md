# 使用 openssl 生成服务器证书

## 环境

1. kubernetes 1.20.6
2. Spring Boot 2.5.1
3. Centos 7

## 目标

使用 openssl 命令，生成服务器证书，这一节是基于上一节，默认已经存在 CA 私钥和证书。

## 查看软件版本

```
[root@master k8s]# openssl version
OpenSSL 1.0.2k-fips  26 Jan 2017
```

### 生成服务器私钥

```
[root@master ssl]# openssl genrsa -out server.key
Generating RSA private key, 2048 bit long modulus
........................................................+++
........................+++
e is 65537 (0x10001)
```

### 查看服务器私钥

```
[root@master ssl]# cat server.key
-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEAnubhdyxuh1HPkXPVGZ3vBSOXyylYiZ359lHcjqL5hNUxpiWD
tQCVuYmJ2Dsatbsv9RR6CIOstMJ1pP79LYF9bcQ4bW2z9ttdhAMl2PCEz2D04h0W
igzDiwXbvltA8Mk9EoKCPzIUpqly0haV0TSP6z9Cg2jJaecPYAnGPlajfmRKyBE0
ovrvtAG4iSenHgz5TXsVzf4su4w6lnk1SIy86ZXfaXt/Sn0CQbm6mUQmUfPv4m4m
BmJWH4vWHEh/pmui3AJON7HoFgvUnPRLyb2Pe7izqrFHzVyF1/c4qQ2sIT4q8IWk
OKDaA/It8xwVQool+51cHA95xePkpiv66996QwIDAQABAoIBAQCGpgI8KAxo4mxP
RqY3FNtrsWN5Jkn+Iz2i+m849uUVKErwDS3z/NDi7ac+8aaKhFn5BSb7fAinC3kB
HYmQO//7OaDPKXysG+q8aKZzlCwWRoR77kYhKPuyHH6+aTk4UG5N9wVDY76YE1JW
sAi3xCCIKYzFKdAyyzb+HIo+XNAh0qhbvnwpkGrD4V8dX4b588P+GcG6d3oYECeS
A8mf3PaE6Z38VkvJLizLh5i0ry+O8hOKwpT2CESTfyr6YeSeoLSfL5OlX8VUpfym
qAeEFeRf8W4twKSDOAkgbKG7qoJ0TZmh4r744Hre/jJu6bpjKOy6ins4rK1166vw
SJaz0CNBAoGBANLgRCtK65UAtj8n1ECx8PL2xsGBBGx5Z6Pzfou60/qSL37zqqLh
AZns/2eQAms9SHYHUgRCudFp8QLQIiLg3qDil5aQAbv6gDeDDP44r1AlckokLy2U
MAbpb4YtIVXzeNfSWPHagIkIrDSz4lJzgqOcKJ9DqJJIkE47Y4gWGxWzAoGBAMDn
e8tEsjnQEYiimPEWTwS8L3nZ2rl77ho3uLdic5eU4nfZuiS0hTegE99+wGW+ZQuf
blW9zrGg1ZIBqS7+fByXefQaeX84XBStF0fnwARlosFDP2NrDvRnzjUyp3KI0rY4
spRZuMgZ1NCvWBsXXornSE9D69AkkH4kZfg/6uExAoGAZ51+oW8ctog1BG9Doc5N
QHT1UFtemxZEUJDyWyIfyn1ODSkZbRL2QHXtQSvHpH3mPuh16lYHC9jsbZ9rRowL
mC3+oiJ8V6tMmPoxxjCAI/diYv/IlrK7ronBHMLMIHmTpShcP+pHQg4a34KL8EuF
o3K1mxwZlldFYMtZqqKCCHUCgYAH80Jh3UHvKgke9XySKxY0nsDiJ71g+Z+Viwhc
3nzfGW6/+8Q1MQ9wfHN1OZcaWl91vCti80Cqh3Nl4hWj7y8USpXnHpQ1TxBUddHI
z99VDw9HIctiAI6MOh8AiEbobIUqp2mMvank7VOdCiz6aNsnpGKRib4VRPyDhmKB
ebjncQKBgQCRbwKfZd40iX4f7O5LC7+fPIPmJzyxu7VTqqyP4Wm/tScGBON1+OJD
mvVgy+eW7TmWAGbNv0QaREiHcj+7o/vG08yXLeLogarieuQfCZvIujVweJqrNy8o
sjF8d1m5ndqywnT3/M7yrKiijsbZIrAsmMAKMaEUiwnY6Jt8rXwl8g==
-----END RSA PRIVATE KEY-----
```

### 生成证书请求

Subject 中的信息需要和 CA 中的一致，CN 表示需要使用的域名，带 * 表示是一个泛域名。

```
[root@master ssl]# openssl req -key server.key -new  -out server.csr  -subj '/CN=*.jiangbo.com/O=jiangbo/ST=ChongQing/L=ChongQing/C=CN'
[root@master ssl]# cat server.csr
-----BEGIN CERTIFICATE REQUEST-----
MIICpDCCAYwCAQAwXzEWMBQGA1UEAwwNKi5qaWFuZ2JvLmNvbTEQMA4GA1UECgwH
amlhbmdibzESMBAGA1UECAwJQ2hvbmdRaW5nMRIwEAYDVQQHDAlDaG9uZ1Fpbmcx
CzAJBgNVBAYTAkNOMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAnubh
dyxuh1HPkXPVGZ3vBSOXyylYiZ359lHcjqL5hNUxpiWDtQCVuYmJ2Dsatbsv9RR6
CIOstMJ1pP79LYF9bcQ4bW2z9ttdhAMl2PCEz2D04h0WigzDiwXbvltA8Mk9EoKC
PzIUpqly0haV0TSP6z9Cg2jJaecPYAnGPlajfmRKyBE0ovrvtAG4iSenHgz5TXsV
zf4su4w6lnk1SIy86ZXfaXt/Sn0CQbm6mUQmUfPv4m4mBmJWH4vWHEh/pmui3AJO
N7HoFgvUnPRLyb2Pe7izqrFHzVyF1/c4qQ2sIT4q8IWkOKDaA/It8xwVQool+51c
HA95xePkpiv66996QwIDAQABoAAwDQYJKoZIhvcNAQELBQADggEBAFy8uIkW+ixn
qsiB7WyOtDJXca79BbMV/B50mMQhk5PzFg3eXjwrSR/BvupzdVYJlU9gnp+SV7R0
ZhP8njRPQWgH8QzNKUtIZ/79Uy6La9Thm7f8pScpK6oDMDnT14C/XUod2q/z3gRb
5qprZy/lNPm/uoU7cP71SSobGbLMCkgHWoVCKmGPzvYYBFcyOA36lUBz9PR+A5G9
aXlWMJ5SqtRONNJiAwFwquvna99vPQRpgLORfLmWzUVv1sCdD23qzFIP0zETYa74
/1lfd/WGaGdWmF5iB+qsJLgvHZ5zZKB8knodfI5ON0WB1A7BV5xiG6JNS3x8WpJM
tTssKtjkzSU=
-----END CERTIFICATE REQUEST-----
```

### 查看证书请求

```
[root@master ssl]# openssl req -noout -text -in server.csr
Certificate Request:
    Data:
        Version: 0 (0x0)
        Subject: CN=*.jiangbo.com, O=jiangbo, ST=ChongQing, L=ChongQing, C=CN
        Subject Public Key Info:
            Public Key Algorithm: rsaEncryption
                Public-Key: (2048 bit)
                Modulus:
                    00:9e:e6:e1:77:2c:6e:87:51:cf:91:73:d5:19:9d:
                    ef:05:23:97:cb:29:58:89:9d:f9:f6:51:dc:8e:a2:
                    f9:84:d5:31:a6:25:83:b5:00:95:b9:89:89:d8:3b:
                    1a:b5:bb:2f:f5:14:7a:08:83:ac:b4:c2:75:a4:fe:
                    fd:2d:81:7d:6d:c4:38:6d:6d:b3:f6:db:5d:84:03:
                    25:d8:f0:84:cf:60:f4:e2:1d:16:8a:0c:c3:8b:05:
                    db:be:5b:40:f0:c9:3d:12:82:82:3f:32:14:a6:a9:
                    72:d2:16:95:d1:34:8f:eb:3f:42:83:68:c9:69:e7:
                    0f:60:09:c6:3e:56:a3:7e:64:4a:c8:11:34:a2:fa:
                    ef:b4:01:b8:89:27:a7:1e:0c:f9:4d:7b:15:cd:fe:
                    2c:bb:8c:3a:96:79:35:48:8c:bc:e9:95:df:69:7b:
                    7f:4a:7d:02:41:b9:ba:99:44:26:51:f3:ef:e2:6e:
                    26:06:62:56:1f:8b:d6:1c:48:7f:a6:6b:a2:dc:02:
                    4e:37:b1:e8:16:0b:d4:9c:f4:4b:c9:bd:8f:7b:b8:
                    b3:aa:b1:47:cd:5c:85:d7:f7:38:a9:0d:ac:21:3e:
                    2a:f0:85:a4:38:a0:da:03:f2:2d:f3:1c:15:42:8a:
                    25:fb:9d:5c:1c:0f:79:c5:e3:e4:a6:2b:fa:eb:df:
                    7a:43
                Exponent: 65537 (0x10001)
        Attributes:
            a0:00
    Signature Algorithm: sha256WithRSAEncryption
         5c:bc:b8:89:16:fa:2c:67:aa:c8:81:ed:6c:8e:b4:32:57:71:
         ae:fd:05:b3:15:fc:1e:74:98:c4:21:93:93:f3:16:0d:de:5e:
         3c:2b:49:1f:c1:be:ea:73:75:56:09:95:4f:60:9e:9f:92:57:
         b4:74:66:13:fc:9e:34:4f:41:68:07:f1:0c:cd:29:4b:48:67:
         fe:fd:53:2e:8b:6b:d4:e1:9b:b7:fc:a5:27:29:2b:aa:03:30:
         39:d3:d7:80:bf:5d:4a:1d:da:af:f3:de:04:5b:e6:aa:6b:67:
         2f:e5:34:f9:bf:ba:85:3b:70:fe:f5:49:2a:1b:19:b2:cc:0a:
         48:07:5a:85:42:2a:61:8f:ce:f6:18:04:57:32:38:0d:fa:95:
         40:73:f4:f4:7e:03:91:bd:69:79:56:30:9e:52:aa:d4:4e:34:
         d2:62:03:01:70:aa:eb:e7:6b:df:6f:3d:04:69:80:b3:91:7c:
         b9:96:cd:45:6f:d6:c0:9d:0f:6d:ea:cc:52:0f:d3:31:13:61:
         ae:f8:ff:59:5f:77:f5:86:68:67:56:98:5e:62:07:ea:ac:24:
         b8:2f:1d:9e:73:64:a0:7c:92:7a:1d:7c:8e:4e:37:45:81:d4:
         0e:c1:57:9c:62:1b:a2:4d:4b:7c:7c:5a:92:4c:b5:3b:2c:2a:
         d8:e4:cd:25
```

### CA 签发证书

先定义一个 extfile.ext 扩展文件，内容如下，签发时需要使用

subjectAltName 表示备用域名

```
basicConstraints=CA:FALSE, pathlen:0
extendedKeyUsage=serverAuth
subjectAltName=DNS:jiangbo.com,DNS:www.jiangbo.com
```

```
[root@master ssl]# openssl ca  -days 100000 -in server.csr -cert ca.crt -keyfile ca.key  -out server.crt -extfile extfile.ext
Using configuration from /etc/pki/tls/openssl.cnf
Check that the request matches the signature
Signature ok
Certificate Details:
        Serial Number: 5 (0x5)
        Validity
            Not Before: Jun 13 17:50:17 2021 GMT
            Not After : Mar 29 17:50:17 2295 GMT
        Subject:
            countryName               = CN
            stateOrProvinceName       = ChongQing
            organizationName          = jiangbo
            commonName                = *.jiangbo.com
        X509v3 extensions:
            X509v3 Basic Constraints:
                CA:FALSE, pathlen:0
            X509v3 Extended Key Usage:
                TLS Web Server Authentication
            X509v3 Subject Alternative Name:
                DNS:jiangbo.com, DNS:www.jiangbo.com
Certificate is to be certified until Mar 29 17:50:17 2295 GMT (100000 days)
Sign the certificate? [y/n]:y


1 out of 1 certificate requests certified, commit? [y/n]y
Write out database with 1 new entries
Data Base Updated
```

### 查看服务器证书

```
[root@master ssl]# openssl x509 -noout -text -in server.crt
Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number: 5 (0x5)
    Signature Algorithm: sha256WithRSAEncryption
        Issuer: CN=JiangBo, C=CN, ST=ChongQing, L=ChongQing, O=jiangbo
        Validity
            Not Before: Jun 13 17:50:17 2021 GMT
            Not After : Mar 29 17:50:17 2295 GMT
        Subject: C=CN, ST=ChongQing, O=jiangbo, CN=*.jiangbo.com
        Subject Public Key Info:
            Public Key Algorithm: rsaEncryption
                Public-Key: (2048 bit)
                Modulus:
                    00:9e:e6:e1:77:2c:6e:87:51:cf:91:73:d5:19:9d:
                    ef:05:23:97:cb:29:58:89:9d:f9:f6:51:dc:8e:a2:
                    f9:84:d5:31:a6:25:83:b5:00:95:b9:89:89:d8:3b:
                    1a:b5:bb:2f:f5:14:7a:08:83:ac:b4:c2:75:a4:fe:
                    fd:2d:81:7d:6d:c4:38:6d:6d:b3:f6:db:5d:84:03:
                    25:d8:f0:84:cf:60:f4:e2:1d:16:8a:0c:c3:8b:05:
                    db:be:5b:40:f0:c9:3d:12:82:82:3f:32:14:a6:a9:
                    72:d2:16:95:d1:34:8f:eb:3f:42:83:68:c9:69:e7:
                    0f:60:09:c6:3e:56:a3:7e:64:4a:c8:11:34:a2:fa:
                    ef:b4:01:b8:89:27:a7:1e:0c:f9:4d:7b:15:cd:fe:
                    2c:bb:8c:3a:96:79:35:48:8c:bc:e9:95:df:69:7b:
                    7f:4a:7d:02:41:b9:ba:99:44:26:51:f3:ef:e2:6e:
                    26:06:62:56:1f:8b:d6:1c:48:7f:a6:6b:a2:dc:02:
                    4e:37:b1:e8:16:0b:d4:9c:f4:4b:c9:bd:8f:7b:b8:
                    b3:aa:b1:47:cd:5c:85:d7:f7:38:a9:0d:ac:21:3e:
                    2a:f0:85:a4:38:a0:da:03:f2:2d:f3:1c:15:42:8a:
                    25:fb:9d:5c:1c:0f:79:c5:e3:e4:a6:2b:fa:eb:df:
                    7a:43
                Exponent: 65537 (0x10001)
        X509v3 extensions:
            X509v3 Basic Constraints:
                CA:FALSE, pathlen:0
            X509v3 Extended Key Usage:
                TLS Web Server Authentication
            X509v3 Subject Alternative Name:
                DNS:jiangbo.com, DNS:www.jiangbo.com
    Signature Algorithm: sha256WithRSAEncryption
         9e:c3:b5:11:20:41:58:89:dd:a8:af:be:50:ef:9f:40:a7:6f:
         e4:c3:c5:9e:e4:ae:10:90:a5:00:bd:91:b2:28:df:c8:05:95:
         95:fd:0a:aa:96:cc:67:af:ec:99:81:dc:70:bb:5a:c7:41:2a:
         a5:72:5a:ab:18:65:ec:11:d2:e2:38:d1:e4:39:f3:42:dc:6a:
         06:4d:02:e9:12:49:d8:46:cb:3b:6e:2f:37:37:79:9d:9d:e3:
         dc:74:27:6d:07:d7:d4:f8:74:f5:51:ab:2e:4a:81:f6:59:0d:
         ed:23:34:19:13:1f:63:9f:0f:c6:42:2e:05:7e:0b:1b:c8:43:
         f5:97:8a:d7:22:5d:23:86:9e:15:07:07:c7:8e:24:e4:3b:47:
         0d:30:fb:36:b2:62:f4:ce:22:e7:88:5f:39:bd:15:1c:21:33:
         3f:ef:7c:c9:53:b7:c3:4e:08:c2:ff:ce:b9:c6:42:5c:a6:98:
         89:eb:0d:11:6f:1c:f4:e7:80:d5:8d:a1:8b:f0:e0:8b:ad:0c:
         5e:39:a4:3f:d2:c4:6c:5f:df:31:92:5a:da:2f:77:36:36:d3:
         1a:34:19:23:80:02:41:33:40:f5:ff:fd:c7:05:2b:f2:c4:6d:
         d2:92:1b:22:02:f6:e4:25:87:43:43:20:5f:d4:5f:a6:f0:47:
         c0:e9:04:de
```

## 总结

介绍了使用 openssl 命令生成 CA 私钥和证书的方式。

## 附录

### 找不到 /etc/pki/CA/index.txt

```
Using configuration from /etc/pki/tls/openssl.cnf
/etc/pki/CA/index.txt: No such file or directory
unable to open '/etc/pki/CA/index.txt'
140406723348368:error:02001002:system library:fopen:No such file or directory:bss_file.c:402:fopen('/etc/pki/CA/index.txt','r')
140406723348368:error:20074002:BIO routines:FILE_CTRL:system lib:bss_file.c:404:
```

使用命令 `touch /etc/pki/CA/index.txt` 修复

### 找不到 /etc/pki/CA/serial

```
Using configuration from /etc/pki/tls/openssl.cnf
/etc/pki/CA/serial: No such file or directory
error while loading serial number
140554196227984:error:02001002:system library:fopen:No such file or directory:bss_file.c:402:fopen('/etc/pki/CA/serial','r')
140554196227984:error:20074002:BIO routines:FILE_CTRL:system lib:bss_file.c:404:
```

使用命令 `echo 01 > /etc/pki/CA/serial` 修复

