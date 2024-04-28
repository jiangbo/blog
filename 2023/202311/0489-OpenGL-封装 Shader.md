# 0489-OpenGL-封装 Shader

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

将着色器的代码进行封装。

## Shader

删除了 shader.zig 文件，新增 engine.zig 文件，增加 Shader 封装。

```zig
const std = @import("std");
const gl = @import("gl");

fn errorPanic(message: ?[]const u8) noreturn {
    @panic(message orelse "unknown error");
}
const cstr = [:0]const u8;
pub const Shader = struct {
    program: c_uint,

    pub fn init(vertexSource: cstr, fragmentSource: cstr) Shader {
        return Shader{ .program = compile(vertexSource, fragmentSource) };
    }

    pub fn use(self: Shader) void {
        gl.UseProgram(self.program);
    }

    pub fn getUniformLocation(self: Shader, name: cstr) c_int {
        const location = gl.GetUniformLocation(self.program, name.ptr);
        if (location == -1) errorPanic("uniform not found");
        return location;
    }

    pub fn setUniform1i(self: Shader, name: cstr, value: c_int) void {
        gl.Uniform1i(self.getUniformLocation(name), value);
    }

    pub fn uniformMatrix4fv(location: c_int, value: [*c]const f32) void {
        gl.UniformMatrix4fv(location, 1, gl.FALSE, value);
    }

    pub fn setUniformMatrix4fv(self: Shader, name: cstr, value: [*c]const f32) void {
        gl.UniformMatrix4fv(self.getUniformLocation(name), 1, gl.FALSE, value);
    }

    pub fn deinit(self: Shader) void {
        gl.DeleteProgram(self.program);
    }

    fn compile(vertexSource: cstr, fragmentSource: cstr) c_uint {
        // 顶点着色器
        const vertexShader = gl.CreateShader(gl.VERTEX_SHADER);
        if (vertexShader == 0) errorPanic("create vertex shader failed");
        defer gl.DeleteShader(vertexShader);
        gl.ShaderSource(vertexShader, 1, (&vertexSource.ptr)[0..1], null);
        gl.CompileShader(vertexShader);
        checkCompileErrors(vertexShader, false);

        // 片段着色器
        const fragmentShader = gl.CreateShader(gl.FRAGMENT_SHADER);
        if (fragmentShader == 0) errorPanic("create fragment shader failed");
        defer gl.DeleteShader(fragmentShader);
        gl.ShaderSource(fragmentShader, 1, (&fragmentSource.ptr)[0..1], null);
        gl.CompileShader(fragmentShader);
        checkCompileErrors(fragmentShader, false);

        // 着色器程序
        const program = gl.CreateProgram();
        if (program == 0) errorPanic("create program failed");
        errdefer gl.DeleteProgram(program);

        gl.AttachShader(program, vertexShader);
        gl.AttachShader(program, fragmentShader);
        gl.LinkProgram(program);
        checkCompileErrors(program, true);
        return program;
    }

    fn checkCompileErrors(object: c_uint, isProgram: bool) void {
        var success: c_int = undefined;
        var logBuffer: [512:0]u8 = undefined;
        if (isProgram) {
            gl.GetProgramiv(object, gl.LINK_STATUS, &success);
            if (success == gl.FALSE) {
                gl.GetProgramInfoLog(object, logBuffer.len, null, &logBuffer);
                errorPanic(std.mem.sliceTo(&logBuffer, 0));
            }
            return;
        }

        gl.GetShaderiv(object, gl.COMPILE_STATUS, &success);
        if (success == gl.FALSE) {
            gl.GetShaderInfoLog(object, logBuffer.len, null, &logBuffer);
            errorPanic(std.mem.sliceTo(&logBuffer, 0));
        }
    }
};
```

## main

将显示一个旋转的立方体。

```zig
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
```

## 效果

只进行了封装，不改变之前的功能。

![旋转的立方体][1]

## 总结

对 Shader 进行了封装。

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

    gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.REPEAT);
    gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.REPEAT);
    gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
    gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);

    var textures: [2]c_uint = undefined;
    gl.GenTextures(textures.len, &textures);
    defer gl.DeleteTextures(textures.len, &textures);

    gl.ActiveTexture(gl.TEXTURE0);
    gl.BindTexture(gl.TEXTURE_2D, textures[0]);
    var image = try zstbi.Image.loadFromFile("assets/container.jpg", 0);
    var w: c_int = @intCast(image.width);
    var h: c_int = @intCast(image.height);
    gl.TexImage2D(gl.TEXTURE_2D, 0, gl.RGB, w, h, 0, gl.RGB, gl.UNSIGNED_BYTE, image.data.ptr);
    gl.GenerateMipmap(gl.TEXTURE_2D);
    image.deinit();
    shader.setUniform1i("texture1", 0);

    gl.ActiveTexture(gl.TEXTURE1);
    gl.BindTexture(gl.TEXTURE_2D, textures[1]);
    image = try zstbi.Image.loadFromFile("assets/awesomeface.png", 0);
    w, h = .{ @intCast(image.width), @intCast(image.height) };
    gl.TexImage2D(gl.TEXTURE_2D, 0, gl.RGBA, w, h, 0, gl.RGBA, gl.UNSIGNED_BYTE, image.data.ptr);
    gl.GenerateMipmap(gl.TEXTURE_2D);
    image.deinit();
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
