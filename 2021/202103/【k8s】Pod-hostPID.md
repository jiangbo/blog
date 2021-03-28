# 【k8s】Pod-hostPID

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

hostPID 可以设置容器里是否可以看到宿主机上的进程。

## 示例

### Pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spring-k8s
spec:
  containers:
    - name: spring-k8s
      image: jiangbo920827/spring-k8s:liveness
      ports:
        - containerPort: 8080
  hostPID: true
```

### 查看进程

```
[root@master ~]# kubectl exec spring-k8s -- ps
PID   USER     TIME  COMMAND
    1 root      0:00 /usr/lib/systemd/systemd --switched-root --system --deserialize 22
    2 root      0:00 [kthreadd]
    3 root      0:01 [kworker/0:0]
    4 root      0:00 [kworker/0:0H]
    5 root      0:00 [kworker/u4:0]
    6 root      0:00 [ksoftirqd/0]
    7 root      0:00 [migration/0]
    8 root      0:00 [rcu_bh]
    9 root      0:02 [rcu_sched]
   10 root      0:00 [lru-add-drain]
   11 root      0:00 [watchdog/0]
   12 root      0:00 [watchdog/1]
   13 root      0:00 [migration/1]
   14 root      0:01 [ksoftirqd/1]
   15 root      0:00 [kworker/1:0]
   16 root      0:00 [kworker/1:0H]
   19 root      0:00 [kdevtmpfs]
   20 root      0:00 [netns]
   21 root      0:00 [khungtaskd]
   22 root      0:00 [writeback]
   23 root      0:00 [kintegrityd]
   24 root      0:00 [bioset]
   25 root      0:00 [bioset]
   26 root      0:00 [bioset]
   27 root      0:00 [kblockd]
   28 root      0:00 [md]
   29 root      0:00 [edac-poller]
   30 root      0:00 [watchdogd]
   31 root      0:00 [kworker/1:1]
   36 root      0:00 [kswapd0]
   37 root      0:00 [ksmd]
   38 root      0:00 [khugepaged]
   39 root      0:00 [crypto]
   47 root      0:00 [kthrotld]
   49 root      0:00 [kmpath_rdacd]
   50 root      0:00 [kaluad]
   51 root      0:00 [kpsmoused]
   53 root      0:00 [ipv6_addrconf]
   66 root      0:00 [deferwq]
  103 root      0:00 [kauditd]
  105 root      0:00 [kworker/1:3]
  108 root      0:00 [kworker/0:2]
  283 root      0:00 [ata_sff]
  287 root      0:00 [scsi_eh_0]
  288 root      0:00 [scsi_tmf_0]
  289 root      0:00 [scsi_eh_1]
  290 root      0:00 [scsi_tmf_1]
  292 root      0:00 [kworker/u4:3]
  293 root      0:00 [scsi_eh_2]
  294 root      0:00 [scsi_tmf_2]
  296 root      0:00 [irq/18-vmwgfx]
  297 root      0:00 [ttm_swap]
  316 root      0:00 [kworker/0:1H]
  373 root      0:00 [kdmflush]
  374 root      0:00 [bioset]
  383 root      0:00 [kdmflush]
  384 root      0:00 [bioset]
  399 root      0:00 [bioset]
  400 root      0:00 [xfsalloc]
  401 root      0:00 [xfs_mru_cache]
  402 root      0:00 [xfs-buf/dm-0]
  403 root      0:00 [xfs-data/dm-0]
  404 root      0:00 [xfs-conv/dm-0]
  405 root      0:00 [xfs-cil/dm-0]
  406 root      0:00 [xfs-reclaim/dm-]
  407 root      0:00 [xfs-log/dm-0]
  408 root      0:00 [xfs-eofblocks/d]
  409 root      0:01 [xfsaild/dm-0]
  410 root      0:00 [kworker/1:1H]
  490 root      0:00 /usr/lib/systemd/systemd-journald
  512 root      0:00 /usr/sbin/lvmetad -f
  521 root      0:00 /usr/lib/systemd/systemd-udevd
  578 root      0:00 [xfs-buf/sda1]
  579 root      0:00 [xfs-data/sda1]
  580 root      0:00 [xfs-conv/sda1]
  581 root      0:00 [xfs-cil/sda1]
  582 root      0:00 [xfs-reclaim/sda]
  583 root      0:00 [xfs-log/sda1]
  584 root      0:00 [xfs-eofblocks/s]
  585 root      0:00 [xfsaild/sda1]
  604 root      0:00 /sbin/auditd
  630 999       0:00 /usr/lib/polkit-1/polkitd --no-debug
  631 root      0:00 /usr/lib/systemd/systemd-logind
  632 81        0:00 /usr/bin/dbus-daemon --system --address=systemd: --nofork --nopidfile --systemd-activation
  637 998       0:00 /usr/sbin/chronyd
  647 root      0:00 /usr/sbin/irqbalance --foreground
  648 root      0:00 /usr/sbin/NetworkManager --no-daemon
  656 root      0:00 /usr/sbin/crond -n
  663 root      0:00 /sbin/agetty --noclear tty1 linux
  702 root      0:00 /sbin/dhclient -d -q -sf /usr/libexec/nm-dhcp-helper -pf /var/run/dhclient-enp0s3.pid -lf /var/lib/NetworkManager/dhclient-34e9f531-5ab3-45bc-97fd-5f9d7595da9f-enp0s3.lease -cf /var/lib/NetworkManager/dhclient-enp0s3.conf enp0s3
  942 root      0:00 {tuned} /usr/bin/python2 -Es /usr/sbin/tuned -l -P
  943 root      0:00 /usr/sbin/sshd -D
  949 root      0:17 /usr/bin/kubelet --bootstrap-kubeconfig=/etc/kubernetes/bootstrap-kubelet.conf --kubeconfig=/etc/kubernetes/kubelet.conf --config=/var/lib/kubelet/config.yaml --network-plugin=cni --pod-infra-container-image=registry.aliyuncs.com/google_containers/pause:3.2
  957 root      0:00 /usr/sbin/rsyslogd -n
  959 root      0:01 /usr/bin/containerd
 1108 root      0:00 /usr/libexec/postfix/master -w
 1175 vpopmail  0:00 pickup -l -t unix -u
 1176 vpopmail  0:00 qmgr -l -t unix -u
 1304 root      0:08 /usr/bin/dockerd -H fd:// --containerd=/run/containerd/containerd.sock
 2551 root      0:00 /usr/bin/containerd-shim-runc-v2 -namespace moby -id 19cf4c1e076c45fc4d1e022243d8d3d793c8f122e5eacf70ad62d97f1e558798 -address /run/containerd/containerd.sock
 2572 root      0:00 /pause
 2608 root      0:00 /usr/bin/containerd-shim-runc-v2 -namespace moby -id 9bc15c853f8ecdba623529bfc1f5be4682edd52d22b43dad39ce180a28ed522b -address /run/containerd/containerd.sock
 2628 root      0:01 /usr/local/bin/kube-proxy --config=/var/lib/kube-proxy/config.conf --hostname-override=node2
 2839 root      0:02 /usr/bin/containerd-shim-runc-v2 -namespace moby -id 0b58ba52aecb7122b9f346a13902519ada84ba3eb0d6a0303953d9949d2edb5d -address /run/containerd/containerd.sock
 2858 root      0:00 /pause
 3046 root      0:00 /usr/bin/containerd-shim-runc-v2 -namespace moby -id 245c8670a789afcae28031a60eca343a7df959658364a448d23018f51bd53613 -address /run/containerd/containerd.sock
 3066 root      0:00 /opt/bin/flanneld --ip-masq --kube-subnet-mgr --iface=enp0s8
 3988 root      0:00 /usr/bin/containerd-shim-runc-v2 -namespace moby -id 27e6cb816b69a8239545c599c53a2e933d5197b61fb88f51cf5d3efa38c04288 -address /run/containerd/containerd.sock
 4010 root      0:00 /pause
 4110 root      0:00 /usr/bin/containerd-shim-runc-v2 -namespace moby -id 5ab8bf21a3c8daaafb686205bb4b6078f092259181578e30172f30537c0a4df0 -address /run/containerd/containerd.sock
 4131 root      0:12 java org.springframework.boot.loader.JarLauncher
 4233 root      0:00 sshd: root@pts/0
 4236 root      0:00 sshd: root@notty
 4238 root      0:00 /usr/libexec/openssh/sftp-server
 4239 root      0:00 -bash
 4531 root      0:00 [kworker/0:1]
 5116 root      0:00 ps
[root@master ~]#
```

## 总结

hostPID 可以设置容器里是否可以看到宿主机上的进程。

## 附录
