# 0547-WebGPU-解决纹理颠倒

## 环境

- Time 2024-05-21
- Zig 0.12.0-dev.3180+83e578a18
- mach 26b2351d4b04122d51c140b2d35325c02ccb0a5a

## 前言

### 说明

参考资料：

1. <https://github.com/hexops/mach/tree/main/src/core/examples>
2. <https://webgpufundamentals.org/>

### 目标

上一节，显示的 F 是倒着的，需要解决这个问题，有三种解决的方式。

## 第一种

在顶点着色器中翻转纹理坐标。

```wgsl
struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) texcoord: vec2f,
};

@vertex
fn vs_main(@builtin(vertex_index) index : u32) -> VertexOutput {

    let pos = array(
          // 1st triangle
          vec2f( 0.0,  0.0),  // center
          vec2f( 1.0,  0.0),  // right, center
          vec2f( 0.0,  1.0),  // center, top

          // 2st triangle
          vec2f( 0.0,  1.0),  // center, top
          vec2f( 1.0,  0.0),  // right, center
          vec2f( 1.0,  1.0),  // right, top
        );

    var out: VertexOutput;
    let xy = pos[index];
    out.position = vec4f(xy, 0.0, 1.0);
    out.texcoord = vec2f(xy.x, 1.0 - xy.y);
    return out;
}

@group(0) @binding(0) var ourSampler: sampler;
@group(0) @binding(1) var ourTexture: texture_2d<f32>;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    return textureSample(ourTexture, ourSampler, in.texcoord);
}
```

## 第二种

在片段着色器中翻转纹理坐标。

```wgsl
struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) texcoord: vec2f,
};

@vertex
fn vs_main(@builtin(vertex_index) index : u32) -> VertexOutput {

    let pos = array(
          // 1st triangle
          vec2f( 0.0,  0.0),  // center
          vec2f( 1.0,  0.0),  // right, center
          vec2f( 0.0,  1.0),  // center, top

          // 2st triangle
          vec2f( 0.0,  1.0),  // center, top
          vec2f( 1.0,  0.0),  // right, center
          vec2f( 1.0,  1.0),  // right, top
        );

    var out: VertexOutput;
    let xy = pos[index];
    out.position = vec4f(xy, 0.0, 1.0);
    out.texcoord = xy;
    return out;
}

@group(0) @binding(0) var ourSampler: sampler;
@group(0) @binding(1) var ourTexture: texture_2d<f32>;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    let texcoord = vec2f(in.texcoord.x, 1.0 - in.texcoord.y);
    return textureSample(ourTexture, ourSampler, texcoord);
}
```

## 第三种

翻转 render.zig 文件中的源数据。

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

    const sampler = device.createSampler(&.{});

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

## 效果

![翻转纹理][1]

## 总结

解决纹理显示是倒着的问题。

[1]: images/webgpu43.png

## 附录
