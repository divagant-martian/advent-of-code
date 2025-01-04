const std = @import("std");
const libgrid = @import("grid");

pub const NumKey = enum {
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

    pub fn from_pos(pos: libgrid.Position) ?NumKey {
        const self: NumKey = switch (pos.i) {
            0 => switch (pos.j) {
                0 => .n7,
                1 => .n8,
                2 => .n9,
                else => return null,
            },
            1 => switch (pos.j) {
                0 => .n4,
                1 => .n5,
                2 => .n6,
                else => return null,
            },
            2 => switch (pos.j) {
                0 => .n1,
                1 => .n2,
                2 => .n3,
                else => return null,
            },
            3 => switch (pos.j) {
                1 => .n0,
                2 => .a,
                else => return null,
            },
            else => return null,
        };
        return self;
    }

    pub fn into_pos(self: NumKey) libgrid.Position {
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

    pub fn move_once(self: NumKey, dir: libgrid.Direction) ?NumKey {
        const curr_pos = self.into_pos().move(dir) orelse return null;
        return NumKey.from_pos(curr_pos);
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

pub const NumStr = struct {
    num_keys: []const NumKey,
    allocator: std.mem.Allocator,

    pub fn from_arraylist(list: *const std.ArrayList(NumKey)) NumStr {
        return NumStr{ .num_keys = list.items, .allocator = list.allocator };
    }

    pub fn parse(data: []const u8, allocator: std.mem.Allocator) !NumStr {
        var numstring = try std.ArrayList(NumKey).initCapacity(allocator, data.len + 1);
        numstring.appendAssumeCapacity(.a);

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

        return NumStr{ .num_keys = numstring.items, .allocator = allocator };
    }

    pub fn deinit(self: NumStr) void {
        self.allocator.free(self.num_keys);
    }

    pub fn format(self: *const @This(), comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        for (self.num_keys) |numkey| {
            try numkey.format(fmt, options, writer);
        }
    }
};
