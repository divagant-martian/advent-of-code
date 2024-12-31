const std = @import("std");
const libgrid = @import("grid");
const Args = @import("args").Args;

pub const std_options = .{
    .log_level = .debug,
};

const IsCorrupted = enum { yes, no };

const Space = struct {
    grid: libgrid.Grid(IsCorrupted),
    falls: std.ArrayList(libgrid.Position),
    idx: usize,

    fn new(data: []const u8, mem_width: usize, allocator: std.mem.Allocator) !Space {
        const falls = try parse_falls(data, allocator);
        const grid = try libgrid.Grid(IsCorrupted).repeat(IsCorrupted.no, mem_width, mem_width, allocator);
        return Space{ .grid = grid, .falls = falls, .idx = 0 };
    }

    fn advance_steps(self: *Space, count: usize) void {
        const new_idx = self.idx + count;
        const apply = self.falls.items[self.idx..new_idx];
        for (apply) |fall_pos| {
            self.grid.get_mut(fall_pos).?.* = IsCorrupted.yes;
        }
        self.idx = new_idx;
    }

    fn find_path_len(self: *const Space) error{ outOfMem, unreachableGoal }!usize {
        const allocator = self.grid.get_allocator();

        const start: libgrid.Position = .{ .i = 0, .j = 0 };

        var queue = std.ArrayList(libgrid.Position).init(allocator);
        queue.append(start) catch return error.outOfMem;
        defer queue.deinit();

        var distances = std.AutoHashMap(libgrid.Position, usize).init(allocator);
        distances.put(start, 0) catch return error.outOfMem;
        defer distances.deinit();

        const mem_width = self.grid.cols;
        const end: libgrid.Position = .{ .i = mem_width - 1, .j = mem_width - 1 };
        while (queue.items.len > 0) {
            const curr = queue.orderedRemove(0);

            const curr_dist = distances.get(curr).?;
            if (curr.eq(&end)) {
                return curr_dist;
            }

            var neighbors = self.grid.neighbors(curr);
            while (neighbors.next()) |neighbor| {
                if (neighbor.item == IsCorrupted.yes) {
                    continue;
                }

                if (!distances.contains(neighbor.pos)) {
                    const alt_dist = curr_dist + 1;
                    distances.put(neighbor.pos, alt_dist) catch return error.outOfMem;
                    queue.append(neighbor.pos) catch return error.outOfMem;
                }
            }
        }

        return error.unreachableGoal;
    }

    fn find_blocking_fall(self: *Space) !libgrid.Position {
        var advance: usize = 0;
        while (advance < self.falls.items.len) : (advance += 1) {
            self.advance_steps(1);
            _ = self.find_path_len() catch |err| switch (err) {
                error.outOfMem => return err,
                error.unreachableGoal => return self.falls.items[self.idx - 1],
            };
        }

        unreachable;
    }
};

fn parse_falls(data: []const u8, allocator: std.mem.Allocator) error{ outOfMem, invalidNumber, invalidLine }!std.ArrayList(libgrid.Position) {
    var lines = std.mem.tokenizeScalar(u8, data, '\n');
    var positions = std.ArrayList(libgrid.Position).init(allocator);
    while (lines.next()) |line| {
        var parts = std.mem.tokenizeScalar(u8, line, ',');
        const x_str = parts.next() orelse return error.invalidLine;
        const y_str = parts.next() orelse return error.invalidLine;
        const x = std.fmt.parseInt(usize, x_str, 10) catch return error.invalidNumber;
        const y = std.fmt.parseInt(usize, y_str, 10) catch return error.invalidNumber;

        // `x` is left to right, which in position is `j`.
        // `y` is up to down, which is conversely `i`
        const pos = libgrid.Position{ .i = y, .j = x };
        positions.append(pos) catch return error.outOfMem;
    }

    return positions;
}

const _ = error{ missingMemWidth, invalidMemWidth, missingAdvanceSteps, invalidAdvanceSteps };

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var args_iter = try std.process.argsWithAllocator(allocator);
    defer args_iter.deinit();

    const args = try Args.from_args_iter(&args_iter);

    const mem_width_str = args_iter.next() orelse return error.missingMemWidth;
    const mem_width = std.fmt.parseInt(usize, mem_width_str, 10) catch return error.invalidMemWidth;

    var file = try std.fs.cwd().openFile(args.file_name, .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    const data = try buf_reader.reader().readAllAlloc(allocator, std.math.maxInt(usize));

    var space = try Space.new(data, mem_width, allocator);

    const advance_steps_str = args_iter.next() orelse return error.missingAdvanceSteps;
    const advance_steps = std.fmt.parseInt(usize, advance_steps_str, 10) catch return error.invalidAdvanceSteps;

    space.advance_steps(advance_steps);

    switch (args.part) {
        .a => {
            const path_len = try space.find_path_len();
            std.log.info("Path len {d}", .{path_len});
        },
        .b => {
            const blocking_pos = try space.find_blocking_fall();
            std.log.info("Blocking pos {xy}", .{blocking_pos});
        },
    }
}
