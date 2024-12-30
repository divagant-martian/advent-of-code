const std = @import("std");
const libgrid = @import("grid");
const Args = @import("args").Args;

pub const std_options = .{
    .log_level = .debug,
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

    const positions = try parse_falls(data, allocator);
    defer positions.deinit();

    switch (args.part) {
        .a => {
            std.log.info("positions {xy}", .{positions.items});
        },
        .b => {},
    }
}
