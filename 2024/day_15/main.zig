const std = @import("std");
const libgrid = @import("grid");
const Args = @import("args").Args;

pub const std_options = .{
    .log_level = .debug,
};

const Str: type = []const u8;
const Int: type = i32;

const VerticalDir = enum { up, down };

const Tile = enum {
    wall,
    box,
    robot,
    empty,

    pub const ParseErr = error{invalidChar};

    pub fn parse(c: u8) !Tile {
        const tile: Tile = switch (c) {
            '#' => .wall,
            'O' => .box,
            '@' => .robot,
            '.' => .empty,
            else => return Tile.ParseErr.invalidChar,
        };

        return tile;
    }
};

const Map = struct {
    grid: libgrid.Grid(Tile),
    robot_pos: libgrid.Position,

    const ParseErr = error{ missingRobot, duplicatedRobot };

    fn parse(data: []const u8, allocator: std.mem.Allocator) !Map {
        const grid = try libgrid.Grid(Tile).new(data, Tile.parse, allocator);
        var robot: ?libgrid.Position = null;
        var iter = grid.iterator();
        while (iter.next()) |entry| {
            if (entry.value.* == .robot) {
                if (robot == null) {
                    robot = entry.position;
                } else {
                    return Map.ParseErr.duplicatedRobot;
                }
            }
        }

        if (robot) |robot_pos| {
            return Map{ .grid = grid, .robot_pos = robot_pos };
        } else {
            return Map.ParseErr.missingRobot;
        }
    }

    /// Moves the robot returning whether the robot moved.
    fn move_robot(self: *Map, direction: libgrid.Direction) bool {
        var maybe_candidate_pos = self.robot_pos.move(direction);

        while (maybe_candidate_pos) |candidate_pos| {
            const next_tile = self.grid.get_mut(candidate_pos) orelse return false;
            switch (next_tile.*) {
                .wall => return false,
                .empty => {
                    // robot simply Moves
                    next_tile.* = .box;
                    self.grid.get_mut(self.robot_pos).?.* = .empty;
                    // to deal with the case of multiple moved boxes, we
                    // recalculate where the robot ends, since candidate_pos
                    // might not be next to robot_pos anymore.
                    self.robot_pos = self.robot_pos.move(direction).?;
                    self.grid.get_mut(self.robot_pos).?.* = .robot;
                    return true;
                },
                .box => {
                    // maybe we can move this, try the next position
                    maybe_candidate_pos = candidate_pos.move(direction);
                },
                .robot => @panic("robot found robot while moving"),
            }
        }

        return false;
    }

    fn move_robot_in_bulk(self: *Map, directions: []const libgrid.Direction, enable_debug: bool) void {
        if (enable_debug) {
            std.log.debug("grid:\n{d}", .{self});
        }

        for (directions) |dir| {
            _ = self.move_robot(dir);
            if (enable_debug) {
                std.log.debug("moved to {any}\n{d}", .{ dir, self });
            }
        }
    }

    fn sum_gps(self: *const Map) usize {
        var iter = self.grid.iterator();
        var sum: usize = 0;
        while (iter.next()) |entry| {
            if (entry.value.* == .box) {
                sum += 100 * entry.position.i + entry.position.j;
            }
        }

        return sum;
    }

    pub fn format(self: *const Map, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        const formatter = self.grid.formatter(grid_fmt);
        try formatter.format(fmt, options, writer);
    }
};

const grid_fmt = struct {
    fn foo(
        value: Tile,
        comptime fmt: []const u8,
        options: std.fmt.FormatOptions,
        writer: anytype,
    ) !void {
        _ = fmt;
        const char: u8 = switch (value) {
            .empty => '.',
            .robot => '@',
            .wall => '#',
            .box => 'O',
        };
        try std.fmt.formatAsciiChar(char, options, writer);
    }
}.foo;

const WideTile = enum {
    wall,
    leftBox,
    rightBox,
    empty,
    robot,
};

