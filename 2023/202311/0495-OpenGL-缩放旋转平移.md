# 0495-OpenGL-缩放旋转平移

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

在 2D 平面内，对精灵进行缩放，旋转和平移。

## renderer.zig

```zig
const std = @import("std");
const gl = @import("gl");
const zlm = @import("zlm");

const Texture2D = @import("texture.zig").Texture2D;
const Shader = @import("shader.zig").Shader;

pub const DrawSpriteOptions = struct {
    texture: Texture2D,
    position: zlm.Vec2 = zlm.Vec2.zero,
    size: zlm.Vec2 = zlm.Vec2.new(10, 10),
    rotate: f32 = 0,
    color: zlm.Vec3 = zlm.Vec3.one,
};

pub const SpriteRenderer = struct {
    shader: Shader,
    vao: c_uint = 0,

    pub fn draw(self: SpriteRenderer, options: DrawSpriteOptions) void {
        self.shader.use();

        var model = zlm.Mat4.createScale(options.size.x, options.size.y, 1);

        const x, const y = .{ -0.5 * options.size.x, -0.5 * options.size.y };
        model = model.mul(zlm.Mat4.createTranslationXYZ(x, y, 0));
        const angle = zlm.toRadians(options.rotate);
        model = model.mul(zlm.Mat4.createAngleAxis(zlm.Vec3.new(0, 0, 1), angle));
        x, y = .{ 0.5 * options.size.x, 0.5 * options.size.y };
        model = model.mul(zlm.Mat4.createTranslationXYZ(x, y, 0));

        x, y = .{ options.position.x, options.position.y };
        model = model.mul(zlm.Mat4.createTranslationXYZ(x, y, 0));

        self.shader.setUniformMatrix4fv("model", &model.fields[0][0]);

        self.shader.setVector3f("spriteColor", options.color);

        gl.ActiveTexture(gl.TEXTURE0);
        options.texture.bind();

        gl.BindVertexArray(self.vao);
        gl.DrawArrays(gl.TRIANGLES, 0, 6);
    }

    pub fn initRenderData(self: *SpriteRenderer) void {
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

## game.zig

```zig
const std = @import("std");
const zlm = @import("zlm");
const glfw = @import("mach-glfw");
const gl = @import("gl");
const resource = @import("resource.zig");
const renderer = @import("renderer.zig");

const GameState = enum { active, menu, win };
pub const Game = struct {
    state: GameState = .active,
    width: u32 = 0,
    height: u32 = 0,
    keys: [1024]bool = [1]bool{false} ** 1024,
    spriteRenderer: renderer.SpriteRenderer = undefined,

    pub fn init(self: *Game) !void {
        const vs: [:0]const u8 = @embedFile("shader/vertex.glsl");
        const fs: [:0]const u8 = @embedFile("shader/fragment.glsl");
        const shader = try resource.loadShader("shader", vs, fs);

        const projection = zlm.Mat4.createOrthogonal(0, 800, 600, 0, -1, 1);
        shader.setUniformMatrix4fv("projection", &projection.fields[0][0]);
        shader.setUniform1i("image", 0);

        self.spriteRenderer = renderer.SpriteRenderer{ .shader = shader };
        self.spriteRenderer.initRenderData();

        const face = "awesomeface.png";
        _ = try resource.loadTexture(face, "assets/" ++ face);
    }
    // game loop
    pub fn processInput(self: Game, deltaTime: f64) void {
        _ = deltaTime;
        _ = self;
    }
    pub fn update(self: Game, deltaTime: f64) void {
        _ = self;
        _ = deltaTime;
    }
    pub fn render(self: Game) void {
        const options = renderer.DrawSpriteOptions{
            .texture = resource.getTexture("awesomeface.png"),
            .position = zlm.Vec2.new(200, 200),
            .size = zlm.Vec2.new(300, 400),
            .rotate = 45,
            .color = zlm.Vec3.new(0, 1, 0),
        };
        self.spriteRenderer.draw(options);
    }
};
```

## main.zig

```zig
const std = @import("std");
const zlm = @import("zlm");
const glfw = @import("mach-glfw");
const gl = @import("gl");
const resource = @import("resource.zig");
const zstbi = @import("zstbi");
const Game = @import("game.zig").Game;

fn logGlfwError(code: glfw.ErrorCode, description: [:0]const u8) void {
    std.log.err("{}: {s}\n", .{ code, description });
}

fn glfwPanic() noreturn {
    @panic(glfw.getErrorString() orelse "unknown error");
}

var breakout: Game = Game{};
var glProcs: gl.ProcTable = undefined;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    glfw.setErrorCallback(logGlfwError);
    if (!glfw.init(.{})) glfwPanic();
    defer glfw.terminate();
    const window = glfw.Window.create(800, 600, "学习 OpenGL", null, null, .{
        .context_version_major = gl.info.version_major,
        .context_version_minor = gl.info.version_minor,
        .opengl_profile = .opengl_core_profile,
    }) orelse glfwPanic();
    defer window.destroy();

    glfw.makeContextCurrent(window);
    defer glfw.makeContextCurrent(null);
    glfw.swapInterval(1);
    glfw.Window.setFramebufferSizeCallback(window, windowChange);
    glfw.Window.setKeyCallback(window, keyCallback);

    if (!glProcs.init(glfw.getProcAddress)) glfwPanic();

    gl.makeProcTableCurrent(&glProcs);
    defer gl.makeProcTableCurrent(null);
    gl.Enable(gl.BLEND);
    gl.BlendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);

    zstbi.init(gpa.allocator());
    defer zstbi.deinit();
    resource.init(gpa.allocator());
    defer resource.deinit();

    try breakout.init();
    var lastFrame: f64 = 0.0;

    while (!window.shouldClose()) {
        const currentFrame = glfw.getTime();
        defer lastFrame = currentFrame;
        const deltaTime = currentFrame - lastFrame;

        glfw.pollEvents();
        breakout.processInput(deltaTime);
        breakout.update(deltaTime);

        gl.ClearColor(0, 0, 0, 0);
        gl.Clear(gl.COLOR_BUFFER_BIT);

        breakout.render();
        window.swapBuffers();
    }
}

fn windowChange(_: glfw.Window, w: u32, h: u32) void {
    gl.Viewport(0, 0, @as(c_int, @intCast(w)), @as(c_int, @intCast(h)));
}

fn keyCallback(
    window: glfw.Window,
    key: glfw.Key,
    scancode: i32,
    action: glfw.Action,
    _: glfw.Mods,
) void {
    if (key == .escape and action == .press)
        window.setShouldClose(true);
    if (scancode >= 0 and scancode < 1024) {
        const index: usize = @intCast(scancode);
        if (action == .press) breakout.keys[index] = true //
        else if (action == .release) breakout.keys[index] = false;
    }
}
```

## 效果

![封装窗口][1]

## 总结

对 2D 纹理，进行缩放，旋转，平移。

[1]: images/opengl21.png

## 附录
