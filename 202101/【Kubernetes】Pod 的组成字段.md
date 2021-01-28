# 【Kubernetes】Pod 的组成字段

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

查看 Pod 的组成字段，学习 pod 的 yaml 的定义方式。

## 组成字段


通过命令 `kubectl explain pod` 来查看 Pod 的组成字段。

```
[root@master pod]# kubectl explain pod
KIND:     Pod
VERSION:  v1

DESCRIPTION:
     Pod is a collection of containers that can run on a host. This resource is
     created by clients and scheduled onto hosts.

FIELDS:
   apiVersion   <string>
     APIVersion defines the versioned schema of this representation of an
     object. Servers should convert recognized schemas to the latest internal
     value, and may reject unrecognized values. More info:
     https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources

   kind <string>
     Kind is a string value representing the REST resource this object
     represents. Servers may infer this from the endpoint the client submits
     requests to. Cannot be updated. In CamelCase. More info:
     https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds

   metadata     <Object>
     Standard object's metadata. More info:
     https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata

   spec <Object>
     Specification of the desired behavior of the pod. More info:
     https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status

   status       <Object>
     Most recently observed status of the pod. This data may not be up to date.
     Populated by the system. Read-only. More info:
     https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
```

从上面可以看到 Pod 的版本是 v1，后续有五大类型的字段。

### apiVersion

编写 yaml 格式文件的第一行，可以通过 expain 命令查看资源获得。

### Kind

资源的类型，现在接触到了两种资源。一种是使用最多的 Pod，在搭建集群时，接触到了 Node，即集群节点。

### metadata

通用型的字段，基本上大家都有的。

### spec

针对每种类型的资源，都有不同的规范，这个字段的差异很大。

### status

当前资源所处的状态，一般由 Kubernetes 来更新这些字段，而不是我们，只读。

## 查看 Pod 信息

根据前面的说明，下面来看一个完整的 Pod 状态。其中的细节字段可以不用关心，现在应该能认识五个最基础的字段。
Pod 的名称，容器的名称和镜像应该不陌生，对于容器的资源限制，也是我们之前设置的。
容器的状态相关的字段，如果对 Docker 比较熟悉，应该也可以看懂。

`kubectl get -f pod.yaml -o yaml`

```yaml
apiVersion: v1
kind: Pod
metadata:
  annotations:
    kubectl.kubernetes.io/last-applied-configuration: |
      {"apiVersion":"v1","kind":"Pod","metadata":{"annotations":{},"name":"pod-demo1","namespace":"default"},"spec":{"containers":[{"image":"jiangbo920827/spring-demo:actuator","name":"pod-demo1","ports":[{"containerPort":8080}],"resources":{"limits":{"cpu":"200m","memory":"200Mi"}}}]}}
  creationTimestamp: "2021-01-28T13:29:10Z"
  name: pod-demo1
  namespace: default
  resourceVersion: "93714"
  uid: 050f84d5-2e5c-4896-bcb3-007767b82f17
spec:
  containers:
  - image: jiangbo920827/spring-demo:actuator
    imagePullPolicy: IfNotPresent
    name: pod-demo1
    ports:
    - containerPort: 8080
      protocol: TCP
    resources:
      limits:
        cpu: 200m
        memory: 200Mi
      requests:
        cpu: 200m
        memory: 200Mi
    terminationMessagePath: /dev/termination-log
    terminationMessagePolicy: File
    volumeMounts:
    - mountPath: /var/run/secrets/kubernetes.io/serviceaccount
      name: default-token-slbq5
      readOnly: true
  dnsPolicy: ClusterFirst
  enableServiceLinks: true
  nodeName: node1
  preemptionPolicy: PreemptLowerPriority
  priority: 0
  restartPolicy: Always
  schedulerName: default-scheduler
  securityContext: {}
  serviceAccount: default
  serviceAccountName: default
  terminationGracePeriodSeconds: 30
  tolerations:
  - effect: NoExecute
    key: node.kubernetes.io/not-ready
    operator: Exists
    tolerationSeconds: 300
  - effect: NoExecute
    key: node.kubernetes.io/unreachable
    operator: Exists
    tolerationSeconds: 300
  volumes:
  - name: default-token-slbq5
    secret:
      defaultMode: 420
      secretName: default-token-slbq5
status:
  conditions:
  - lastProbeTime: null
    lastTransitionTime: "2021-01-28T13:29:10Z"
    status: "True"
    type: Initialized
  - lastProbeTime: null
    lastTransitionTime: "2021-01-28T13:29:12Z"
    status: "True"
    type: Ready
  - lastProbeTime: null
    lastTransitionTime: "2021-01-28T13:29:12Z"
    status: "True"
    type: ContainersReady
  - lastProbeTime: null
    lastTransitionTime: "2021-01-28T13:29:10Z"
    status: "True"
    type: PodScheduled
  containerStatuses:
  - containerID: docker://5e264ac5bb2ed1fe8cee028d8fa05bc463b30c85ab2c7d2b53fb9aac6d7deab8
    image: jiangbo920827/spring-demo:actuator
    imageID: docker-pullable://jiangbo920827/spring-demo@sha256:fef2dd74c274e783e4cf2f270da15cadbc0766c8a0d24dad31dd0258e2eb4722
    lastState: {}
    name: pod-demo1
    ready: true
    restartCount: 0
    started: true
    state:
      running:
        startedAt: "2021-01-28T13:29:12Z"
  hostIP: 192.168.56.102
  phase: Running
  podIP: 10.244.1.64
  podIPs:
  - ip: 10.244.1.64
  qosClass: Guaranteed
  startTime: "2021-01-28T13:29:10Z"
```