const WideMap = struct {
    grid: libgrid.Grid(WideTile),
    robot_pos: libgrid.Position,

    fn from_map(map: Map) !WideMap {
        defer map.grid.deinit();

        var new_grid = try std.ArrayList(WideTile).initCapacity(map.grid.get_allocator(), map.grid.items().len * 2);
        for (map.grid.items()) |tile| {
            const wide_tiles: struct { WideTile, WideTile } = switch (tile) {
                .wall => .{ .wall, .wall },
                .empty => .{ .empty, .empty },
                .robot => .{ .robot, .empty },
                .box => .{ .leftBox, .rightBox },
            };

            const left_tile, const right_tile = wide_tiles;
            new_grid.appendAssumeCapacity(left_tile);
            new_grid.appendAssumeCapacity(right_tile);
        }

        const robot_pos = libgrid.Position{ .i = map.robot_pos.i, .j = map.robot_pos.j * 2 };
        const wide_grid = libgrid.Grid(WideTile){ .grid = new_grid, .cols = map.grid.cols * 2 };
        return WideMap{ .grid = wide_grid, .robot_pos = robot_pos };
    }

    fn move_robot_hotizontally(self: *WideMap, dir: enum { left, right }) bool {
        const direction: libgrid.Direction = switch (dir) {
            .left => .left,
            .right => .right,
        };
        var maybe_candidate_pos = self.robot_pos.move(direction);

        while (maybe_candidate_pos) |candidate_pos| {
            const next_tile = self.grid.get_mut(candidate_pos) orelse return false;
            switch (next_tile.*) {
                .wall => return false,
                .empty => {
                    // "push" right the boxes from the right to the left so that the
                    // position where the robot should go is "empty"
                    var alt_j = candidate_pos.j;
                    while (alt_j != self.robot_pos.j) {
                        const curr_pos = libgrid.Position{ .i = candidate_pos.i, .j = alt_j };
                        switch (dir) {
                            .right => alt_j -= 1,
                            .left => alt_j += 1,
                        }
                        const next_pos = libgrid.Position{ .i = candidate_pos.i, .j = alt_j };
                        self.grid.get_mut(curr_pos).?.* = self.grid.get(next_pos).?;
                    }

                    // now empty where the robot used to be and update its position
                    self.grid.get_mut(self.robot_pos).?.* = .empty;
                    self.robot_pos = self.robot_pos.move(direction).?;
                    return true;
                },
                .leftBox => {
                    // maybe we can move this, try the next position
                    maybe_candidate_pos = candidate_pos.move(direction);
                },

                .rightBox => {
                    // maybe we can move this, try the next position
                    maybe_candidate_pos = candidate_pos.move(direction);
                },
                .robot => @panic("robot found robot while moving"),
            }
        }

        return false;
    }

    /// Checks if `pos` can be moved in the given direction.
    ///
    /// Will return the list of nodes to be moved, starting from the last one.
    fn can_move(self: *const WideMap, dir: VerticalDir) !?std.ArrayList(libgrid.Position) {
        const allocator = self.grid.get_allocator();
        const direction: libgrid.Direction = switch (dir) {
            .up => .up,
            .down => .down,
        };
        var to_check = std.AutoArrayHashMap(libgrid.Position, void).init(allocator);
        defer to_check.deinit();
        var to_move = std.ArrayList(libgrid.Position).init(allocator);
        try to_check.put(self.robot_pos, {});
        while (true) {
            const pos = to_check.keys()[0];
            to_check.orderedRemoveAt(0);
            // there is always wall ahead and we should never try to move a wall.
            const next_pos = pos.move(direction).?;
            const next_tile = self.grid.get(next_pos).?;
            switch (next_tile) {
                .leftBox => {
                    if (!to_check.contains(next_pos)) {
                        try to_check.put(next_pos, {});
                    }

                    const right_box = next_pos.move(.right).?;
                    if (!to_check.contains(right_box)) {
                        try to_check.put(right_box, {});
                    }
                },
                .rightBox => {
                    const left_box = next_pos.move(.left).?;
                    if (!to_check.contains(left_box)) {
                        try to_check.put(left_box, {});
                    }

                    if (!to_check.contains(next_pos)) {
                        try to_check.put(next_pos, {});
                    }
                },
                .wall => {
                    to_move.deinit();
                    return null;
                },
                .empty => {},
                .robot => @panic("robot found robot while moving"),
            }
            try to_move.append(pos);
            // std.log.debug("appending {}, {} to to_move", .{ pos.i, pos.j });
            if (to_check.count() == 0) {
                break;
            }
        }

        return to_move;
    }

    fn move_robot_vertically(self: *WideMap, dir: VerticalDir) !bool {
        var maybe_can_move = try self.can_move(dir);
        if (maybe_can_move) |*to_move| {
            defer to_move.deinit();
            const direction: libgrid.Direction = switch (dir) {
                .up => .up,
                .down => .down,
            };
            while (to_move.popOrNull()) |move_pos| {
                const curr = self.grid.get_mut(move_pos).?;
                self.grid.get_mut(move_pos.move(direction).?).?.* = curr.*;
                curr.* = .empty;
            }
            self.robot_pos = self.robot_pos.move(direction).?;
            return true;
        }

        return false;
    }

    /// Moves the robot returning whether the robot moved.
    fn move_robot(self: *WideMap, direction: libgrid.Direction) !bool {
        return switch (direction) {
            .up => try self.move_robot_vertically(.up),
            .down => try self.move_robot_vertically(.down),
            .left => self.move_robot_hotizontally(.left),
            .right => self.move_robot_hotizontally(.right),
        };
    }

    fn move_robot_in_bulk(self: *WideMap, directions: []const libgrid.Direction, enable_debug: bool) !void {
        if (enable_debug) {
            std.log.debug("wide grid:\n{d}", .{self});
        }

        for (directions) |dir| {
            _ = try self.move_robot(dir);
            if (enable_debug) {
                std.log.debug("moved to {any}\n{d}", .{ dir, self });
            }
        }
    }

    fn sum_gps(self: *const WideMap) usize {
        var iter = self.grid.iterator();
        var sum: usize = 0;
        while (iter.next()) |entry| {
            if (entry.value.* == .leftBox) {
                sum += 100 * entry.position.i + entry.position.j;
            }
        }

        return sum;
    }

    pub fn format(self: *const WideMap, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        const formatter = self.grid.formatter(wide_grid_fmt);
        try formatter.format(fmt, options, writer);
    }
};

