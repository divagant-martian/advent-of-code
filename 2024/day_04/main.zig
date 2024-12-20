const std = @import("std");

const Xmas = enum { x, m, a, s };

const Grid = struct {
    grid: std.ArrayList(Xmas),
    cols: usize,

    fn new(data: []const u8, allocator: std.mem.Allocator) !Grid {
        var lines = std.mem.tokenizeScalar(u8, data, '\n');
        var grid = std.ArrayList(Xmas).init(allocator);
        var cols: usize = 0;

        while (lines.next()) |line| {
            var row_cols: usize = 0;
            for (line) |c| {
                const l: Xmas = switch (c) {
                    'X' => .x,
                    'M' => .m,
                    'A' => .a,
                    'S' => .s,
                    else => @panic("must be XMAS !"),
                };
                try grid.append(l);
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

    fn get(self: *const Grid, i: usize, j: usize) ?Xmas {
        if (j >= self.cols) {
            return null;
        }
        const idx = i * self.cols + j;
        if (idx >= self.grid.items.len) {
            return null;
        }

        return self.grid.items[idx];
    }

    fn get_with_offset(self: *const Grid, i: usize, j: usize, delta_i: i8, delta_j: i8) ?Xmas {
        const new_i = pos_with_offset(i, delta_i) orelse return null;
        const new_j = pos_with_offset(j, delta_j) orelse return null;
        return self.get(new_i, new_j);
    }

    fn is_xmas_in_direction(self: *const Grid, i: usize, delta_i: i8, j: usize, delta_j: i8) bool {
        const here: Xmas = self.get(i, j) orelse return false;
        if ((here != .x) or ((delta_i == 0) and (delta_j == 0))) {
            return false;
        }
        for ([_]Xmas{ .m, .a, .s }, [_]i8{ 1, 2, 3 }) |needle, scalar| {
            if (self.get_with_offset(i, j, delta_i * scalar, delta_j * scalar)) |letter| {
                if (letter != needle) {
                    return false;
                }
            } else {
                return false;
            }
        }

        return true;
    }

    fn find_xmas(self: *const Grid, i: usize, j: usize) usize {
        var count: usize = 0;
        for ([_]i8{ -1, 0, 1 }) |delta_i| {
            for ([_]i8{ -1, 0, 1 }) |delta_j| {
                if (self.is_xmas_in_direction(i, delta_i, j, delta_j)) {
                    if (count == 0) {
                        std.debug.print("({d}, {d})", .{ i, j });
                    }
                    count += 1;

                    std.debug.print("{s}", .{direction_char(delta_i, delta_j)});
                }
            }
        }

        if (count > 0) {
            std.debug.print("\n", .{});
        }

        return count;
    }

    fn find_x_mas(self: *const Grid, i: usize, j: usize) bool {
        const here = self.get(i, j) orelse return false;
        if (here != .a) {
            return false;
        }

        const top_left = self.get_with_offset(i, j, -1, -1) orelse return false;
        const top_right = self.get_with_offset(i, j, -1, 1) orelse return false;
        const bot_left = self.get_with_offset(i, j, 1, -1) orelse return false;
        const bot_right = self.get_with_offset(i, j, 1, 1) orelse return false;

        var count: usize = 0;

        if ((top_left == .m) and (bot_right == .s)) {
            count += 1;
        } else if ((top_left == .s) and (bot_right == .m)) {
            count += 1;
        }

        if ((top_right == .m) and (bot_left == .s)) {
            count += 1;
        } else if ((top_right == .s) and (bot_left == .m)) {
            count += 1;
        }

        return (count == 2);
    }

    fn count_xmas(self: *const Grid, part: enum { part1, part2 }) usize {
        const rows = self.grid.items.len / self.cols;
        var total: usize = 0;

        for (0..rows) |i| {
            for (0..self.cols) |j| {
                _ = self.get(i, j).?;
                switch (part) {
                    .part1 => total += self.find_xmas(i, j),
                    .part2 => if (self.find_x_mas(i, j)) {
                        total += 1;
                    },
                }
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

pub fn main() !void {
    const allocator = std.heap.page_allocator;

    var args = try std.process.argsWithAllocator(allocator);
    defer args.deinit();

    // process program name
    _ = args.next().?;

    var second_part = false;
    if (args.next()) |part| {
        if (std.mem.eql(u8, part, "a")) {
            second_part = false;
        } else if (std.mem.eql(u8, part, "b")) {
            second_part = true;
        } else {
            std.log.err("parts a or b, you on drugs: {s}", .{part});
            std.process.exit(1);
        }
    } else {
        std.log.err("missing program part", .{});
        std.process.exit(1);
    }

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
    defer grid.grid.deinit();

    if (second_part) {
        const total = grid.count_xmas(.part2);
        std.log.info("{d}\n", .{total});
    } else {
        const total = grid.count_xmas(.part1);
        std.log.info("{d}\n", .{total});
    }
}
