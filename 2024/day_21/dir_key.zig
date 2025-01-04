const std = @import("std");
const libgrid = @import("grid");
const numkey = @import("num_key");

pub const DirKey = enum(u3) {
    up,
    down,
    left,
    right,
    enter,

    pub fn ge(self: DirKey, other: DirKey) bool {
        return @intFromEnum(self) >= @intFromEnum(other);
    }

    pub fn into_pos(self: DirKey) libgrid.Position {
        return switch (self) {
            .up => .{ .i = 0, .j = 1 },
            .down => .{ .i = 1, .j = 1 },
            .left => .{ .i = 1, .j = 0 },
            .right => .{ .i = 1, .j = 2 },
            .enter => .{ .i = 0, .j = 2 },
        };
    }

    pub fn into_move(self: DirKey) ?libgrid.Direction {
        const dir: libgrid.Direction = switch (self) {
            .up => .up,
            .left => .left,
            .right => .right,
            .down => .down,
            .enter => return null,
        };
        return dir;
    }

    pub fn move_once(self: DirKey, dir: libgrid.Direction) ?DirKey {
        const curr_pos = self.into_pos().move(dir) orelse return null;
        return DirKey.from_pos(curr_pos);
    }

    pub fn from_pos(pos: libgrid.Position) ?DirKey {
        const self: DirKey = switch (pos.i) {
            0 => switch (pos.j) {
                1 => .up,
                2 => .enter,
                else => return null,
            },
            1 => switch (pos.j) {
                0 => .left,
                1 => .down,
                2 => .right,
                else => return null,
            },
            else => return null,
        };
        return self;
    }

    fn append_delta(delta: libgrid.Delta, add_enter: bool, arr: *std.ArrayList(DirKey)) !void {
        // reserve space for all the keystrokes
        if (add_enter) {
            try arr.ensureUnusedCapacity(delta.norm() + 1);
        } else {
            try arr.ensureUnusedCapacity(delta.norm());
        }

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
                arr.appendAssumeCapacity(j_key);
            }
        }

        if (add_enter) {
            arr.appendAssumeCapacity(.enter);
        }
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

pub const DirStr = struct {
    dir_keys: []DirKey,
    allocator: std.mem.Allocator,

    pub fn from_num_keys(from: numkey.NumKey, to: numkey.NumKey, allocator: std.mem.Allocator) !DirStr {
        return from_key(numkey.NumKey, from, to, allocator);
    }

    fn from_key(comptime T: type, from: T, to: T, allocator: std.mem.Allocator) !DirStr {
        var dirstring = std.ArrayList(DirKey).init(allocator);
        const delta = to.into_pos().sub(&from.into_pos());
        try DirKey.append_delta(delta, false, &dirstring);
        return DirStr{ .dir_keys = dirstring.items, .allocator = dirstring.allocator };
    }

    pub fn parse(data: []const u8, allocator: std.mem.Allocator) !DirStr {
        var dirstr = try std.ArrayList(DirKey).initCapacity(allocator, data.len);
        for (data) |char| {
            const dirkey: DirKey = switch (char) {
                'A' => .enter,
                '<' => .left,
                '^' => .up,
                '>' => .right,
                'v' => .down,
                else => return error.invalidDirKey,
            };
            dirstr.appendAssumeCapacity(dirkey);
        }

        return DirStr{ .dir_keys = dirstr.items, .allocator = allocator };
    }

    pub fn deinit(self: DirStr) void {
        self.allocator.free(self.dir_keys);
    }

    pub fn format(self: *const @This(), comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        for (self.dir_keys) |dir_key| {
            try dir_key.format(fmt, options, writer);
        }
    }

    pub fn from_dir_keys(from: DirKey, to: DirKey, allocator: std.mem.Allocator) !DirStr {
        return from_key(DirKey, from, to, allocator);
    }

    pub fn with_enter(self: *const DirStr) !DirStr {
        const new_ptr = try self.allocator.alloc(DirKey, self.dir_keys.len + 1);
        std.mem.copyForwards(DirKey, new_ptr, self.dir_keys);
        new_ptr[self.dir_keys.len] = .enter;
        return DirStr{ .dir_keys = new_ptr, .allocator = self.allocator };
    }
};

// test "029A numkey to dirkey" {
//     const allocator = std.testing.allocator;
//
//     const numstring = try numkey.NumStr.parse("A029A", allocator);
//     defer numstring.deinit();
//
//     const expected_dirstring = try DirStr.parse("<A^A^^>AvvvA", allocator);
//     defer expected_dirstring.deinit();
//
//     const dirstring = try DirStr.from_num_str(&numstring);
//     defer dirstring.deinit();
//
//     try std.testing.expectEqualSlices(DirKey, expected_dirstring.dir_keys, dirstring.dir_keys);
// }

// test "029A dirkey to dirkey" {
//     const dirstring = [_]DirKey{ .enter, .left, .enter, .up, .enter, .up, .up, .right, .enter, .down, .down, .down, .enter };
//     const expected_dirdirstring = [_]DirKey{ .down, .left, .left, .enter, .up, .right, .right, .enter, .left, .enter, .right, .enter, .left, .enter, .enter, .down, .right, .enter, .up, .enter, .down, .left, .enter, .enter, .enter, .up, .right, .enter };
//
//     const dirdirstring = try dirstring_to_dirstring(&dirstring, std.testing.allocator);
//     defer dirdirstring.deinit();
//
//     std.debug.print("yourmom\n", .{});
//     std.debug.print("expected: {any}\n", .{expected_dirdirstring});
//     std.debug.print("found:    {any}\n", .{dirdirstring.items});
//     try std.testing.expectEqualSlices(DirKey, &expected_dirdirstring, dirdirstring.items);
// }
//
// test "<vA dirkey to dirkey pt 2" {
//     const dirstring = [_]DirKey{ .enter, .left, .down, .enter };
//     var dirdirstring = try dirstring_to_dirstring(&dirstring, std.testing.allocator);
//     defer dirdirstring.deinit();
//     try dirdirstring.insert(0, .enter);
//     const dirdirdirstring = try dirstring_to_dirstring(dirdirstring.items, std.testing.allocator);
//     defer dirdirdirstring.deinit();
//
//     std.debug.print("yourgreatmom\n", .{});
//     std.debug.print("nonce:   {any}\n", .{dirstring});
//     std.debug.print("once:    {any}\n", .{dirdirstring.items});
//     std.debug.print("twice:   {any}\n", .{dirdirdirstring.items});
//     @panic("fuck...");
// }
