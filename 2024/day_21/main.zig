const std = @import("std");
const libgrid = @import("grid");
const Sorted = @import("sorted_array").Sorted;
const Args = @import("args").Args;

pub const std_options = .{
    .log_level = .debug,
};

const NumKey = enum {
    n0,
    n1,
    n2,
    n3,
    n4,
    n5,
    n6,
    n7,
    n8,
    n9,
    a,

    fn into_pos(self: NumKey) libgrid.Position {
        return switch (self) {
            .n0 => .{ .i = 3, .j = 1 },
            .n1 => .{ .i = 2, .j = 0 },
            .n2 => .{ .i = 2, .j = 1 },
            .n3 => .{ .i = 2, .j = 2 },
            .n4 => .{ .i = 1, .j = 0 },
            .n5 => .{ .i = 1, .j = 1 },
            .n6 => .{ .i = 1, .j = 2 },
            .n7 => .{ .i = 0, .j = 0 },
            .n8 => .{ .i = 0, .j = 1 },
            .n9 => .{ .i = 0, .j = 2 },
            .a => .{ .i = 3, .j = 2 },
        };
    }

    pub fn format(self: @This(), comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;

        const char: u8 = switch (self) {
            .n0 => '0',
            .n1 => '1',
            .n2 => '2',
            .n3 => '3',
            .n4 => '4',
            .n5 => '5',
            .n6 => '6',
            .n7 => '7',
            .n8 => '8',
            .n9 => '9',
            .a => 'A',
        };

        return std.fmt.formatAsciiChar(char, options, writer);
    }
};

const DirKey = enum {
    up,
    down,
    left,
    right,
    enter,

    fn into_pos(self: DirKey) libgrid.Position {
        return switch (self) {
            .up => .{ .i = 0, .j = 1 },
            .down => .{ .i = 1, .j = 1 },
            .left => .{ .i = 1, .j = 0 },
            .right => .{ .i = 1, .j = 2 },
            .enter => .{ .i = 0, .j = 2 },
        };
    }

    fn append_delta(delta: libgrid.Delta, arr: *std.ArrayList(DirKey)) !void {
        // reserve space for all the keystrokes
        try arr.ensureUnusedCapacity(delta.norm() + 1);
        // pick the vertical key based on the delta's sign
        const maybe_i_key: ?DirKey = switch (std.math.sign(delta.i)) {
            1 => .down,
            0 => null,
            -1 => .up,
            else => unreachable,
        };
        // append vertical keystrokes
        if (maybe_i_key) |i_key| {
            for (0..@abs(delta.i)) |_| {
                // std.debug.print(" appending {any}\n", .{i_key});
                arr.appendAssumeCapacity(i_key);
            }
        }
        // pick the horizontal key based on the delta's sign
        const maybe_j_key: ?DirKey = switch (std.math.sign(delta.j)) {
            1 => .right,
            0 => null,
            -1 => .left,
            else => unreachable,
        };
        // append horizontal keystrokes
        if (maybe_j_key) |j_key| {
            for (0..@abs(delta.j)) |_| {
                // std.debug.print(" appending {any}\n", .{j_key});
                arr.appendAssumeCapacity(j_key);
            }
        }

        // std.debug.print(" appending {any}\n", .{.enter});
        arr.appendAssumeCapacity(.enter);
    }

    pub fn format(self: @This(), comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;

        const char: u8 = switch (self) {
            .up => '^',
            .down => 'v',
            .left => '<',
            .right => '>',
            .enter => 'E',
        };

        return std.fmt.formatAsciiChar(char, options, writer);
    }
};

fn parse_numkeys(data: []const u8, allocator: std.mem.Allocator) !std.ArrayList(NumKey) {
    var numstring = try std.ArrayList(NumKey).initCapacity(allocator, data.len);
    for (data) |char| {
        const numkey: NumKey = switch (char) {
            '0' => .n0,
            '1' => .n1,
            '2' => .n2,
            '3' => .n3,
            '4' => .n4,
            '5' => .n5,
            '6' => .n6,
            '7' => .n7,
            '8' => .n8,
            '9' => .n9,
            'A' => .a,
            else => return error.invalidNumKey,
        };
        numstring.appendAssumeCapacity(numkey);
    }

    return numstring;
}

fn numstring_to_dirstring(numstring: []const NumKey, allocator: std.mem.Allocator) !std.ArrayList(DirKey) {
    var dirstring = std.ArrayList(DirKey).init(allocator);

    var iter = std.mem.window(NumKey, numstring, 2, 1);
    while (iter.next()) |window| {
        const delta = window[1].into_pos().sub(&window[0].into_pos());
        try DirKey.append_delta(delta, &dirstring);
    }

    return dirstring;
}

