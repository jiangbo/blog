# 0494-OpenGL-封装 Window

## 环境

- Time 2024-04-29
- Zig 0.12.0-dev.3180+83e578a18
- WSL-Ubuntu 22.04.3 LTS
- OpenGL 3.3

## 前言

### 说明

参考资料：

1. <https://learnopengl-cn.github.io/01%20Getting%20started/03%20Hello%20Window/>

### 目标

封装窗口的渲染逻辑，并且支持了窗口的缩放。

## window.zig

```zig
const std = @import("std");
const gl = @import("gl");
const glfw = @import("mach-glfw");
const zlm = @import("zlm");

const GameStateEnum = enum { menu, running, win };

fn logGlfwError(code: glfw.ErrorCode, description: [:0]const u8) void {
    std.log.err("{}: {s}\n", .{ code, description });
}

fn glfwPanic() noreturn {
    @panic(glfw.getErrorString() orelse "unknown error");
}

var glProcs: gl.ProcTable = undefined;

pub const GraphicWindow = struct {
    state: GameStateEnum = .running,
    keys: [1024]bool = undefined,
    width: usize,
    height: usize,
    window: glfw.Window,
    pub fn init(name: [:0]const u8, width: u32, height: u32) GraphicWindow {
        glfw.setErrorCallback(logGlfwError);

        if (!glfw.init(.{})) glfwPanic();

        const w = glfw.Window.create(width, height, name, null, null, .{
            .context_version_major = gl.info.version_major,
            .context_version_minor = gl.info.version_minor,
            .opengl_profile = .opengl_core_profile,
        }) orelse glfwPanic();

        glfw.makeContextCurrent(w);
        glfw.swapInterval(1);
        glfw.Window.setFramebufferSizeCallback(w, windowChange);

        if (!glProcs.init(glfw.getProcAddress)) glfwPanic();

        gl.makeProcTableCurrent(&glProcs);
        gl.Enable(gl.BLEND);
        gl.BlendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);

        return GraphicWindow{ .width = width, .height = height, .window = w };
    }

    pub fn shouldClose(self: GraphicWindow) bool {
        return self.window.shouldClose();
    }

    pub fn beginDraw(_: GraphicWindow) void {
        gl.ClearColor(0, 0, 0, 0);
        gl.Clear(gl.COLOR_BUFFER_BIT);
    }

    pub fn endDraw(self: GraphicWindow) void {
        self.window.swapBuffers();
        glfw.pollEvents();
    }

    fn processInput() void {}
    fn update() void {}
    fn render() void {}
    pub fn deinit(self: GraphicWindow) void {
        self.window.destroy();
    }

    fn windowChange(_: glfw.Window, w: u32, h: u32) void {
        gl.Viewport(0, 0, @as(c_int, @intCast(w)), @as(c_int, @intCast(h)));
    }
};
```

## main.zig

```zig
const std = @import("std");
const zlm = @import("zlm");
const engine = @import("engine.zig");
const resource = @import("resource.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    engine.init(gpa.allocator());
    defer engine.deinit();

    const window = engine.GraphicWindow.init("学习 OpenGL", 640, 480);
    defer window.deinit();
    resource.init(gpa.allocator());
    defer resource.deinit();

    const vs: [:0]const u8 = @embedFile("shader/vertex.glsl");
    const fs: [:0]const u8 = @embedFile("shader/fragment.glsl");
    const shader = try resource.loadShader("shader", vs, fs);

    var renderer = engine.Renderer{ .shader = shader };
    renderer.initRenderData();

    const face = "awesomeface.png";
    const texture = try resource.loadTexture(face, "assets/" ++ face, true);

    shader.use();
    const projection = zlm.Mat4.createOrthogonal(0, 1, 1, 0, -1, 1);
    shader.setUniformMatrix4fv("projection", &projection.fields[0][0]);
    shader.setUniform1i("image", 0);

    while (!window.shouldClose()) {
        window.beginDraw();
        renderer.draw(texture);
        window.endDraw();
    }
}
```

## 效果

![封装窗口][1]

## 总结

封装窗口到 window.zig 文件中，并且支持了窗口的缩放。

[1]: images/opengl20.gif

## 附录
