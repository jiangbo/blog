# 0773-sokol-渲染透明通道

## 目标

前一节渲染出来的图片，透明度有问题，导致精灵有黑色的边框，这一节进行解决。

## 环境

- Time 2025-02-21
- Zig 0.14.0-dev.2851+b074fb7dd

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>
2. <https://www.bilibili.com/video/BV1vM411f7nJ/>

## 想法

blend 涉及到的几个参数，不清楚是什么意思，这个是询问 AI 解决的，有空可以对这个了解一下。

## graphics.zig

只修改了创建渲染管线的地方。

```zig
...
        pipeline = pipeline orelse RenderPipeline{ .value = sk.gfx.makePipeline(.{
            .shader = sk.gfx.makeShader(batch.batchShaderDesc(sk.gfx.queryBackend())),
            .colors = init: {
                var c: [4]sk.gfx.ColorTargetState = @splat(.{});
                c[0] = .{ .blend = .{
                    .enabled = true,
                    .src_factor_rgb = .SRC_ALPHA,
                    .dst_factor_rgb = .ONE_MINUS_SRC_ALPHA,
                } };
                break :init c;
            },
            .depth = .{
                .compare = .LESS_EQUAL,
                .write_enabled = true,
            },
            .cull_mode = .BACK,
        }) };
...
        pipeline = pipeline orelse RenderPipeline{
            .value = sk.gfx.makePipeline(.{
                .shader = sk.gfx.makeShader(single.singleShaderDesc(sk.gfx.queryBackend())),
                .layout = init: {
                    var l = sk.gfx.VertexLayoutState{};
                    l.attrs[single.ATTR_single_position].format = .FLOAT3;
                    l.attrs[single.ATTR_single_color0].format = .FLOAT4;
                    l.attrs[single.ATTR_single_texcoord0].format = .FLOAT2;
                    break :init l;
                },
                .colors = init: {
                    var c: [4]sk.gfx.ColorTargetState = @splat(.{});
                    c[0] = .{
                        .blend = .{
                            .enabled = true,
                            .src_factor_rgb = .SRC_ALPHA,
                            .dst_factor_rgb = .ONE_MINUS_SRC_ALPHA,
                        },
                    };
                    break :init c;
                },
                .index_type = .UINT16,
                .depth = .{
                    .compare = .LESS_EQUAL,
                    .write_enabled = true,
                },
            }),
        };
...
```

## 效果

![透明通道][1]

[1]: images/sokol036.webp

## 附录
