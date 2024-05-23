# 0549-WebGPU-纹理 addressMode

## 环境

- Time 2024-05-23
- Zig 0.12.0-dev.3180+83e578a18
- mach 26b2351d4b04122d51c140b2d35325c02ccb0a5a

## 前言

### 说明

参考资料：

1. <https://github.com/hexops/mach/tree/main/src/core/examples>
2. <https://webgpufundamentals.org/>

### 目标

addressMode 和 OpenGL 的 wrap 模式类似，都是纹理超出范围时，如何处理。

## shader.wgsl

无变化。

## render.zig

创建采样器时，指定 addressMode 为 repeat。

```zig
const std = @import("std");
const mach = @import("mach");

const Color = extern struct {
    r: u8 = 0,
    g: u8 = 0,
    b: u8 = 0,
    a: u8 = 255,
};

pub const RenderContext = struct {
    bindGroup: *mach.gpu.BindGroup,
    pipeline: *mach.gpu.RenderPipeline,

    pub fn release(self: *RenderContext) void {
        self.pipeline.release();
    }
};

pub fn createRenderPipeline() RenderContext {
    const device = mach.core.device;

    // 编译 shader
    const source = @embedFile("shader.wgsl");
    const module = device.createShaderModuleWGSL("shader.wgsl", source);
    defer module.release();

    const vertex = mach.gpu.VertexState.init(.{
        .module = module,
        .entry_point = "vs_main",
    });

    // 片段着色器状态
    const fragment = mach.gpu.FragmentState.init(.{
        .module = module,
        .entry_point = "fs_main",
        .targets = &.{.{ .format = mach.core.descriptor.format }},
    });

    const r: Color = Color{ .r = 255 };
    const y: Color = Color{ .r = 255, .g = 255 };
    const b: Color = Color{ .b = 255 };
    const textureData = [7][5]Color{
        .{ r, r, r, r, r },
        .{ r, y, r, r, r },
        .{ r, y, r, r, r },
        .{ r, y, y, r, r },
        .{ r, y, r, r, r },
        .{ r, y, y, y, r },
        .{ b, r, r, r, r },
    };

    const width, const height = .{ textureData[0].len, textureData.len };
    const texture = device.createTexture(&.{
        .label = "F texture",
        .size = .{ .width = width, .height = height },
        .format = .rgba8_unorm,
        .usage = .{ .texture_binding = true, .copy_dst = true },
    });

    const layout = mach.gpu.Texture.DataLayout{
        .bytes_per_row = width * 4,
        .rows_per_image = height,
    };
    const size = mach.gpu.Extent3D{ .width = width, .height = height };
    device.getQueue().writeTexture(&.{ .texture = texture }, &layout, &size, &textureData);

    const sampler = device.createSampler(&.{
        .mag_filter = .linear,
        .address_mode_u = .repeat,
        .address_mode_v = .repeat,
    });

    // 创建渲染管线
    const descriptor = mach.gpu.RenderPipeline.Descriptor{
        .fragment = &fragment,
        .vertex = vertex,
    };

    const pipeline = device.createRenderPipeline(&descriptor);

    const view = texture.createView(&.{
        .format = .rgba8_unorm,
        .dimension = .dimension_2d,
    });
    const bindGroup = device.createBindGroup(
        &mach.gpu.BindGroup.Descriptor.init(.{
            .layout = pipeline.getBindGroupLayout(0),
            .entries = &.{
                mach.gpu.BindGroup.Entry.sampler(0, sampler),
                mach.gpu.BindGroup.Entry.textureView(1, view),
            },
        }),
    );

    return RenderContext{
        .bindGroup = bindGroup,
        .pipeline = pipeline,
    };
}
```

## main.zig

无变化。

## 效果

![包裹方式][1]

## 总结

使用 `addressMode` 可以控制纹理的 UV 坐标超出纹理范围时的行为。

[1]: images/webgpu45.png

## 附录
