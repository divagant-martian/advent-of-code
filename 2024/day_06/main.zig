const std = @import("std");

const Position = packed struct { i: usize, j: usize };

const Map = struct {
    obstacles: std.AutoHashMap(Position, void),
    cols: usize,
    lines: usize,

    const newErr = error{ invalidChar, notRectangular, missingGuard };

    fn new(data: []const u8, allocator: std.mem.Allocator) !struct { Map, Guard } {
        var lines = std.mem.tokenizeScalar(u8, data, '\n');

        var obstacles = std.AutoHashMap(Position, void).init(allocator);

        var guard: ?Guard = null;
        var line_number: usize = 0;
        var cols: usize = 0;
        while (lines.next()) |line| : (line_number += 1) {
            for (line, 0..) |char, col_number| switch (char) {
                '.' => continue,
                '#' => try obstacles.put(.{ .i = line_number, .j = col_number }, {}),
                '^' => guard = Guard{ .position = .{ .i = line_number, .j = col_number }, .direction = .up, .is_outside = null },
                else => return Map.newErr.invalidChar,
            };
            if (cols == 0) {
                cols = line.len;
            } else if (cols != line.len) {
                return Map.newErr.notRectangular;
            }
        }
        return .{ Map{ .obstacles = obstacles, .lines = line_number, .cols = cols }, guard orelse return Map.newErr.missingGuard };
    }

    fn is_obstacle(self: *const Map, pos: Position) bool {
        return self.obstacles.contains(pos);
    }
};

const Guard = struct {
    position: Position,
    direction: Guard.Direction,
    is_outside: ?Guard.Direction,

    const Direction = enum {
        up,
        down,
        left,
        right,

        /// Rotates the direction 90° to the right.
        fn rotate(self: *Direction) void {
            self.* = switch (self.*) {
                .up => .right,
                .down => .left,
                .left => .up,
                .right => .down,
            };
        }
    };

    /// Moves the guard one step in the map.
    ///
    /// Returns whether the guard moved.
    fn patrol_step(self: *Guard, map: *const Map) bool {
        var next_pos = self.position;
        switch (self.direction) {
            .up => next_pos.i = std.math.sub(usize, self.position.i, 1) catch {
                self.is_outside = .up;
                return true;
            },
            .down => {
                const new_i = self.position.i + 1;
                if (new_i >= map.lines) {
                    self.is_outside = .down;
                    return true;
                }
                next_pos.i = new_i;
            },
            .left => next_pos.j = std.math.sub(usize, self.position.j, 1) catch {
                self.is_outside = .left;
                return true;
            },
            .right => {
                const new_j = self.position.j + 1;
                if (new_j >= map.cols) {
                    self.is_outside = .right;
                    return true;
                }
                next_pos.j = new_j;
            },
        }

        if (map.is_obstacle(next_pos)) {
            self.direction.rotate();
            return false;
        }

        self.position = next_pos;

        return true;
    }
};

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

    const map: Map, var guard: Guard = try Map.new(data, allocator);

    if (second_part) {
        // std.log.info("part 2: {d}", .{total});
    } else {
        var visited = std.AutoArrayHashMap(Position, void).init(allocator);
        try visited.put(guard.position, {});
        while (guard.is_outside == null) {
            if (guard.patrol_step(&map)) {
                try visited.put(guard.position, {});
            }
        }
        std.log.info("part 1: {d}", .{visited.keys().len});
    }
}