const wide_grid_fmt = struct {
    fn foo(
        value: WideTile,
        comptime fmt: []const u8,
        options: std.fmt.FormatOptions,
        writer: anytype,
    ) !void {
        _ = fmt;
        const char: u8 = switch (value) {
            .empty => ' ',
            .robot => '@',
            .wall => '#',
            .leftBox => '[',
            .rightBox => ']',
        };
        try std.fmt.formatAsciiChar(char, options, writer);
    }
}.foo;

pub fn parse_instructions(data: []const u8, allocator: std.mem.Allocator) !std.ArrayList(libgrid.Direction) {
    var instructions = try std.ArrayList(libgrid.Direction).initCapacity(allocator, data.len);

    for (data) |char| {
        const direction: libgrid.Direction = switch (char) {
            '^' => .up,
            'v' => .down,
            '<' => .left,
            '>' => .right,
            else => continue,
        };

        instructions.appendAssumeCapacity(direction);
    }

    return instructions;
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

    var sections = std.mem.tokenizeSequence(u8, data, "\n\n");

    var map = try Map.parse(sections.next().?, allocator);
    var instructions_str: []const u8 = undefined;

    if (args_iter.next()) |next_arg| {
        instructions_str = next_arg;
    } else {
        instructions_str = sections.next().?;
    }
    const instructions = try parse_instructions(instructions_str, allocator);

    switch (args.part) {
        .a => {
            defer map.grid.deinit();

            map.move_robot_in_bulk(instructions.items, true);
            const sum_gps = map.sum_gps();
            std.log.info("sum gps {d}", .{sum_gps});
        },
        .b => {
            var wide_map = try WideMap.from_map(map);
            try wide_map.move_robot_in_bulk(instructions.items, true);
            const sum_gps = wide_map.sum_gps();
            std.log.info("sum gps {d}", .{sum_gps});
        },
    }
}
