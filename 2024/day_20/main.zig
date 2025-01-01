const std = @import("std");
const libgrid = @import("grid");
const Sorted = @import("sorted_array").Sorted;
const Args = @import("args").Args;

pub const std_options = .{
    .log_level = .debug,
};

const Tile = enum {
    wall,
    start,
    end,
    empty,

    pub fn parse(c: u8) error{invalidChar}!Tile {
        const tile: Tile = switch (c) {
            '#' => .wall,
            'S' => .start,
            'E' => .end,
            '.' => .empty,
            else => return error.invalidChar,
        };

        return tile;
    }

    pub fn format(self: Tile, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;

        const char: u8 = switch (self) {
            .wall => '#',
            .start => 'S',
            .end => 'E',
            .empty => ' ',
        };

        return std.fmt.formatIntValue(char, "c", options, writer);
    }
};

const Racetrack = struct {
    grid: libgrid.Grid(Tile),
    start: libgrid.Position,
    end: libgrid.Position,

    fn parse(data: []const u8, allocator: std.mem.Allocator) !Racetrack {
        const grid = try libgrid.Grid(Tile).new(data, Tile.parse, allocator);
        var maybe_start: ?libgrid.Position = null;
        var maybe_end: ?libgrid.Position = null;

        var iter = grid.iterator();
        while (iter.next()) |entry| {
            switch (entry.value.*) {
                .wall => continue,
                .empty => continue,
                .start => maybe_start = entry.position,
                .end => maybe_end = entry.position,
            }
        }

        return .{ .grid = grid, .start = maybe_start orelse return error.missingStart, .end = maybe_end orelse return error.missingEnd };
    }

    fn get_path(self: *const Racetrack) !std.AutoArrayHashMap(libgrid.Position, usize) {
        var curr = self.start;
        var idx: usize = 1;

        var path = std.AutoArrayHashMap(libgrid.Position, usize).init(self.grid.get_allocator());
        try path.put(self.start, 0);

        while (!curr.eq(&self.end)) {
            var neighbors = self.grid.neighbors(curr);

            while (neighbors.next()) |neighbor| {
                if ((neighbor.item == .empty) or (neighbor.item == .end)) {
                    const entry = try path.getOrPut(neighbor.pos);
                    if (!entry.found_existing) {
                        entry.value_ptr.* = idx;
                        curr = neighbor.pos;
                        idx += 1;
                        break;
                    }
                }
            }
        }

        return path;
    }

    fn cheater_neighbors(self: *const Racetrack, pos: libgrid.Position, dist: usize) CheaterNeighborIter {
        return .{
            .grid = &self.grid,
            .pos = pos,
            .dist = dist,
        };
    }

    fn count_top_cheats(self: *const Racetrack, min: usize, max: usize, dist: usize) !usize {
        var count: usize = 0;
        const path = try self.get_path();
        const path_len = path.get(self.end).?;

        var iterator = path.iterator();

        while (iterator.next()) |curr| {
            const curr_pos = curr.key_ptr.*;
            const traveled_dist_before_jump = curr.value_ptr.*;

            var neighbors = self.cheater_neighbors(curr_pos, dist);

            while (neighbors.next()) |neighbor| {
                if (neighbor.item == .wall) {
                    continue;
                }

                const traveled_dist_after_jump = path.get(neighbor.pos).?;
                const jump_len = curr_pos.dist(&neighbor.pos);
                const path_after_jump_len = traveled_dist_before_jump + jump_len + (path_len - traveled_dist_after_jump);
                if (path_len >= path_after_jump_len) {
                    const saved_distance = path_len - path_after_jump_len;
                    if (saved_distance >= min and saved_distance <= max) {
                        std.log.debug("saved {d} by jumping from {ij} to {ij}", .{ saved_distance, curr_pos, neighbor.pos });
                        count += 1;
                    }
                }
            }
        }

        return count;
    }

    pub fn format(self: *const Racetrack, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        return self.grid.formatter(Tile.format).format(fmt, options, writer);
    }
};

fn move_twice(pos: libgrid.Position, fst_dir: libgrid.Direction, snd_dir: libgrid.Direction) ?libgrid.Position {
    if (pos.move(fst_dir)) |mid| {
        return mid.move(snd_dir);
    }
    return null;
}

