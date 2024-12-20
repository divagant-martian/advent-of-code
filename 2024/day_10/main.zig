const std = @import("std");

const Num: type = u4;

const Position = struct {
    i: usize,
    j: usize,

    fn move(self: *const Position, direction: Direction) ?Position {
        const delta_i, const delta_j = direction.as_deltas();
        const i = pos_with_offset(self.i, delta_i) orelse return null;
        const j = pos_with_offset(self.j, delta_j) orelse return null;
        return Position{ .i = i, .j = j };
    }

    fn neighbors(self: *const Position) [4]?Position {
        return [4]?Position{ self.move(.up), self.move(.down), self.move(.left), self.move(.right) };
    }
};

const Direction = enum {
    up,
    down,
    left,
    right,

    fn as_deltas(self: Direction) struct { i2, i2 } {
        return switch (self) {
            .up => .{ -1, 0 },
            .down => .{ 1, 0 },
            .left => .{ 0, -1 },
            .right => .{ 0, 1 },
        };
    }
};

const Grid = struct {
    grid: std.ArrayList(Num),
    cols: usize,

    fn new(data: []const u8, allocator: std.mem.Allocator) !Grid {
        var lines = std.mem.tokenizeScalar(u8, data, '\n');
        var grid = std.ArrayList(Num).init(allocator);
        var cols: usize = 0;

        while (lines.next()) |line| {
            var row_cols: usize = 0;
            for (line) |c| {
                const d = try std.fmt.parseInt(Num, &[_]u8{c}, 10);
                try grid.append(d);
                row_cols += 1;
            }
            if ((cols != 0) and cols != row_cols) {
                @panic("not a rentangle!");
            }
            cols = row_cols;
        }

        return .{
            .grid = grid,
            .cols = cols,
        };
    }

    fn get(self: *const Grid, pos: Position) ?Num {
        if (pos.j >= self.cols) {
            return null;
        }
        const idx = pos.i * self.cols + pos.j;
        if (idx >= self.grid.items.len) {
            return null;
        }

        return self.grid.items[idx];
    }

    fn get_with_offset(self: *const Grid, i: usize, j: usize, delta_i: i8, delta_j: i8) ?Num {
        const new_i = pos_with_offset(i, delta_i) orelse return null;
        const new_j = pos_with_offset(j, delta_j) orelse return null;
        return self.get(new_i, new_j);
    }

    fn trails_from_position(self: *const Grid, pos: Position, start_val: Num) !usize {
        const head = self.get(pos) orelse @panic("finding trail outside grid");
        if (head != start_val) {
            return 0;
        }

        var trails = std.AutoArrayHashMap(Position, void).init(self.grid.allocator);
        var queue = std.ArrayList(Position).init(self.grid.allocator);
        try queue.append(pos);

        while (queue.popOrNull()) |current| {
            const my_val = self.get(current) orelse @panic("queued outside of grid");
            for (current.neighbors()) |maybe_neighbor| {
                if (maybe_neighbor) |neighbor| {
                    if (self.get(neighbor)) |val| {
                        if (my_val + 1 == val) {
                            if (val == 9) {
                                try trails.put(neighbor, {});
                            } else {
                                try queue.append(neighbor);
                            }
                        }
                    }
                }
            }
        }

        return trails.count();
    }

    fn all_trails_score(self: *const Grid) !usize {
        const lines = self.grid.items.len / self.cols;

        var total: usize = 0;

        for (0..lines) |i| {
            for (0..self.cols) |j| {
                total += try self.trails_from_position(Position{ .i = i, .j = j }, 0);
            }
        }

        return total;
    }
};

fn pos_with_offset(i: usize, delta_i: i8) ?usize {
    if (delta_i == 0) {
        return i;
    } else if (delta_i < 0) {
        return std.math.sub(usize, i, @as(usize, @intCast(@as(i8, -1) * delta_i))) catch null;
    } else {
        return std.math.add(usize, i, @intCast(delta_i)) catch null;
    }
}

fn direction_char(delta_i: i8, delta_j: i8) []const u8 {
    if ((delta_i > 0) and (delta_j < 0)) {
        return "⬋";
    } else if ((delta_i == 0) and (delta_j < 0)) {
        return "⬅";
    } else if ((delta_i < 0) and (delta_j < 0)) {
        return "⬉";
    } else if ((delta_i < 0) and (delta_j == 0)) {
        return "⬆";
    } else if ((delta_i < 0) and (delta_j > 0)) {
        return "⬈";
    } else if ((delta_i == 0) and (delta_j > 0)) {
        return "⮕";
    } else if ((delta_i > 0) and (delta_j > 0)) {
        return "⬊";
    } else {
        return "⬇";
    }
}

const Part = enum {
    a,
    b,

    const Err = error{ missingProgramPart, unkownPart };

    fn from_args(args: *std.process.ArgIterator) Err!Part {
        if (args.next()) |part| {
            if (std.mem.eql(u8, part, "a")) {
                return Part.a;
            } else if (std.mem.eql(u8, part, "b")) {
                return Part.b;
            } else {
                return Part.Err.unkownPart;
            }
        } else {
            return Part.Err.missingProgramPart;
        }
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var args = try std.process.argsWithAllocator(allocator);
    defer args.deinit();

    // process program name
    _ = args.next().?;

    const part = try Part.from_args(&args);

    var file_name: []const u8 = undefined;
    if (args.next()) |name| {
        file_name = name;
    } else {
        std.log.err("missing file name", .{});
        std.process.exit(1);
    }

    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    const data = try buf_reader.reader().readAllAlloc(allocator, std.math.maxInt(usize));

    const grid = try Grid.new(data, allocator);

    switch (part) {
        .a => {
            const total = try grid.all_trails_score();
            std.log.info("score {d}", .{total});
        },
        .b => {},
    }
}