fn dirstring_to_dirstring(dirstring: []const DirKey, allocator: std.mem.Allocator) !std.ArrayList(DirKey) {
    var dirdirstring = std.ArrayList(DirKey).init(allocator);

    // std.debug.print("fuckyourmom\n", .{});
    var iter = std.mem.window(DirKey, dirstring, 2, 1);
    while (iter.next()) |window| {
        const delta = window[1].into_pos().sub(&window[0].into_pos());
        // std.debug.print("{any}({ij}) -> {any}({ij}) delta: {any}\n", .{ window[0], window[0].into_pos(), window[1], window[1].into_pos(), delta });
        try DirKey.append_delta(delta, &dirdirstring);
    }

    return dirdirstring;
}

fn print_numstring(numstring: []const NumKey) void {
    for (numstring) |numkey| {
        std.debug.print("{c}", .{numkey});
    }
    std.debug.print("\n", .{});
}

fn print_dirstring(dirstring: []const DirKey) void {
    for (dirstring) |dirkey| {
        std.debug.print("{c}", .{dirkey});
    }
    std.debug.print("\n", .{});
}

test "029A numkey to dirkey" {
    const numstring = [_]NumKey{ .a, .n0, .n2, .n9, .a };
    const expected_dirstring = [_]DirKey{ .left, .enter, .up, .enter, .up, .up, .right, .enter, .down, .down, .down, .enter };

    const dirstring = try numstring_to_dirstring(&numstring, std.testing.allocator);
    defer dirstring.deinit();

    // std.debug.print("{any}\n", .{dirstrokes.items});
    try std.testing.expectEqualSlices(DirKey, &expected_dirstring, dirstring.items);
}

test "029A dirkey to dirkey" {
    const dirstring = [_]DirKey{ .enter, .left, .enter, .up, .enter, .up, .up, .right, .enter, .down, .down, .down, .enter };
    const expected_dirdirstring = [_]DirKey{ .down, .left, .left, .enter, .up, .right, .right, .enter, .left, .enter, .right, .enter, .left, .enter, .enter, .down, .right, .enter, .up, .enter, .down, .left, .enter, .enter, .enter, .up, .right, .enter };

    const dirdirstring = try dirstring_to_dirstring(&dirstring, std.testing.allocator);
    defer dirdirstring.deinit();

    std.debug.print("yourmom\n", .{});
    std.debug.print("expected: {any}\n", .{expected_dirdirstring});
    std.debug.print("found:    {any}\n", .{dirdirstring.items});
    try std.testing.expectEqualSlices(DirKey, &expected_dirdirstring, dirdirstring.items);
}

test "<vA dirkey to dirkey pt 2" {
    const dirstring = [_]DirKey{ .enter, .left, .down, .enter };
    var dirdirstring = try dirstring_to_dirstring(&dirstring, std.testing.allocator);
    defer dirdirstring.deinit();
    try dirdirstring.insert(0, .enter);
    const dirdirdirstring = try dirstring_to_dirstring(dirdirstring.items, std.testing.allocator);
    defer dirdirdirstring.deinit();

    std.debug.print("yourgreatmom\n", .{});
    std.debug.print("nonce:   {any}\n", .{dirstring});
    std.debug.print("once:    {any}\n", .{dirdirstring.items});
    std.debug.print("twice:   {any}\n", .{dirdirdirstring.items});
    @panic("fuck...");
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

    var lines = std.mem.tokenizeScalar(u8, data, '\n');

    while (lines.next()) |line| {
        var numstring = try parse_numkeys(line, allocator);
        try numstring.insert(0, .a);

        std.debug.print("numstring:  ", .{});
        print_numstring(numstring.items);

        var dirstring0 = try numstring_to_dirstring(numstring.items, allocator);
        try dirstring0.insert(0, .enter);

        std.debug.print("dirstring0: ", .{});
        print_dirstring(dirstring0.items);

        var dirstring1 = try dirstring_to_dirstring(dirstring0.items, allocator);
        try dirstring1.insert(0, .enter);

        std.debug.print("dirstring1: ", .{});
        print_dirstring(dirstring1.items);

        const dirstring2 = try dirstring_to_dirstring(dirstring1.items, allocator);

        std.debug.print("dirstring2: ", .{});
        print_dirstring(dirstring2.items);

        std.debug.print("final len: {d}\n\n", .{dirstring2.items.len});
    }
}
