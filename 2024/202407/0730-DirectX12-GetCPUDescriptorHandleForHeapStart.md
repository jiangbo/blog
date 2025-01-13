# 0730-DirectX12-GetCPUDescriptorHandleForHeapStart

## 目标

解决 GetCPUDescriptorHandleForHeapStart 调用的段错误问题。

## 环境

- Time 2025-01-13
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <https://www.3dgep.com/learning-directx-12-1/#DirectX_12_Graphics_Pipeline>
2. <https://github.com/marlersoft/zigwin32/issues/16>

## 想法

调用这个方法的时候，一直段错误，结果是微软文档还有绑定的错误。目前还没有修复，只能手动修改下。

## 问题

微软文档给的是这样：

```C++
D3D12_CPU_DESCRIPTOR_HANDLE GetCPUDescriptorHandleForHeapStart();
```

但是 Zig 调用的时候，需要修改成这样：

```zig
var handle: d12.D3D12_CPU_DESCRIPTOR_HANDLE = undefined;
self.descriptorHeap.GetCPUDescriptorHandleForHeapStart(&handle);
```

浪费大量时间，感觉还是多找找教程的原生语言跟着学好，像这种错误，自己排查不出来。

## 附录
