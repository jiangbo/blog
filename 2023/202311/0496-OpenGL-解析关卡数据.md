# 0496-OpenGL-解析关卡数据

## 环境

- Time 2024-05-01
- Zig 0.12.0-dev.3180+83e578a18
- WSL-Ubuntu 22.04.3 LTS
- OpenGL 3.3

## 前言

### 说明

参考资料：

1. <https://learnopengl-cn.github.io/06%20In%20Practice/2D-Game>
2. <https://learnopengl.com/In-Practice/2D-Game/>

### 目标

从文件中读取关卡数据，然后将其解析成关卡对象。

## GameLevel

```zig
const GameLevel = struct {
    bricks: std.ArrayList(Sprite) = undefined,
    width: f32 = 0,
    height: f32 = 0,

    fn draw(self: GameLevel, renderer: SpriteRenderer) void {
        for (self.bricks.items) |brick| {
            renderer.draw(brick);
        }
    }

    fn deinit(self: GameLevel) void {
        self.bricks.deinit();
    }
    // fn isCompleted() bool{
    //     return false;
    // };
    fn init(self: *GameLevel, allocator: std.mem.Allocator, path: []const u8) !void {
        try self.doInit(allocator, path);
    }

    fn doInit(self: *GameLevel, allocator: std.mem.Allocator, path: []const u8) !void {
        std.log.info("load level: {s}", .{path});
        const file = try std.fs.cwd().openFile(path, .{});
        defer file.close();

        const text = try file.readToEndAlloc(allocator, 1024 * 4);
        defer allocator.free(text);

        const parsed = try std.json.parseFromSlice(FileLevel, allocator, text, .{});
        defer parsed.deinit();

        try self.parse(allocator, parsed.value);
    }

    fn parse(self: *GameLevel, allocator: std.mem.Allocator, level: FileLevel) !void {
        const size = level.width * level.height;
        self.bricks = try std.ArrayList(Sprite).initCapacity(allocator, size);

        const unitWidth = self.width / @as(f32, @floatFromInt(level.width));
        const unitHeight = self.height / @as(f32, @floatFromInt(level.height));

        for (level.level, 0..) |unit, index| {
            const x: f32 = @floatFromInt(index % level.width);
            const y: f32 = @floatFromInt(index / level.width);
            if (unit == 1) {
                try self.bricks.append(Sprite{
                    .position = zlm.Vec2.new(x * unitWidth, y * unitHeight),
                    .size = zlm.Vec2.new(unitWidth, unitHeight),
                    .texture = resource.getTexture(.solid_block),
                    .solid = true,
                });
                continue;
            }

            const color = switch (unit) {
                0 => continue,
                2 => zlm.Vec3.new(0.2, 0.6, 1.0),
                3 => zlm.Vec3.new(0.0, 0.7, 0.0),
                4 => zlm.Vec3.new(0.8, 0.8, 0.4),
                5 => zlm.Vec3.new(1.0, 0.5, 0.0),
                else => zlm.Vec3.new(1.0, 1.0, 1.0),
            };

            const sprite = Sprite{
                .position = zlm.Vec2.new(x * unitWidth, y * unitHeight),
                .size = zlm.Vec2.new(unitWidth, unitHeight),
                .texture = resource.getTexture(.block),
                .color = color,
            };
            try self.bricks.append(sprite);
        }
    }
};

const FileLevel = struct {
    level: []const u8,
    width: usize,
    height: usize,
};
```

## 效果

## 总结

从文件中读取了关卡数据，并且将其解析成了 GameLevel 对象。

## 附录

### lv1.json

```json
{"width":15,"height":8,"level":[5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,
5,5,5,5,5,5,5,5,4,4,4,4,4,0,0,0,0,0,4,4,4,4,4,4,1,4,1,4,0,0,1,0,0,4,1,4,1,4,
3,3,3,3,3,0,0,0,0,0,3,3,3,3,3,3,3,1,3,3,3,3,3,3,3,3,3,1,3,3,2,2,2,2,2,2,2,2,
2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2]}
```

### lv2.json

```json
{"width":15,"height":8,"level":[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,5,5,0,5,5,
0,5,5,0,5,5,0,1,1,5,5,5,5,5,5,5,5,5,5,5,5,5,1,1,0,3,3,0,3,3,0,3,3,0,3,3,0,1,
1,3,3,3,3,3,3,3,3,3,3,3,3,3,1,1,0,2,2,0,2,2,0,2,2,0,2,2,0,1,1,2,2,2,2,2,2,2,
2,2,2,2,2,2,1,1,0,1,1,0,1,1,0,1,1,0,1,1,0,1]}
```

### lv3.json

```json
{"width":13,"height":9,"level":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,
0,2,0,0,0,0,0,2,0,0,0,0,0,2,0,0,0,0,0,0,5,5,5,5,5,5,5,0,0,0,0,0,5,5,0,5,5,5,
0,5,5,0,0,0,5,5,5,5,5,5,5,5,5,5,5,0,0,3,0,1,1,1,1,1,1,1,0,3,0,0,3,0,3,0,0,0,
0,0,3,0,3,0,0,0,0,0,4,4,0,4,4,0,0,0,0]}
```

### lv4.json

```json
{"width":13,"height":6,"level":[1,2,1,2,1,2,1,2,1,2,1,2,1,2,2,2,2,2,2,2,2,2,
2,2,2,2,2,1,3,1,4,1,5,1,4,1,3,1,2,2,3,3,4,4,5,5,5,4,4,3,3,2,2,1,3,1,4,1,5,1,
4,1,3,1,2,2,2,3,3,4,4,5,4,4,3,3,2,2]}
```
