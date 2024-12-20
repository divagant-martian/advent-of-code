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

    fn rotate_no_repeat(self: Direction) ?Direction {
        return switch (self) {
            .up => .right,
            .right => .down,
            .down => .left,
            .left => null,
        };
    }

    fn as_deltas(self: Direction) struct { i2, i2 } {
        return switch (self) {
            .up => .{ -1, 0 },
            .down => .{ 1, 0 },
            .left => .{ 0, -1 },
            .right => .{ 0, 1 },
        };
    }
};

fn Gridy(comptime T: type) type {
    return struct {
        const Self = @This();

        grid: std.ArrayList(T),
        cols: usize,

        fn new(data: []const u8, parse_fn: fn (u8) anyerror!T, allocator: std.mem.Allocator) !Self {
            var lines = std.mem.tokenizeScalar(u8, data, '\n');
            var grid = std.ArrayList(Num).init(allocator);
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

        fn get(self: *const Self, pos: Position) ?T {
            const idx = self.get_idx(pos) orelse return null;
            return self.grid.items[idx];
        }

        fn get_idx(self: *const Self, pos: Position) ?usize {
            if (pos.j >= self.cols) {
                return null;
            }
            const idx = pos.i * self.cols + pos.j;
            if (idx >= self.grid.items.len) {
                return null;
            }

            return idx;
        }

        fn get_mut(self: *Self, pos: Position) ?*T {
            const idx = self.get_idx(pos) orelse return null;
            return &self.grid.items[idx];
        }

        inline fn items(self: *const Self) []const T {
            return self.grid.items;
        }

        fn neighbors(self: *const Self, pos: Position) NeighborIter {
            return NeighborIter{ .gridy = self, .pos = pos, .curr_dir = .up };
        }

        fn get_with_offset(self: *const Self, i: usize, j: usize, delta_i: i8, delta_j: i8) ?T {
            const new_i = pos_with_offset(i, delta_i) orelse return null;
            const new_j = pos_with_offset(j, delta_j) orelse return null;
            return self.get(new_i, new_j);
        }

        const NeighborIter = struct {
            gridy: *const Self,
            pos: Position,
            curr_dir: ?Direction,

            fn next(self: *NeighborIter) ?struct {
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
    };
}

const Grid = struct {
    grid: Gridy(Num),

    const Self = @This();

    fn new(data: []const u8, allocator: std.mem.Allocator) !Grid {
        const parse_num = struct {
            pub fn call(c: u8) !Num {
                return std.fmt.parseInt(Num, &[_]u8{c}, 10);
            }
        }.call;
        const grid = try Gridy(Num).new(data, parse_num, allocator);
        return Self{ .grid = grid };
    }

    fn summits_from_position(self: *const Self, pos: Position, start_val: Num) !usize {
        const head = self.grid.get(pos) orelse @panic("finding trail outside grid");
        if (head != start_val) {
            return 0;
        }

        var trails = std.AutoArrayHashMap(Position, void).init(self.grid.grid.allocator);
        var queue = std.ArrayList(Position).init(self.grid.grid.allocator);
        try queue.append(pos);

        while (queue.popOrNull()) |current| {
            const my_val = self.grid.get(current) orelse @panic("queued outside of grid");
            for (current.neighbors()) |maybe_neighbor| {
                if (maybe_neighbor) |neighbor| {
                    if (self.grid.get(neighbor)) |val| {
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
        const lines = self.grid.items().len / self.grid.cols;

        var total: usize = 0;

        for (0..lines) |i| {
            for (0..self.grid.cols) |j| {
                total += try self.summits_from_position(Position{ .i = i, .j = j }, 0);
            }
        }

        return total;
    }

    fn all_trails_rating(self: *const Self) !usize {
        const grid = std.ArrayList(?usize).init(self.grid.grid.allocator);
        var trails = Gridy(?usize){ .grid = grid, .cols = self.grid.cols };
        try trails.grid.appendNTimes(null, self.grid.items().len);

        const digits = [_]Num{ 9, 8, 7, 6, 5, 4, 3, 2, 1, 0 };

        const lines = self.grid.items().len / self.grid.cols;

        var total: usize = 0;

        for (digits) |d| {
            for (0..lines) |i| {
                for (0..self.grid.cols) |j| {
                    const pos = Position{ .i = i, .j = j };

                    const val = self.grid.get(pos).?;

                    // only deal with the digit we are searching for
                    if (val != d) {
                        continue;
                    }

                    // fill the base case: there is 1 reachable 9s from 9 (arrived)
                    if (d == 9) {
                        trails.get_mut(pos).?.* = 1;
                        continue;
                    }

                    var my_total: usize = 0;

                    var neighbors = self.grid.neighbors(pos);
                    while (neighbors.next()) |neighbor| {
                        if (neighbor.item == val + 1) {
                            my_total += trails.get(neighbor.pos).?.?;
                        }
                    }

                    trails.get_mut(pos).?.* = my_total;

                    if (d == 0) {
                        total += my_total;
                    }
                }
            }

            // magic of printing
            std.debug.print("DIGIT {d}\n", .{d});
            for (0..lines) |ii| {
                for (0..self.grid.cols) |jj| {
                    if (trails.get(Position{ .j = jj, .i = ii }).?) |trail_val| {
                        std.debug.print("{d: >2} ", .{trail_val});
                    } else {
                        std.debug.print("__ ", .{});
                    }
                }
                std.debug.print("\n", .{});
            }
            std.debug.print("\n\n", .{});
        }

        return total;
    }
};

fn pos_with_offset(i: usize, delta_i: i2) ?usize {
    if (delta_i == 0) {
        return i;
    } else if (delta_i < 0) {
        return std.math.sub(usize, i, @as(usize, @intCast(@as(i8, -1) * delta_i))) catch null;
    } else {
        return std.math.add(usize, i, @intCast(delta_i)) catch null;
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
        .b => {
            const total = try grid.all_trails_rating();
            std.log.info("rating {d}", .{total});
        },
    }
}
