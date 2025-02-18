# 0759-sokol-矩阵转置问题

## 目标

zmath 是行向量，所以传递给 HLSL 着色器的时候，按道理应该转置，在 Sokol 中情况不一样。

## 环境

- Time 2025-02-18
- Zig 0.14.0-dev.2851+b074fb7dd

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>
<https://learn.microsoft.com/zh-cn/windows/win32/direct3dhlsl/dx-graphics-hlsl-mul>

## 想法

sokol 将矩阵放到了乘法的右边去了。

## hlsl

从下面可以看到，sokol 把 position 顶点向量放到第一个位置了。
根据微软的文档，如果 x 是向量，则视为行向量，导致后面的矩阵也是行主矩阵，所以在代码中进行转置一直不对。

```zig
//    void vert_main()
//    {
//        gl_Position = mul(position, _19_view);
//        color = color0;
//        uv = texcoord0;
//    }
```

## 附录
