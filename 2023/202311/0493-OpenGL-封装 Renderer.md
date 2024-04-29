# 0493-OpenGL-封装 Renderer

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

封装精灵的渲染逻辑。

## sprite.zig

```zig
const std = @import("std");
const engine = @import("engine.zig");
const zlm = @import("zlm");
const gl = @import("gl");

const Texture = engine.Texture;
const Shader = engine.Shader;

pub const Renderer = struct {
    shader: Shader,
    vao: c_uint = 0,

    pub fn draw(self: Renderer, texture: Texture) void {
        self.shader.use();

        const model = zlm.Mat4.identity;
        self.shader.setUniformMatrix4fv("model", &model.fields[0][0]);

        gl.ActiveTexture(gl.TEXTURE0);
        texture.bind();

        gl.BindVertexArray(self.vao);
        gl.DrawArrays(gl.TRIANGLES, 0, 6);
    }

    pub fn initRenderData(self: *Renderer) void {
        const vertices = [_]f32{
            0.0, 1.0, 0.0, 1.0, //
            1.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 0.0,

            0.0, 1.0, 0.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
            1.0, 0.0, 1.0, 0.0,
        };

        var vbos: [1]c_uint = undefined;
        gl.GenBuffers(vbos.len, &vbos);
        gl.BindBuffer(gl.ARRAY_BUFFER, vbos[0]);
        const size = @sizeOf(@TypeOf(vertices));
        gl.BufferData(gl.ARRAY_BUFFER, size, &vertices, gl.STATIC_DRAW);

        gl.GenVertexArrays(1, (&self.vao)[0..1]);
        gl.BindVertexArray(self.vao);
        gl.EnableVertexAttribArray(0);
        gl.VertexAttribPointer(0, 4, gl.FLOAT, gl.FALSE, 4 * @sizeOf(f32), 0);

        gl.BindBuffer(gl.ARRAY_BUFFER, 0);
        gl.BindVertexArray(0);
    }
};
```

## vertex.glsl

```zig
#version 330 core

layout(location = 0) in vec4 vertex; // 顶点位置
uniform mat4 model;
uniform mat4 projection;
out vec2 uv;


void main()
{
    gl_Position = projection * model * vec4(vertex.xy, 0.0, 1.0);
    uv = vertex.zw;
}
```

## fragment.glsl

```zig
#version 330 core
in vec2 uv;
out vec4 color;
uniform sampler2D image;

void main()
{
    color = texture(image, uv);
}
```

## main.zig

```zig
const std = @import("std");
const glfw = @import("mach-glfw");
const gl = @import("gl");
const zstbi = @import("zstbi");
const zlm = @import("zlm");
const engine = @import("engine.zig");
const resource = @import("resource.zig");
const sprite = @import("sprite.zig");

fn logGlfwError(code: glfw.ErrorCode, description: [:0]const u8) void {
    std.log.err("{}: {s}\n", .{ code, description });
}

fn glfwPanic() noreturn {
    @panic(glfw.getErrorString() orelse "unknown error");
}

var glProcs: gl.ProcTable = undefined;
const vertices = [_]f32{
    0.0, 1.0, 0.0, 1.0, //
    1.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 0.0,

    0.0, 1.0, 0.0, 1.0,
    1.0, 1.0, 1.0, 1.0,
    1.0, 0.0, 1.0, 0.0,
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const window = initWindow();
    defer deinit(window);

    zstbi.init(gpa.allocator());
    defer zstbi.deinit();

    glfw.makeContextCurrent(window);
    defer glfw.makeContextCurrent(null);
    glfw.swapInterval(1);

    if (!glProcs.init(glfw.getProcAddress)) glfwPanic();

    gl.makeProcTableCurrent(&glProcs);
    defer gl.makeProcTableCurrent(null);
    gl.Enable(gl.BLEND);

    resource.init(gpa.allocator());
    defer resource.deinit();

    const vs: [:0]const u8 = @embedFile("vertex.glsl");
    const fs: [:0]const u8 = @embedFile("fragment.glsl");
    const shader = try resource.loadShader("shader", vs, fs);

    var renderer = sprite.Renderer{ .shader = shader };
    renderer.initRenderData();

    const face = "awesomeface.png";
    const texture = try resource.loadTexture(face, "assets/" ++ face, true);

    shader.use();
    const projection = zlm.Mat4.createOrthogonal(0, 1, 1, 0, -1, 1);
    shader.setUniformMatrix4fv("projection", &projection.fields[0][0]);
    shader.setUniform1i("image", 0);

    while (!window.shouldClose()) {
        glfw.pollEvents();
        gl.ClearColor(0, 0, 0, 0);
        gl.Clear(gl.COLOR_BUFFER_BIT);

        renderer.draw(texture);

        window.swapBuffers();
    }
}

fn initWindow() glfw.Window {
    glfw.setErrorCallback(logGlfwError);

    if (!glfw.init(.{})) glfwPanic();

    return glfw.Window.create(640, 480, "学习 OpenGL", null, null, .{
        .context_version_major = gl.info.version_major,
        .context_version_minor = gl.info.version_minor,
        .opengl_profile = .opengl_core_profile,
    }) orelse glfwPanic();
}

fn deinit(window: glfw.Window) void {
    window.destroy();
    glfw.terminate();
}
```

## 效果

![封装精灵渲染][1]

## 总结

封装了精灵的渲染，将主函数的顶点的逻辑移动到了 sprite.zig 文件中。

[1]: images/opengl19.png

## 附录
