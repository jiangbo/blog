# 0490-OpenGL-封装 Texture

## 环境

- Time 2024-04-28
- Zig 0.12.0-dev.3180+83e578a18
- WSL-Ubuntu 22.04.3 LTS
- OpenGL 3.3

## 前言

### 说明

参考资料：

1. <https://github.com/hexops/mach-glfw-opengl-example/tree/main>
2. <https://learnopengl-cn.github.io/01%20Getting%20started/03%20Hello%20Window/>
3. <https://www.bilibili.com/video/BV1CZ421e7YB>

### 目标

将纹理的代码进行封装。

## Texture

```zig
pub const Texture = struct {
    id: c_uint = 0,
    image: zstbi.Image,
    width: c_int = 0,
    height: c_int = 0,

    pub fn init(path: [:0]const u8) !Texture {
        const image = try zstbi.Image.loadFromFile(path, 0);
        var texture = Texture{ .image = image };
        gl.GenTextures(1, (&texture.id)[0..1]);
        texture.width = @intCast(image.width);
        texture.height = @intCast(image.height);
        return texture;
    }

    pub fn generate(self: *Texture, internalformat: c_int, imageFormat: c_uint) void {
        gl.BindTexture(gl.TEXTURE_2D, self.id);
        gl.TexImage2D(gl.TEXTURE_2D, 0, internalformat, self.width, self.height, //
            0, imageFormat, gl.UNSIGNED_BYTE, self.image.data.ptr);
        self.image.deinit();

        gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.REPEAT);
        gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.REPEAT);
        gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
        gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
        // unbind texture
        gl.BindTexture(gl.TEXTURE_2D, 0);
    }

    pub fn bind(self: *Texture) void {
        gl.BindTexture(gl.TEXTURE_2D, self.id);
    }

    pub fn deinit(self: *Texture) void {
        gl.DeleteTextures(1, (&self.id)[0..1]);
    }
};
```

## 效果

只进行了封装，不改变之前的功能。

![旋转的立方体][1]

## 总结

对 Texture 进行了封装。

[1]: images/opengl13.gif

## 附录

### main.zig

