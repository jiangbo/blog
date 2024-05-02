# 0501-OpenGL-处理碰撞

## 环境

- Time 2024-05-02
- Zig 0.12.0-dev.3180+83e578a18
- WSL-Ubuntu 22.04.3 LTS
- OpenGL 3.3

## 前言

### 说明

参考资料：

1. <https://learnopengl-cn.github.io/06%20In%20Practice/2D-Game>
2. <https://learnopengl.com/In-Practice/2D-Game/>

### 目标

实现小球的粒子效果。

## sprite.zig

```zig
var rand: ?std.rand.DefaultPrng = null;

fn randomNumber() f32 {
    if (rand == null) {
        const seed = @as(u64, @bitCast(std.time.milliTimestamp()));
        rand = std.rand.DefaultPrng.init(seed);
    }

    return @floatFromInt(rand.?.random().int(u16) % 100);
}

pub const Particle = struct {
    position: zlm.Vec2 = zlm.Vec2.zero,
    velocity: zlm.Vec2 = zlm.Vec2.zero,
    color: zlm.Vec4 = zlm.Vec4.one,
    life: f32 = 0,
    texture: Texture2D,

    pub fn respawn(self: *Particle, object: Sprite, offset: zlm.Vec2) void {
        const random = (randomNumber() - 50) / 10.0;
        const rColor = 0.5 + (randomNumber() / 100.0);
        const translate = zlm.Vec2.all(random).add(offset);
        self.position = object.position.add(translate);
        self.color = zlm.Vec4.new(rColor, rColor, rColor, 1.0);
        self.life = 1.0;
        self.velocity = object.velocity.scale(0.1);
    }
};
```

## resource.zig

```zig
const std = @import("std");
const zstbi = @import("zstbi");
const Texture2D = @import("texture.zig").Texture2D;
const Shader = @import("shader.zig").Shader;

pub const Texture2DEnum = enum { face, block, solid_block, background, paddle };
pub const ShaderEnum = enum { shader };

var textures: std.EnumMap(Texture2DEnum, Texture2D) = undefined;
var shaders: std.EnumMap(ShaderEnum, Shader) = undefined;

pub fn init(allocator: std.mem.Allocator) void {
    zstbi.init(allocator);

    textures = std.EnumMap(Texture2DEnum, Texture2D){};
    shaders = std.EnumMap(ShaderEnum, Shader){};
}

const cstr = [:0]const u8;
pub fn loadShader(name: ShaderEnum, vs: cstr, fs: cstr) Shader {
    const shader = Shader.init(vs, fs);
    shaders.put(name, shader);
    return shader;
}

pub fn getShader(name: ShaderEnum) Shader {
    return shaders.get(name).?;
}

fn loadTexture(name: Texture2DEnum, file: cstr) Texture2D {
    var image = zstbi.Image.loadFromFile(file, 4) catch unreachable;
    defer image.deinit();

    var texture = Texture2D{};
    texture.generate(image.width, image.height, image.data);

    textures.put(name, texture);
    return texture;
}

pub fn getTexture(name: Texture2DEnum) Texture2D {
    return textures.get(name) orelse loadTexture(name, switch (name) {
        .face => "assets/awesomeface.png",
        .block => "assets/block.png",
        .solid_block => "assets/block_solid.png",
        .background => "assets/background.jpg",
        .paddle => "assets/paddle.png",
    });
}

pub fn deinit() void {
    var textureIterator = textures.iterator();
    while (textureIterator.next()) |texture| texture.value.deinit();
    var shaderIterator = shaders.iterator();
    while (shaderIterator.next()) |shader| shader.value.deinit();
    zstbi.deinit();
}
```

## ParticleRenderer

```zig
pub const ParticleRenderer = struct {
    shader: Shader,
    vao: c_uint = 0,

    pub fn initRenderData(self: *ParticleRenderer) void {
        // Set up mesh and attribute properties
        var vbos: [1]c_uint = undefined;
        const vertices = [_]f32{
            0.0, 1.0, 0.0, 1.0, //
            1.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 0.0,

            0.0, 1.0, 0.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
            1.0, 0.0, 1.0, 0.0,
        };

        gl.GenVertexArrays(1, (&self.vao)[0..1]);
        gl.GenBuffers(vbos.len, &vbos);
        gl.BindVertexArray(self.vao);
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

    pub fn draw(self: ParticleRenderer, particles: []const Particle) void {

        // Use additive blending to give it a 'glow' effect
        gl.BlendFunc(gl.SRC_ALPHA, gl.ONE);
        self.shader.use();
        for (particles) |particle| {
            if (particle.life <= 0.0) continue;
            self.shader.setVector2f("offset", particle.position);
            self.shader.setVector4f("color", particle.color);
            particle.texture.bind();
            gl.BindVertexArray(self.vao);
            gl.DrawArrays(gl.TRIANGLES, 0, 6);
            gl.BindVertexArray(0);
        }
        // Don't forget to reset to default blending mode
        gl.BlendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);
    }
};
```

## 效果

![粒子效果][1]

## 总结

实现了小球的粒子效果。

[1]: images/opengl27.gif

## 附录
