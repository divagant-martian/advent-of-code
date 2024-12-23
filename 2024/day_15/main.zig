const std = @import("std");
const libgrid = @import("grid");
const Args = @import("args").Args;

pub const std_options = .{
    .log_level = .debug,
};

const Str: type = []const u8;
const Int: type = i32;

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

    /// Moves the robot returning whether the rebot moved.
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

    pub fn format(self: Map, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
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
    defer map.grid.deinit();

    const instructions = try parse_instructions(sections.next().?, allocator);
    map.move_robot_in_bulk(instructions.items, true);
    const sum_gps = map.sum_gps();
    std.log.info("sum gps {d}", .{sum_gps});
}