## 描述 Pod 状态

除了上面查看 Pod 状态的方式外，还有一种方式，该种方式对于人们来说更可读。

`kubectl describe -f pod.yaml`

```yaml
Name:         pod-demo1
Namespace:    default
Priority:     0
Node:         node1/192.168.56.102
Start Time:   Thu, 28 Jan 2021 21:29:10 +0800
Labels:       <none>
Annotations:  <none>
Status:       Running
IP:           10.244.1.64
IPs:
  IP:  10.244.1.64
Containers:
  pod-demo1:
    Container ID:   docker://5e264ac5bb2ed1fe8cee028d8fa05bc463b30c85ab2c7d2b53fb9aac6d7deab8
    Image:          jiangbo920827/spring-demo:actuator
    Image ID:       docker-pullable://jiangbo920827/spring-demo@sha256:fef2dd74c274e783e4cf2f270da15cadbc0766c8a0d24dad31dd0258e2eb4722
    Port:           8080/TCP
    Host Port:      0/TCP
    State:          Running
      Started:      Thu, 28 Jan 2021 21:29:12 +0800
    Ready:          True
    Restart Count:  0
    Limits:
      cpu:     200m
      memory:  200Mi
    Requests:
      cpu:        200m
      memory:     200Mi
    Environment:  <none>
    Mounts:
      /var/run/secrets/kubernetes.io/serviceaccount from default-token-slbq5 (ro)
Conditions:
  Type              Status
  Initialized       True
  Ready             True
  ContainersReady   True
  PodScheduled      True
Volumes:
  default-token-slbq5:
    Type:        Secret (a volume populated by a Secret)
    SecretName:  default-token-slbq5
    Optional:    false
QoS Class:       Guaranteed
Node-Selectors:  <none>
Tolerations:     node.kubernetes.io/not-ready:NoExecute op=Exists for 300s
                 node.kubernetes.io/unreachable:NoExecute op=Exists for 300s
Events:
  Type    Reason     Age   From               Message
  ----    ------     ----  ----               -------
  Normal  Scheduled  38m   default-scheduler  Successfully assigned default/pod-demo1 to node1
  Normal  Pulled     38m   kubelet            Container image "jiangbo920827/spring-demo:actuator" already present on machine
  Normal  Created    38m   kubelet            Created container pod-demo1
  Normal  Started    38m   kubelet            Started container pod-demo1
```

## 总结

介绍了 Pod 的组成字段，以及查看 Pod 信息的两种方式。第一种更接近定义的方式，后一种可读性更好。

## 附录
