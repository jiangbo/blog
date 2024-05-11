# 0514-WebGPU-屏幕比例矫正

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

前面一直说画的是一个正方形，其实根据屏幕比例，应该是一个长方形。这里使用屏幕比率简单矫正，后面使用更好的方式。

## shader.wgsl

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
    // 屏幕比率矫正
    let ratio = 800.0 / 600.0;
    out.position = vec4f(in.position.x, in.position.y * ratio, 0.0, 1.0);
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    return vec4f(in.color, 1.0);
}
```

## main.zig

main.zig 无变化。

## 效果

![屏幕比例矫正][1]

## 总结

在屏幕上显示一个正方形，不能修改屏幕的长宽比例。

[1]: images/webgpu11.png

## 附录
