const std = @import("std");

pub const Delta = struct {
    i: isize,
    j: isize,

    pub fn norm(self: *const Delta) usize {
        return @intCast(@abs(self.i) + @abs(self.j));
    }
};

pub const Position = struct {
    i: usize,
    j: usize,

    pub fn x(self: *const Position) usize {
        return self.j;
    }

    pub fn y(self: *const Position) usize {
        return self.i;
    }

    pub fn move(self: *const Position, dir: Direction) ?Position {
        const delta_i, const delta_j = dir.as_deltas();
        const i = pos_with_offset(self.i, delta_i) orelse return null;
        const j = pos_with_offset(self.j, delta_j) orelse return null;
        return Position{ .i = i, .j = j };
    }

    pub fn neighbors(self: *const Position) [4]?Position {
        return [4]?Position{ self.move(.up), self.move(.down), self.move(.left), self.move(.right) };
    }

    /// Computes the direction such that `self.move(direction)` returns `other`.
    pub fn direction(self: *const Position, other: *const Position) Direction {
        if (self.i == other.i) {
            if (self.j > other.j) {
                return .left;
            } else {
                return .right;
            }
        } else if (self.j == other.j) {
            if (self.i > other.i) {
                return .up;
            } else {
                return .down;
            }
        } else {
            unreachable;
        }
    }

    pub fn eq(self: *const Position, other: *const Position) bool {
        return (self.i == other.i) and (self.j == other.j);
    }

    pub fn dist(self: *const Position, other: *const Position) usize {
        var distance: usize = 0;

        if (self.i > other.i) {
            distance += self.i - other.i;
        } else {
            distance += other.i - self.i;
        }

        if (self.j > other.j) {
            distance += self.j - other.j;
        } else {
            distance += other.j - self.j;
        }

        return distance;
    }

    pub fn sub(self: *const Position, other: *const Position) Delta {
        const i = @as(isize, @intCast(self.i)) - @as(isize, @intCast(other.i));
        const j = @as(isize, @intCast(self.j)) - @as(isize, @intCast(other.j));
        return .{ .i = i, .j = j };
    }

    pub fn format(self: *const Position, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        var first = self.i;
        var second = self.j;
        var first_name: u8 = 'i';
        var second_name: u8 = 'j';
        if (std.mem.eql(u8, fmt, "xy")) {
            first = self.j;
            first_name = 'x';
            second = self.i;
            second_name = 'y';
        }

        try std.fmt.format(writer, "Pos{{{c}:", .{first_name});
        try std.fmt.formatIntValue(first, "d", options, writer);
        try std.fmt.format(writer, ", {c}:", .{second_name});
        try std.fmt.formatIntValue(second, "d", options, writer);
        try writer.writeAll("}");
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

    pub fn opposite(self: Direction) Direction {
        const out: Direction = switch (self) {
            .up => .down,
            .left => .right,
            .down => .up,
            .right => .left,
        };

        return out;
    }

    pub fn count_rotations(self: Direction, other: Direction) usize {
        if (self == other) {
            return 0;
        } else if (self.opposite() == other) {
            return 2;
        } else {
            return 1;
        }
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

        pub fn repeat(value: T, lines: usize, cols: usize, allocator: std.mem.Allocator) !Self {
            var grid = try std.ArrayList(T).initCapacity(allocator, lines * cols);
            grid.appendNTimesAssumeCapacity(value, lines * cols);

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

        pub const NeighborEntry = struct { pos: Position, item: T };

        pub const NeighborIter = struct {
            gridy: *const Self,
            pos: Position,
            curr_dir: ?Direction,

            pub fn next(self: *NeighborIter) ?NeighborEntry {
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

        pub fn get_with_offset(self: *const Self, pos: Position, diff: struct { di: isize, dj: isize }) ?NeighborEntry {
            const i = pos_with_offset(pos.i, diff.di) orelse return null;
            const j = pos_with_offset(pos.j, diff.dj) orelse return null;
            const new_pos = Position{ .i = i, .j = j };
            if (self.get(new_pos)) |item| {
                return .{ .pos = new_pos, .item = item };
            }
            return null;
        }

        pub fn formatter(self: *const Self, T_format: *const fn (T, comptime []const u8, std.fmt.FormatOptions, anytype) anyerror!void) gen_fmt(T, T_format) {
            return gen_fmt(T, T_format){ .grid = self };
        }

        pub fn iterator(self: *const Self) Iter {
            return Iter{ .grid = self };
        }

        pub const Iter = struct {
            idx: ?usize = 0,
            grid: *const Self,

            pub fn next(self: *Self.Iter) ?struct { position: Position, value: *const T } {
                if (self.idx) |index| {
                    const value = &self.grid.grid.items[index];
                    const pos = Position{ .i = index / self.grid.cols, .j = index % self.grid.cols };
                    if (index + 1 < self.grid.grid.items.len) {
                        self.idx = index + 1;
                    } else {
                        self.idx = null;
                    }
                    return .{ .position = pos, .value = value };
                }

                return null;
            }
        };

        pub fn iterator_mut(self: *Self) IterMut {
            return IterMut{ .grid = self };
        }

        pub const IterMut = struct {
            idx: ?usize = 0,
            grid: *const Self,

            pub fn next(self: *Self.PosIter) ?struct { position: Position, value: *T } {
                if (self.idx) |index| {
                    const value = &self.grid.grid.items[index];
                    const pos = self.grid.get_pos(index);
                    if (index + 1 < self.grid.grid.items.len) {
                        self.idx = index + 1;
                    } else {
                        self.idx = null;
                    }
                    return .{ .position = pos, .value = value };
                }

                return null;
            }
        };
    };
}

fn pos_with_offset(i: usize, delta_i: isize) ?usize {
    if (delta_i == 0) {
        return i;
    } else if (delta_i < 0) {
        return std.math.sub(usize, i, @as(usize, @intCast(@as(isize, -1) * delta_i))) catch null;
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

            const cols_mod = std.math.pow(usize, 10, options.width orelse 1);

            try writer.writeAll("grid:\n");

            try std.fmt.formatIntValue(' ', "c", lines_options, writer);
            try writer.writeAll(" \x1b[1;95m");
            for (0..cols) |col| {
                const col_header = col % cols_mod;
                if (col_header == 0) {
                    try writer.writeAll("\x1b[4;35m");
                    try std.fmt.formatIntValue(col_header, "d", options, writer);
                    try writer.writeAll("\x1b[0m\x1b[1;95m");
                } else {
                    try std.fmt.formatIntValue(col_header, "d", options, writer);
                }
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