```zig
const std = @import("std");
const glfw = @import("mach-glfw");
const gl = @import("gl");
// const shader = @import("shader.zig");
const zstbi = @import("zstbi");
const zlm = @import("zlm");
const engine = @import("engine.zig");

fn logGlfwError(code: glfw.ErrorCode, description: [:0]const u8) void {
    std.log.err("{}: {s}\n", .{ code, description });
}

fn glfwPanic() noreturn {
    @panic(glfw.getErrorString() orelse "unknown error");
}

var glProcs: gl.ProcTable = undefined;
const vertices = [_]f32{
    -0.5, -0.5, -0.5, 0.0, 0.0, //
    0.5,  -0.5, -0.5, 1.0, 0.0,
    0.5,  0.5,  -0.5, 1.0, 1.0,
    0.5,  0.5,  -0.5, 1.0, 1.0,
    -0.5, 0.5,  -0.5, 0.0, 1.0,
    -0.5, -0.5, -0.5, 0.0, 0.0,

    -0.5, -0.5, 0.5,  0.0, 0.0,
    0.5,  -0.5, 0.5,  1.0, 0.0,
    0.5,  0.5,  0.5,  1.0, 1.0,
    0.5,  0.5,  0.5,  1.0, 1.0,
    -0.5, 0.5,  0.5,  0.0, 1.0,
    -0.5, -0.5, 0.5,  0.0, 0.0,

    -0.5, 0.5,  0.5,  1.0, 0.0,
    -0.5, 0.5,  -0.5, 1.0, 1.0,
    -0.5, -0.5, -0.5, 0.0, 1.0,
    -0.5, -0.5, -0.5, 0.0, 1.0,
    -0.5, -0.5, 0.5,  0.0, 0.0,
    -0.5, 0.5,  0.5,  1.0, 0.0,

    0.5,  0.5,  0.5,  1.0, 0.0,
    0.5,  0.5,  -0.5, 1.0, 1.0,
    0.5,  -0.5, -0.5, 0.0, 1.0,
    0.5,  -0.5, -0.5, 0.0, 1.0,
    0.5,  -0.5, 0.5,  0.0, 0.0,
    0.5,  0.5,  0.5,  1.0, 0.0,

    -0.5, -0.5, -0.5, 0.0, 1.0,
    0.5,  -0.5, -0.5, 1.0, 1.0,
    0.5,  -0.5, 0.5,  1.0, 0.0,
    0.5,  -0.5, 0.5,  1.0, 0.0,
    -0.5, -0.5, 0.5,  0.0, 0.0,
    -0.5, -0.5, -0.5, 0.0, 1.0,

    -0.5, 0.5,  -0.5, 0.0, 1.0,
    0.5,  0.5,  -0.5, 1.0, 1.0,
    0.5,  0.5,  0.5,  1.0, 0.0,
    0.5,  0.5,  0.5,  1.0, 0.0,
    -0.5, 0.5,  0.5,  0.0, 0.0,
    -0.5, 0.5,  -0.5, 0.0, 1.0,
};
const indices = [_]u32{ 0, 1, 2, 0, 2, 3 };
const texCoords = [_]f32{ 0, 0, 1, 0, 1, 1, 0, 1 };

const vertexSource: [:0]const u8 = @embedFile("vertex.glsl");
const fragmentSource: [:0]const u8 = @embedFile("fragment.glsl");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const window = initWindow();
    defer deinit(window);

    zstbi.init(gpa.allocator());
    defer zstbi.deinit();
    zstbi.setFlipVerticallyOnLoad(true);

    glfw.makeContextCurrent(window);
    defer glfw.makeContextCurrent(null);
    glfw.swapInterval(1);

    if (!glProcs.init(glfw.getProcAddress)) glfwPanic();

    gl.makeProcTableCurrent(&glProcs);
    defer gl.makeProcTableCurrent(null);
    gl.Enable(gl.DEPTH_TEST);

    const shader = engine.Shader.init(vertexSource, fragmentSource);
    defer shader.deinit();

    // VBO 顶点缓冲对象
    var vbos: [1]c_uint = undefined;
    gl.GenBuffers(vbos.len, &vbos);
    defer gl.DeleteBuffers(vbos.len, &vbos);
    gl.BindBuffer(gl.ARRAY_BUFFER, vbos[0]);
    gl.BufferData(gl.ARRAY_BUFFER, @sizeOf(@TypeOf(vertices)), &vertices, gl.STATIC_DRAW);

    // VAO 顶点数组对象
    var vao: c_uint = undefined;
    gl.GenVertexArrays(1, (&vao)[0..1]);
    gl.BindVertexArray(vao);
    gl.BindBuffer(gl.ARRAY_BUFFER, vbos[0]);
    gl.EnableVertexAttribArray(0);
    gl.VertexAttribPointer(0, 3, gl.FLOAT, gl.FALSE, 5 * @sizeOf(f32), 0);

    gl.EnableVertexAttribArray(1);
    gl.VertexAttribPointer(1, 2, gl.FLOAT, gl.FALSE, 5 * @sizeOf(f32), 3 * @sizeOf(f32));

    shader.use();

    gl.ActiveTexture(gl.TEXTURE0);
    var texture1 = try engine.Texture.init("assets/container.jpg");
    defer texture1.deinit();
    texture1.generate(gl.RGB, gl.RGB);
    texture1.bind();
    shader.setUniform1i("texture1", 0);

    gl.ActiveTexture(gl.TEXTURE1);
    var texture2 = try engine.Texture.init("assets/awesomeface.png");
    defer texture2.deinit();
    texture2.generate(gl.RGBA, gl.RGBA);
    texture2.bind();
    shader.setUniform1i("texture2", 1);

    const view = zlm.Mat4.createTranslationXYZ(0, 0, -3);
    shader.setUniformMatrix4fv("view", &view.fields[0][0]);

    const projection = zlm.Mat4.createPerspective(zlm.toRadians(45.0), 640 / 480, 0.1, 100);
    shader.setUniformMatrix4fv("projection", &projection.fields[0][0]);

    while (!window.shouldClose()) {
        glfw.pollEvents();
        gl.ClearColor(0.2, 0.3, 0.3, 1.0);
        gl.Clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

        const angle: f32 = @floatCast(glfw.getTime() * zlm.toRadians(50.0));
        const model = zlm.Mat4.createAngleAxis(zlm.Vec3.new(0.5, 1.0, 0.0), angle);
        shader.setUniformMatrix4fv("model", &model.fields[0][0]);
        gl.DrawArrays(gl.TRIANGLES, 0, 36);

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
