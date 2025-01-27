# 0745-sokol-VP 变换

## 目标

引入数学库，计算视图和投影变换。

## 环境

- Time 2025-01-27
- Zig 0.14.0-dev.2851+b074fb7dd

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>

## 想法

各种图形 API，涉及到左手，右手，Z 深度，行主矩阵，列主矩阵，有点乱，弄不明白，走一步看一步。

## build.zig.zon

加了 zmath 数学库。

```zig
.{
    .name = "demo",
    .version = "0.0.0",
    .dependencies = .{
        .sokol = .{
            .url = "git+https://github.com/floooh/sokol-zig.git#9a02a991ce8374f1664ca3fbf8483d2d7ded84d2",
            .hash = "12205fe90783eed8f7ab3423f83cd84094c53f93fa6c342021203301061d6df1f412",
        },
        .zstbi = .{
            .url = "git+https://github.com/zig-gamedev/zstbi#bcbd249f3f57fb84d6d76f1bc621c7bd3bfaa4a2",
            .hash = "12208b7d15a730294a7d8ee3a9d3ef145e109f94d0a68be7f0ee282e0630ede093d5",
        },
        .zmath = .{
            .url = "git+https://github.com/zig-gamedev/zmath#f40bb8a935b4878c707c3fccfd5c234cfcb43bf2",
            .hash = "1220c1ab7d1e49106f59c6adb4004befdb95525a72056364a663757aaa1365aae64a",
        },
    },

    .paths = .{""},
}
```

## build.zig

```zig
const std = @import("std");

pub fn build(b: *std.Build) !void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "demo",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    exe.subsystem = .Windows;
    b.installArtifact(exe);

    const sokol = b.dependency("sokol", .{
        .target = target,
        .optimize = optimize,
    });
    exe.root_module.addImport("sokol", sokol.module("sokol"));

    const zstbi = b.dependency("zstbi", .{});
    exe.root_module.addImport("stbi", zstbi.module("root"));
    exe.linkLibrary(zstbi.artifact("zstbi"));

    const zmath = b.dependency("zmath", .{
        .target = target,
        .optimize = optimize,
    });
    exe.root_module.addImport("zmath", zmath.module("root"));

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);
}
```

## main.zig

顶点坐标修改了，从 NDC 坐标系改为世界坐标了。

```zig
const std = @import("std");
const sk = @import("sokol");
const stbi = @import("stbi");
const zm = @import("zmath");

const shd = @import("shader/test.glsl.zig");

const clearColor: sk.gfx.Color = .{ .r = 1, .b = 1, .a = 1 };
var info: sk.gfx.PassAction = undefined;
var pipeline: sk.gfx.Pipeline = undefined;
var bind: sk.gfx.Bindings = undefined;

export fn init() void {
    sk.gfx.setup(.{
        .environment = sk.glue.environment(),
        .logger = .{ .func = sk.log.func },
    });
    info.colors[0] = .{ .load_action = .CLEAR, .clear_value = clearColor };

    bind.vertex_buffers[0] = sk.gfx.makeBuffer(.{
        .data = sk.gfx.asRange(&[_]f32{
            // 顶点和颜色
            -100, 100,  1.0, 1.0, 1.0, 0, 0,
            100,  100,  1.0, 1.0, 1.0, 1, 0,
            100,  -100, 1.0, 1.0, 1.0, 1, 1,
            -100, -100, 1.0, 1.0, 1.0, 0, 1,
        }),
    });

    bind.index_buffer = sk.gfx.makeBuffer(.{
        .type = .INDEXBUFFER,
        .data = sk.gfx.asRange(&[_]u16{ 0, 1, 2, 0, 2, 3 }),
    });

    var image = stbi.Image.loadFromFile("assets/player.bmp", 4) catch unreachable;
    defer image.deinit();
    bind.images[shd.IMG_tex] = sk.gfx.allocImage();
    sk.gfx.initImage(bind.images[shd.IMG_tex], .{
        .width = @intCast(image.width),
        .height = @intCast(image.height),
        .pixel_format = .RGBA8,
        .data = init: {
            var data = sk.gfx.ImageData{};
            data.subimage[0][0] = sk.gfx.asRange(image.data);
            break :init data;
        },
    });

    bind.samplers[shd.SMP_smp] = sk.gfx.makeSampler(.{});

    pipeline = sk.gfx.makePipeline(.{
        .shader = sk.gfx.makeShader(shd.testShaderDesc(sk.gfx.queryBackend())),
        .layout = init: {
            var l = sk.gfx.VertexLayoutState{};
            l.attrs[shd.ATTR_test_position].format = .FLOAT2;
            l.attrs[shd.ATTR_test_color0].format = .FLOAT3;
            l.attrs[shd.ATTR_test_texcoord0].format = .FLOAT2;
            break :init l;
        },
        .index_type = .UINT16,
    });
}

const width = 800;
const height = 600;

export fn frame() void {
    sk.gfx.beginPass(.{ .action = info, .swapchain = sk.glue.swapchain() });

    sk.gfx.applyPipeline(pipeline);
    sk.gfx.applyBindings(bind);
    const params = shd.VsParams{
        .vp = zm.orthographicLh(width, height, 0, 1),
    };

    sk.gfx.applyUniforms(shd.UB_vs_params, sk.gfx.asRange(&params));
    sk.gfx.draw(0, 6, 1);

    sk.gfx.endPass();
    sk.gfx.commit();
}

export fn cleanup() void {
    sk.gfx.shutdown();
}

pub fn main() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    stbi.init(gpa.allocator());
    defer stbi.deinit();

    sk.app.run(.{
        .width = width,
        .height = height,
        .window_title = "学习 sokol",
        .logger = .{ .func = sk.log.func },
        .win32_console_attach = true,
        .init_cb = init,
        .frame_cb = frame,
        .cleanup_cb = cleanup,
    });
}
```

## 效果

![VP 变换][1]

[1]: images/sokol011.png

## 附录
