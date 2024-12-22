const std = @import("std");

pub const Position = struct {
    i: usize,
    j: usize,

    pub fn move(self: *const Position, direction: Direction) ?Position {
        const delta_i, const delta_j = direction.as_deltas();
        const i = pos_with_offset(self.i, delta_i) orelse return null;
        const j = pos_with_offset(self.j, delta_j) orelse return null;
        return Position{ .i = i, .j = j };
    }

    pub fn neighbors(self: *const Position) [4]?Position {
        return [4]?Position{ self.move(.up), self.move(.down), self.move(.left), self.move(.right) };
    }
};

pub const Direction = enum {
    up,
    down,
    left,
    right,

    /// Rotates clockwise.
    pub fn rotate(self: Direction) Direction {
        return switch (self) {
            .up => .right,
            .right => .down,
            .down => .left,
            .left => .up,
        };
    }

    /// Rotates without going back to up.
    pub fn rotate_no_repeat(self: Direction) ?Direction {
        return switch (self) {
            .up => .right,
            .right => .down,
            .down => .left,
            .left => null,
        };
    }

    pub fn as_deltas(self: Direction) struct { i2, i2 } {
        return switch (self) {
            .up => .{ -1, 0 },
            .down => .{ 1, 0 },
            .left => .{ 0, -1 },
            .right => .{ 0, 1 },
        };
    }
};

pub fn Grid(comptime T: type) type {
    return struct {
        const Self = @This();

        grid: std.ArrayList(T),
        cols: usize,

        pub fn new(data: []const u8, parse_fn: fn (u8) anyerror!T, allocator: std.mem.Allocator) !Self {
            var lines = std.mem.tokenizeScalar(u8, data, '\n');
            var grid = std.ArrayList(T).init(allocator);
            var cols: usize = 0;

            while (lines.next()) |line| {
                var row_cols: usize = 0;
                for (line) |c| {
                    const val: T = try parse_fn(c);
                    try grid.append(val);
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

        pub fn deinit(self: Self) void {
            self.grid.deinit();
        }

        /// Returns null if out of bounds.
        pub fn get(self: *const Self, pos: Position) ?T {
            const idx = self.get_idx(pos) orelse return null;
            return self.grid.items[idx];
        }

        pub fn get_idx(self: *const Self, pos: Position) ?usize {
            if (pos.j >= self.cols) {
                return null;
            }
            const idx = pos.i * self.cols + pos.j;
            if (idx >= self.grid.items.len) {
                return null;
            }

            return idx;
        }

        pub fn get_allocator(self: *const Self) std.mem.Allocator {
            return self.grid.allocator;
        }

        pub fn get_mut(self: *Self, pos: Position) ?*T {
            const idx = self.get_idx(pos) orelse return null;
            return &self.grid.items[idx];
        }

        pub inline fn items(self: *const Self) []const T {
            return self.grid.items;
        }

        pub fn neighbors(self: *const Self, pos: Position) NeighborIter {
            return NeighborIter{ .gridy = self, .pos = pos, .curr_dir = .up };
        }

        pub inline fn get_lines(self: *const Self) usize {
            return self.items().len / self.cols;
        }

        pub const NeighborIter = struct {
            gridy: *const Self,
            pos: Position,
            curr_dir: ?Direction,

            pub fn next(self: *NeighborIter) ?struct {
                pos: Position,
                item: T,
            } {
                var dir = self.curr_dir orelse return null;

                while (true) {
                    // check if the current direction has anything in it
                    const maybe_neighbor = self.pos.move(dir);

                    if (maybe_neighbor) |neighbor| {
                        // we have a neighbor position, now we need to check if
                        // this neighbor exists within the grid
                        if (self.gridy.get(neighbor)) |val| {
                            self.curr_dir = dir.rotate_no_repeat();
                            return .{ .item = val, .pos = neighbor };
                        }
                    }

                    if (dir.rotate_no_repeat()) |next_dir| {
                        // we can now check the next direction
                        dir = next_dir;
                    } else {
                        // neighbor is null and all directions have been exhausted,
                        // set curr_dir to null to fuse the iterator
                        self.curr_dir = null;
                        return null;
                    }
                }
            }
        };

        fn get_with_offset(self: *const Self, i: usize, j: usize, delta_i: i8, delta_j: i8) ?T {
            const new_i = pos_with_offset(i, delta_i) orelse return null;
            const new_j = pos_with_offset(j, delta_j) orelse return null;
            return self.get(new_i, new_j);
        }

        pub fn formatter(self: *const Self, T_format: *const fn (T, comptime []const u8, std.fmt.FormatOptions, anytype) anyerror!void) gen_fmt(T, T_format) {
            return gen_fmt(T, T_format){ .grid = self };
        }
    };
}

fn pos_with_offset(i: usize, delta_i: i2) ?usize {
    if (delta_i == 0) {
        return i;
    } else if (delta_i < 0) {
        return std.math.sub(usize, i, @as(usize, @intCast(@as(i8, -1) * delta_i))) catch null;
    } else {
        return std.math.add(usize, i, @intCast(delta_i)) catch null;
    }
}

fn gen_fmt(comptime T: type, t_format: *const fn (T, comptime []const u8, std.fmt.FormatOptions, anytype) anyerror!void) type {
    return struct {
        grid: *const Grid(T),

        const Self = @This();

        pub fn format(self: Self, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
            const cols = self.grid.cols;
            const lines = self.grid.get_lines();

            const lines_width = std.math.log10_int(lines) + 1;
            var lines_options = options;
            lines_options.width = lines_width;

            try writer.writeAll("grid:\n");

            try std.fmt.formatIntValue(' ', "c", lines_options, writer);
            try writer.writeAll(" \x1b[1;95m");
            for (0..cols) |col| {
                try std.fmt.formatIntValue(col, "d", options, writer);
            }
            try writer.writeAll("\x1b[0m\n");

            for (0..lines) |i| {
                try writer.writeAll("\x1b[1;95m");
                try std.fmt.formatIntValue(i, "d", lines_options, writer);
                try writer.writeAll(" \x1b[0m");
                for (0..cols) |j| {
                    const item = self.grid.get(Position{ .i = i, .j = j }).?;
                    try t_format(item, fmt, options, writer);
                }
                try writer.writeAll("\n");
            }
        }
    };
}