const CheaterNeighborIter =
    struct {
    pos: libgrid.Position,
    grid: *const libgrid.Grid(Tile),
    /// Null means the iterator is fuzed.
    a: ?usize = 0,
    b: usize = 1,
    dist: usize,
    /// Names mean offset in i(lines) and j(columns)
    quadrant: enum(u2) { NegativePositive = 0, PositivePositive = 1, PositiveNegative = 2, NegativeNegative = 3 } = .NegativePositive,

    fn next_abs_offsets(self: *CheaterNeighborIter) ?struct { a: usize, b: usize } {
        // for a in [0, dist]:
        //     for b in [1, dist - a]:
        //         yield (a, b)

        const a = self.a orelse return null;
        const pair = .{ .a = a, .b = self.b };
        if (self.b + 1 + a <= self.dist) {
            // if next b <= dist - a
            self.b += 1;
        } else if (a + 1 <= self.dist) {
            // if next a <= dist
            self.a = a + 1;
            self.b = 1;
        } else {
            self.a = null;
            self.b = 1;
        }

        const norm = pair.a + pair.b;
        if (norm < 2 or norm > self.dist) {
            return self.next_abs_offsets();
        }

        return pair;
    }

    fn reset_offsets(self: *CheaterNeighborIter) void {
        self.a = 0;
        self.b = 1;
    }

    fn next_offsets(self: *CheaterNeighborIter) ?struct { di: isize, dj: isize } {
        var maybe_offsets = self.next_abs_offsets();
        if (maybe_offsets == null and self.quadrant != .NegativeNegative) {
            self.reset_offsets();
            self.quadrant = @enumFromInt(@intFromEnum(self.quadrant) + 1);
            maybe_offsets = self.next_abs_offsets();
        }

        const offsets = maybe_offsets orelse return null;

        switch (self.quadrant) {
            .NegativePositive => {
                const di: isize = @as(isize, @intCast(offsets.b)) * -1;
                const dj: isize = @intCast(offsets.a);
                return .{ .di = di, .dj = dj };
            },
            .PositivePositive => {
                const di: isize = @intCast(offsets.a);
                const dj: isize = @intCast(offsets.b);
                return .{ .di = di, .dj = dj };
            },
            .PositiveNegative => {
                const di: isize = @intCast(offsets.b);
                const dj: isize = @as(isize, @intCast(offsets.a)) * -1;
                return .{ .di = di, .dj = dj };
            },
            .NegativeNegative => {
                const di: isize = @as(isize, @intCast(offsets.a)) * -1;
                const dj: isize = @as(isize, @intCast(offsets.b)) * -1;
                return .{ .di = di, .dj = dj };
            },
        }
    }

    fn next(self: *CheaterNeighborIter) ?libgrid.Grid(Tile).NeighborEntry {
        while (self.next_offsets()) |offsets| {
            if (self.grid.get_with_offset(self.pos, .{ .di = offsets.di, .dj = offsets.dj })) |entry| {
                return entry;
            }
        }

        return null;
    }
};

test "distance 2" {
    const grid = try libgrid.Grid(Tile).repeat(.empty, 5, 5, std.testing.allocator);
    defer grid.deinit();

    const pos = .{ .i = 2, .j = 2 };

    var neighbors = CheaterNeighborIter{ .pos = pos, .grid = &grid, .dist = 2 };

    try expect_pos(neighbors.next().?.pos, libgrid.Position{ .i = 0, .j = 2 });
    try expect_pos(neighbors.next().?.pos, libgrid.Position{ .i = 1, .j = 3 });
    try expect_pos(neighbors.next().?.pos, libgrid.Position{ .i = 2, .j = 4 });
    try expect_pos(neighbors.next().?.pos, libgrid.Position{ .i = 3, .j = 3 });
    try expect_pos(neighbors.next().?.pos, libgrid.Position{ .i = 4, .j = 2 });
    try expect_pos(neighbors.next().?.pos, libgrid.Position{ .i = 3, .j = 1 });
    try expect_pos(neighbors.next().?.pos, libgrid.Position{ .i = 2, .j = 0 });
    try expect_pos(neighbors.next().?.pos, libgrid.Position{ .i = 1, .j = 1 });
}

test "distance 3" {
    const grid = try libgrid.Grid(Tile).repeat(.empty, 7, 7, std.testing.allocator);
    defer grid.deinit();

    const pos = .{ .i = 3, .j = 3 };

    var neighbors = CheaterNeighborIter{ .pos = pos, .grid = &grid, .dist = 3 };

    while (neighbors.next()) |neighbor| {
        std.debug.print("{ij}\n", .{neighbor.pos});
    }
}

fn expect_pos(actual: libgrid.Position, expected: libgrid.Position) !void {
    if (!expected.eq(&actual)) {
        std.debug.print("expected {ij}, found {ij}\n", .{ expected, actual });
        return error.TestExpectedEqual;
    }
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var args_iter = try std.process.argsWithAllocator(allocator);
    defer args_iter.deinit();

    const args = try Args.from_args_iter(&args_iter);

    var file = try std.fs.cwd().openFile(args.file_name, .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    const data = try buf_reader.reader().readAllAlloc(allocator, std.math.maxInt(usize));

    const racetrack = try Racetrack.parse(data, allocator);

    std.log.info("racetrack\n{c}", .{racetrack});

    const dist: usize = switch (args.part) {
        .a => 2,
        .b => 20,
    };

    const count = try racetrack.count_top_cheats(100, std.math.maxInt(usize), dist);
    std.log.info("{d}", .{count});
}
