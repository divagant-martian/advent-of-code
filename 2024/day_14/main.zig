const std = @import("std");
const libgrid = @import("grid");
const Args = @import("args").Args;

const Str: type = []const u8;
const Int: type = i32;

pub const std_options = .{
    .log_level = .info,
};

const State: type = struct {
    px: Int,
    py: Int,
    vx: Int,
    vy: Int,

    const Self = @This();

    const ParseErr = error{
        MissingLine,
        InvalidLine,
    };

    fn parse(input: Str) !Self {
        const Iter = struct {
            iter: std.mem.TokenIterator(u8, .any),
            fn next(self: *@This()) !Int {
                const int_str = self.iter.next() orelse {
                    return Self.ParseErr.InvalidLine;
                };

                return try std.fmt.parseInt(Int, int_str, 10);
            }
        };

        var iter = Iter{
            .iter = std.mem.tokenizeAny(u8, input, "pv=, "),
        };

        return Self{
            .px = try iter.next(),
            .py = try iter.next(),
            .vx = try iter.next(),
            .vy = try iter.next(),
        };
    }

    fn simulate(self: *const Self, steps: Int, width: Int, height: Int) struct { x: Int, y: Int } {
        return .{
            .x = @mod(self.px + self.vx * steps, width),
            .y = @mod(self.py + self.vy * steps, height),
        };
    }

    pub fn format(
        self: *const Self,
        comptime fmt: []const u8,
        options: std.fmt.FormatOptions,
        writer: anytype,
    ) !void {
        // "p = (px, py), v = (vx, vy)"
        try writer.writeAll("p = (");
        try std.fmt.formatIntValue(self.px, fmt, options, writer);
        try writer.writeAll(", ");
        try std.fmt.formatIntValue(self.py, fmt, options, writer);
        try writer.writeAll("), v = (");
        try std.fmt.formatIntValue(self.vx, fmt, options, writer);
        try writer.writeAll(", ");
        try std.fmt.formatIntValue(self.vy, fmt, options, writer);
        try writer.writeAll(")");
    }
};

fn parse_states(data: Str, allocator: std.mem.Allocator) !std.ArrayList(State) {
    var states = std.ArrayList(State).init(allocator);

    var iter = std.mem.tokenizeSequence(u8, data, "\n");
    while (iter.next()) |input| {
        const state = try State.parse(input);
        try states.append(state);
    }

    return states;
}

fn count_quadrants(states: []const State, width: Int, height: Int, seconds: Int, allocator: std.mem.Allocator) !struct { [4]usize, libgrid.Grid(usize) } {
    var quadrants = [4]usize{ 0, 0, 0, 0 };

    const mid_x = @divExact(width - 1, 2);
    const mid_y = @divExact(height - 1, 2);

    const grid_len: usize = @intCast(width * height);
    var grid_inner = try std.ArrayList(usize).initCapacity(allocator, grid_len);
    grid_inner.appendNTimesAssumeCapacity(0, grid_len);

    var grid =
        libgrid.Grid(usize){ .grid = grid_inner, .cols = @intCast(width) };

    for (states) |state| {
        const pos = state.simulate(seconds, width, height);
        grid.get_mut(.{ .i = @intCast(pos.y), .j = @intCast(pos.x) }).?.* += 1;

        std.log.debug("{d} => ({d}, {d})", .{ state, pos.x, pos.y });

        if (pos.x == mid_x or pos.y == mid_y) {
            continue;
        }

        const idx: usize = switch (pos.x < mid_x) {
            true => switch (pos.y < mid_y) {
                true => 0,
                false => 2,
            },
            false => switch (pos.y < mid_y) {
                true => 1,
                false => 3,
            },
        };
        std.log.debug("({d}, {d}) is in quadrant {d}", .{ pos.x, pos.y, idx });

        quadrants[idx] += 1;
    }

    return .{ quadrants, grid };
}

const grid_fmt = struct {
    fn foo(
        value: usize,
        comptime fmt: []const u8,
        options: std.fmt.FormatOptions,
        writer: anytype,
    ) !void {
        _ = fmt;
        if (value == 0) {
            try std.fmt.formatText("  ", "s", options, writer);
        } else {
            try std.fmt.formatText("🮋🮋", "s", options, writer);
        }
    }
}.foo;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var args_iter = try std.process.argsWithAllocator(allocator);
    defer args_iter.deinit();

    const args = try Args.from_args_iter(&args_iter);

    const seconds = try std.fmt.parseInt(Int, args_iter.next() orelse "100", 10);

    var file = try std.fs.cwd().openFile(args.file_name, .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    const data = try buf_reader.reader().readAllAlloc(allocator, std.math.maxInt(usize));

    const states = try parse_states(data, allocator);

    const width: Int, const height: Int = switch (std.mem.startsWith(u8, args.file_name, "example")) {
        true => .{ 11, 7 },
        false => .{ 101, 103 },
    };

    switch (args.part) {
        .a => {
            const quadrants, const grid = try count_quadrants(states.items, width, height, seconds, allocator);
            grid.deinit();
            const safety_factor = quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3];
            std.log.info("quadrants: {d}", .{quadrants});
            std.log.info("safety factor: {d}", .{safety_factor});
        },
        .b => {
            var secs: Int = 7000;
            _, const orig_grid = try count_quadrants(states.items, width, height, 0, allocator);
            while (secs < 10403) : (secs += 1) {
                _, const grid = try count_quadrants(states.items, width, height, secs, allocator);

                const formatter = grid.formatter(grid_fmt);
                std.log.info("grid:\n{d}", .{formatter});
                std.log.info("seconds: {d}", .{secs});
                std.time.sleep(std.time.ns_per_ms * 50);

                if (std.mem.eql(usize, orig_grid.items(), grid.items())) {
                    std.log.info("period is: {d}", .{secs});

                    break;
                }
                grid.deinit();
            }
        },
    }
}
