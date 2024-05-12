# 0516-WebGPU-标准化坐标

## 环境

- Time 2024-05-11
- Zig 0.12.0-dev.3180+83e578a18
- mach 26b2351d4b04122d51c140b2d35325c02ccb0a5a

## 前言

### 说明

参考资料：

1. <https://github.com/hexops/mach/tree/main/src/core/examples>
2. <https://eliemichel.github.io/LearnWebGPU/index.html>

### 目标

WebGPU 中的坐标范围为：x 和 y 都在 [-1.0, 1.0]，z 在 [0.0, 1.0]。

## shader.wgsl

上一节的坐标有些超过了 1，使用着色器的平移可以解决。

```wgsl
struct VertexInput {
    @location(0) position: vec2f,
    @location(1) color: vec3f,
};

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) color: vec3f,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    // 屏幕比率矫正，缩放
    let ratio = 800.0 / 600.0;
    // 平移
    let offset = vec2f(-0.6875, -0.463);
    let y = (in.position.y + offset.y) * ratio;
    out.position = vec4f(in.position.x + offset.x, y, 0.0, 1.0);
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    return vec4f(in.color, 1.0);
}
```

## main.zig

无变化

## 效果

![标准化坐标][1]

## 总结

介绍了 WebGPU 中的标准化坐标，解决了坐标超过 1 的问题。

[1]: images/webgpu13.png

## 附录
